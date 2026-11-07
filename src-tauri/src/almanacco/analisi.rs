//! Motore di analisi agronomica per l'Almanacco colturale.
//!
//! Tutte le funzioni sono pure e deterministiche: incrociano i dati meteo e del
//! suolo normalizzati con le soglie fisiologiche delle colture (temperatura minima
//! e massima di germinazione a 6 cm, stato di tempera a 3–9 e 9–27 cm, resistenza
//! al gelo, fabbisogno idrico FAO-56, periodo di Smith e benessere animale THI).

use std::collections::{BTreeMap, BTreeSet};

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use serde::Serialize;

use super::SchedaColturale;
use crate::agro::frost_soil::{IdoneitaSemina, ParametriSuolo, StatoUmidita, stato_umidita};
use crate::agro::thi::thi;
use crate::agro::Semaforo;
use crate::model::Ora;
use crate::report::{GiornoAgro, Kpi, ParametriAgro, StatoSuolo};

/// Formattazione numerica con virgola decimale (convenzione italiana).
pub fn fmt_it(v: f64, decimali: usize) -> String {
    if !v.is_finite() {
        return "—".into();
    }
    let s = format!("{v:.decimali$}");
    s.replace('.', ",")
}

/// Abbreviazione italiana del giorno della settimana + numero (es. «dom 1»).
pub fn giorno_breve_it(d: NaiveDate) -> String {
    let nome = match d.weekday().num_days_from_monday() {
        0 => "lun",
        1 => "mar",
        2 => "mer",
        3 => "gio",
        4 => "ven",
        5 => "sab",
        _ => "dom",
    };
    format!("{nome} {}", d.day())
}

/// Abbreviazione italiana di giorno e ora (es. «dom 06:00»).
pub fn ora_breve_it(dt: NaiveDateTime) -> String {
    let nome = match dt.date().weekday().num_days_from_monday() {
        0 => "lun",
        1 => "mar",
        2 => "mer",
        3 => "gio",
        4 => "ven",
        5 => "sab",
        _ => "dom",
    };
    format!("{nome} {}", dt.format("%H:%M"))
}

fn ordine_semaforo(s: Semaforo) -> u8 {
    match s {
        Semaforo::Verde | Semaforo::Inattivo => 0,
        Semaforo::Giallo => 1,
        Semaforo::Arancio => 2,
        Semaforo::Rosso => 3,
    }
}

fn peggiore(a: Semaforo, b: Semaforo) -> Semaforo {
    if ordine_semaforo(b) > ordine_semaforo(a) { b } else { a }
}

fn nome_stato_umidita(s: StatoUmidita) -> &'static str {
    match s {
        StatoUmidita::Tempera => "in tempera",
        StatoUmidita::Secco => "secco",
        StatoUmidita::Eccesso => "in eccesso idrico",
    }
}

/// Valutazione idrica di uno strato del suolo (stato e frazione di acqua disponibile).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ValutazioneStrato {
    pub stato: StatoUmidita,
    pub frazione: f64,
}

/// Calcola lo stato idrico e la frazione di acqua disponibile `(u - PA) / (CC - PA)`.
pub fn valuta_strato_umidita(u: f64, suolo: ParametriSuolo) -> ValutazioneStrato {
    let denom = (suolo.capacita_campo - suolo.punto_appassimento).max(1e-6);
    let frazione = (u - suolo.punto_appassimento) / denom;
    ValutazioneStrato { stato: stato_umidita(u, suolo), frazione }
}

/// Singolo giorno analizzato per il Periodo di Smith.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GiornoSmith {
    pub data: NaiveDate,
    pub n: usize,
    pub tmin: f64,
    pub umide: usize,
    pub ok: bool,
}

/// Intervallo di almeno 2 giorni consecutivi che soddisfano i criteri di Smith.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PeriodoSmith {
    pub inizio: NaiveDate,
    pub fine: NaiveDate,
}

/// Risultato dell'analisi di Smith sull'orizzonte orario.
#[derive(Debug, Clone, Serialize)]
pub struct RisultatoSmith {
    pub giorni: Vec<GiornoSmith>,
    pub periodi: Vec<PeriodoSmith>,
}

/// Periodo di Smith (peronospora della patata e del pomodoro): due o più giorni
/// consecutivi completi (≥ 20 ore) con temperatura minima ≥ 10 °C e almeno 11 ore
/// con umidità relativa ≥ 90 %.
pub fn periodi_smith(ore: &[Ora]) -> RisultatoSmith {
    let mut per_giorno: BTreeMap<NaiveDate, (usize, f64, usize)> = BTreeMap::new();
    for o in ore {
        let voce = per_giorno.entry(o.time.date()).or_insert((0, f64::INFINITY, 0));
        voce.0 += 1;
        if o.t.is_finite() && o.t < voce.1 {
            voce.1 = o.t;
        }
        if o.rh >= 90.0 {
            voce.2 += 1;
        }
    }

    let giorni: Vec<GiornoSmith> = per_giorno
        .into_iter()
        .filter(|(_, (n, _, _))| *n >= 20)
        .map(|(data, (n, tmin, umide))| GiornoSmith {
            data,
            n,
            tmin,
            umide,
            ok: tmin >= 10.0 && umide >= 11,
        })
        .collect();

    let mut periodi = Vec::new();
    let mut inizio: Option<usize> = None;
    for (i, g) in giorni.iter().enumerate() {
        if g.ok && inizio.is_none() {
            inizio = Some(i);
        }
        let chiude = !g.ok || i + 1 == giorni.len();
        if chiude && let Some(idx_inizio) = inizio {
            let fine = if g.ok { i } else { i.saturating_sub(1) };
            if fine > idx_inizio {
                periodi.push(PeriodoSmith {
                    inizio: giorni[idx_inizio].data,
                    fine: giorni[fine].data,
                });
            }
            inizio = None;
        }
    }

    RisultatoSmith { giorni, periodi }
}

/// Contesto sintetico estratto dal report per alimentare i consigli e le schede.
#[derive(Debug, Clone, Serialize)]
pub struct ContestoAgro {
    pub oggi: NaiveDate,
    pub mese: u32,
    pub t6: f64,
    pub stato_semina: IdoneitaSemina,
    pub superficiale: ValutazioneStrato,
    pub profondo: ValutazioneStrato,
    pub t_min_72: f64,
    pub ora_min: Option<NaiveDateTime>,
    pub t_max_72: f64,
    pub pioggia_48: f64,
    pub pioggia_72: f64,
    pub pioggia_passata_7: f64,
    pub pioggia_futura_7: f64,
    pub pioggia_futura_3: f64,
    pub et0_futura_7: f64,
    pub pioggia_efficace_futura_7: f64,
    pub giorni_futuri: usize,
    pub thi_ora: f64,
    pub thi_max_72: f64,
    pub ore_umide_72: usize,
    pub mattine_umide: Vec<NaiveDate>,
    pub giorni_bui: usize,
    pub finestra_secca: Vec<NaiveDate>,
    pub et0_media_finestra_secca: f64,
    pub giorni_lavorabili: Vec<NaiveDate>,
    pub smith: RisultatoSmith,
}

/// Costruisce il contesto agronomico dai dati già calcolati nel report.
pub fn costruisci_contesto(
    ora_corrente: NaiveDateTime,
    indice_ora: usize,
    ore: &[Ora],
    giorni: &[GiornoAgro],
    suolo_stato: &StatoSuolo,
    kpi: &Kpi,
    parametri: &ParametriAgro,
) -> ContestoAgro {
    let oggi = ora_corrente.date();
    let mese = oggi.month();
    let fine_72 = (indice_ora + 72).min(ore.len());
    let fine_48 = (indice_ora + 48).min(ore.len());
    let prossime_72 = &ore[indice_ora..fine_72];
    let prossime_48 = &ore[indice_ora..fine_48];

    let futuri: Vec<&GiornoAgro> = giorni.iter().filter(|g| g.meteo.data > oggi).collect();
    let passati: Vec<&GiornoAgro> = giorni.iter().filter(|g| g.meteo.data < oggi).collect();

    let mut t_min_72 = f64::INFINITY;
    let mut ora_min = None;
    let mut t_max_72 = f64::NEG_INFINITY;
    let mut thi_max_72 = f64::NEG_INFINITY;
    for o in prossime_72 {
        if o.t.is_finite() {
            if o.t < t_min_72 {
                t_min_72 = o.t;
                ora_min = Some(o.time);
            }
            if o.t > t_max_72 {
                t_max_72 = o.t;
            }
        }
        let v_thi = thi(o.t, o.rh);
        if v_thi.is_finite() && v_thi > thi_max_72 {
            thi_max_72 = v_thi;
        }
    }

    // Rugiada o brina al mattino (ore 4–9): aria quasi satura sulla vegetazione.
    let mattine_umide: BTreeSet<NaiveDate> = prossime_72
        .iter()
        .filter(|o| {
            let h = o.time.time().hour();
            (4..=9).contains(&h) && (o.rh >= 95.0 || o.t - o.td <= 1.5)
        })
        .map(|o| o.time.date())
        .collect();

    // Giorni consecutivi asciutti a partire da domani (finestra di fienagione).
    let mut finestra_secca = Vec::new();
    let mut somma_et0_secca = 0.0;
    for g in &futuri {
        if g.meteo.pioggia < 1.0 {
            finestra_secca.push(g.meteo.data);
            if g.meteo.et0.is_finite() {
                somma_et0_secca += g.meteo.et0;
            }
        } else {
            break;
        }
    }
    let et0_media_finestra_secca = if finestra_secca.is_empty() {
        0.0
    } else {
        somma_et0_secca / finestra_secca.len() as f64
    };

    let superficiale = valuta_strato_umidita(suolo_stato.umidita[2], parametri.suolo);
    let profondo = valuta_strato_umidita(suolo_stato.umidita[3], parametri.suolo);

    let somma = |vals: &[f64]| vals.iter().copied().filter(|v| v.is_finite()).sum::<f64>();

    ContestoAgro {
        oggi,
        mese,
        t6: suolo_stato.semina.t_suolo_6cm,
        stato_semina: suolo_stato.semina.clone(),
        superficiale,
        profondo,
        t_min_72,
        ora_min,
        t_max_72,
        pioggia_48: somma(&prossime_48.iter().map(|o| o.pioggia).collect::<Vec<_>>()),
        pioggia_72: somma(&prossime_72.iter().map(|o| o.pioggia).collect::<Vec<_>>()),
        pioggia_passata_7: somma(&passati.iter().map(|g| g.meteo.pioggia).collect::<Vec<_>>()),
        pioggia_futura_7: somma(&futuri.iter().map(|g| g.meteo.pioggia).collect::<Vec<_>>()),
        pioggia_futura_3: somma(&futuri.iter().take(3).map(|g| g.meteo.pioggia).collect::<Vec<_>>()),
        et0_futura_7: somma(&futuri.iter().map(|g| g.meteo.et0).collect::<Vec<_>>()),
        pioggia_efficace_futura_7: somma(&futuri.iter().map(|g| g.pioggia_efficace).collect::<Vec<_>>()),
        giorni_futuri: futuri.len(),
        thi_ora: kpi.thi,
        thi_max_72,
        ore_umide_72: prossime_72.iter().filter(|o| o.rh >= 90.0).count(),
        mattine_umide: mattine_umide.into_iter().collect(),
        giorni_bui: futuri.iter().filter(|g| g.meteo.ore_sole < 3.0 && g.meteo.t_max < 14.0).count(),
        finestra_secca,
        et0_media_finestra_secca,
        giorni_lavorabili: futuri
            .iter()
            .filter(|g| g.semina.stato_umidita == StatoUmidita::Tempera)
            .map(|g| g.meteo.data)
            .collect(),
        smith: periodi_smith(ore),
    }
}

/// Verifica se la coltura è presumibilmente in campo nel mese dato (1–12),
/// includendo i mesi intermedi tra semina/trapianto e raccolta/utilizzo.
pub fn in_campo(scheda: &SchedaColturale, mese: u32) -> bool {
    let Some(c) = &scheda.calendario else {
        return false;
    };
    if c.semina.contains(&mese)
        || c.trapianto.contains(&mese)
        || c.raccolta.contains(&mese)
        || c.utilizzo.contains(&mese)
    {
        return true;
    }
    let fine: BTreeSet<u32> = c.raccolta.iter().chain(c.utilizzo.iter()).copied().collect();
    for &inizio in c.semina.iter().chain(c.trapianto.iter()) {
        for k in 0..12 {
            let mm = ((inizio - 1 + k) % 12) + 1;
            if mm == mese {
                return true;
            }
            if fine.contains(&mm) {
                break;
            }
        }
    }
    false
}

/// Esito del confronto tra temperatura del suolo a 6 cm e soglie cardinali della coltura.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerificaSuolo {
    pub ok: bool,
    pub motivo: &'static str,
    pub testo: String,
}

/// Confronta la temperatura del suolo a 6 cm con le soglie cardinali della coltura.
pub fn verifica_suolo(scheda: &SchedaColturale, t6: f64) -> Option<VerificaSuolo> {
    let s = scheda.t_suolo.as_ref()?;
    if !t6.is_finite() {
        return None;
    }
    if t6 < s.min {
        Some(VerificaSuolo {
            ok: false,
            motivo: "freddo",
            testo: format!("Suolo freddo: {} °C, servono almeno {} °C", fmt_it(t6, 1), fmt_it(s.min, 0)),
        })
    } else if let Some(max) = s.max
        && t6 > max
    {
        Some(VerificaSuolo {
            ok: false,
            motivo: "caldo",
            testo: format!(
                "Suolo troppo caldo: {} °C, oltre {} °C la germinazione è inibita",
                fmt_it(t6, 1),
                fmt_it(max, 0)
            ),
        })
    } else {
        Some(VerificaSuolo {
            ok: true,
            motivo: "ok",
            testo: format!("Suolo a {} °C, sopra la minima di {} °C", fmt_it(t6, 1), fmt_it(s.min, 0)),
        })
    }
}

/// Indicatore sintetico («chip») mostrato nella testata di una scheda colturale.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChipColtura {
    pub livello: Semaforo,
    pub testo: String,
}

/// Valuta lo stato attuale di una coltura rispetto al mese e alle condizioni meteo/suolo.
pub fn valuta_coltura(scheda: &SchedaColturale, ctx: &ContestoAgro) -> Vec<ChipColtura> {
    let Some(c) = &scheda.calendario else {
        return Vec::new();
    };
    let m = ctx.mese;
    let mut chip = Vec::new();
    let semina = c.semina.contains(&m);
    let trapianto = c.trapianto.contains(&m);
    if semina {
        chip.push(ChipColtura { livello: Semaforo::Verde, testo: "Epoca di semina".into() });
    }
    if trapianto {
        chip.push(ChipColtura { livello: Semaforo::Verde, testo: "Epoca di trapianto".into() });
    }
    if c.semenzaio.contains(&m) {
        chip.push(ChipColtura { livello: Semaforo::Verde, testo: "Semina in semenzaio".into() });
    }
    if c.raccolta.contains(&m) {
        chip.push(ChipColtura { livello: Semaforo::Inattivo, testo: "In raccolta".into() });
    }
    if c.utilizzo.contains(&m) {
        chip.push(ChipColtura { livello: Semaforo::Inattivo, testo: "In utilizzo".into() });
    }
    if chip.is_empty() {
        chip.push(ChipColtura { livello: Semaforo::Inattivo, testo: "Fuori epoca".into() });
    }

    if (semina || trapianto)
        && let Some(v) = verifica_suolo(scheda, ctx.t6)
    {
        chip.push(ChipColtura {
            livello: if v.ok { Semaforo::Verde } else { Semaforo::Giallo },
            testo: v.testo,
        });
    }

    if let Some(gelo) = scheda.gelo
        && ctx.t_min_72.is_finite()
        && in_campo(scheda, m)
        && ctx.t_min_72 <= gelo + 2.0
    {
        chip.push(ChipColtura {
            livello: if ctx.t_min_72 <= gelo { Semaforo::Rosso } else { Semaforo::Giallo },
            testo: format!(
                "Minima prevista {} °C (danni sotto {} °C)",
                fmt_it(ctx.t_min_72, 1),
                fmt_it(gelo, 0)
            ),
        });
    }
    chip
}

/// Riferimento rapido a una coltura cliccabile all'interno di un consiglio o calendario.
#[derive(Debug, Clone, Serialize)]
pub struct RifColtura {
    pub id: &'static str,
    pub nome: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classe: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titolo: Option<String>,
}

/// Sotto-gruppo di colture in un consiglio (es. colture in attesa o in semenzaio).
#[derive(Debug, Clone, Serialize)]
pub struct GruppoDopo {
    pub testo: String,
    pub colture: Vec<RifColtura>,
}

/// Scheda di consiglio operativo dinamico per la sezione «Oggi in campo».
#[derive(Debug, Clone, Serialize)]
pub struct ConsiglioGiorno {
    pub id: &'static str,
    pub illustrazione: &'static str,
    pub titolo: &'static str,
    pub livello: Semaforo,
    pub dato: String,
    pub testi: Vec<String>,
    pub colture: Vec<RifColtura>,
    pub dopo: Vec<GruppoDopo>,
    pub perche: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<&'static str>,
}

fn consiglio_semina(ctx: &ContestoAgro, schede: &[SchedaColturale]) -> ConsiglioGiorno {
    let m = ctx.mese;
    let mut pronte = Vec::new();
    let mut attesa = Vec::new();

    for s in schede.iter().filter(|s| {
        s.t_suolo.is_some()
            && s.calendario
                .as_ref()
                .is_some_and(|c| c.semina.contains(&m) || c.trapianto.contains(&m))
    }) {
        let v = verifica_suolo(s, ctx.t6);
        let ok = v.as_ref().is_some_and(|x| x.ok);
        let rif = RifColtura {
            id: s.id,
            nome: s.nome,
            classe: Some(if ok { "ok" } else { "attesa" }),
            titolo: v.map(|x| x.testo),
        };
        if ok {
            pronte.push(rif);
        } else {
            attesa.push(rif);
        }
    }

    let semenzaio: Vec<RifColtura> = schede
        .iter()
        .filter(|s| s.calendario.as_ref().is_some_and(|c| c.semenzaio.contains(&m)))
        .map(|s| RifColtura {
            id: s.id,
            nome: s.nome,
            classe: None,
            titolo: Some("Semina in semenzaio protetto".into()),
        })
        .collect();

    let stato = ctx.stato_semina.stato_umidita;
    let livello = match stato {
        StatoUmidita::Tempera => Semaforo::Verde,
        StatoUmidita::Eccesso => Semaforo::Rosso,
        StatoUmidita::Secco => Semaforo::Giallo,
    };

    let mut testi = Vec::new();
    match stato {
        StatoUmidita::Tempera => testi.push("Il letto di semina è in tempera: condizioni adatte a seminare le colture in elenco.".into()),
        StatoUmidita::Secco => testi.push("Lo strato superficiale è secco: seminare in irriguo o attendere una pioggia utile (20–25 mm per le colture in asciutto), per evitare emergenze scalari o una falsa partenza.".into()),
        StatoUmidita::Eccesso => testi.push("Suolo in eccesso idrico: attendere che sgrondi prima di entrare con le macchine, per non compattarlo.".into()),
    }
    if !pronte.is_empty() {
        testi.push("Temperatura del suolo adatta per:".into());
    }

    let mut dopo = Vec::new();
    if !attesa.is_empty() {
        dopo.push(GruppoDopo {
            testo: "In epoca, ma con suolo non ancora idoneo:".into(),
            colture: attesa,
        });
    }
    if !semenzaio.is_empty() {
        dopo.push(GruppoDopo {
            testo: "In semenzaio protetto:".into(),
            colture: semenzaio,
        });
    }

    ConsiglioGiorno {
        id: "semina",
        illustrazione: "cons-semina",
        titolo: "Cosa seminare o trapiantare ora",
        livello,
        dato: format!("Suolo a 6 cm {} °C · strato 3–9 cm {}", fmt_it(ctx.t6, 1), nome_stato_umidita(stato)),
        testi,
        colture: pronte,
        dopo,
        perche: "La temperatura del suolo a 6 cm governa la germinazione: sotto la minima cardinale il seme assorbe acqua ma non cresce, e resta esposto a marciumi e insetti.",
        link: Some("letto-semina"),
    }
}

fn consiglio_lavorazioni(ctx: &ContestoAgro) -> ConsiglioGiorno {
    let p = ctx.profondo;
    let pioggia_forte = ctx.pioggia_72 > 25.0;
    let mut livello = match p.stato {
        StatoUmidita::Tempera => Semaforo::Verde,
        StatoUmidita::Eccesso => Semaforo::Rosso,
        StatoUmidita::Secco => Semaforo::Giallo,
    };

    let mut testi = Vec::new();
    match p.stato {
        StatoUmidita::Tempera => testi.push("Lo strato 9–27 cm, quello interessato dall'aratura, è in tempera: buone condizioni per arare o ripuntare.".into()),
        StatoUmidita::Eccesso => testi.push("Strato profondo sopra la capacità di campo: arare ora formerebbe una suola compatta e zolle lisciate. Attendere qualche giorno asciutto.".into()),
        StatoUmidita::Secco => testi.push("Strato profondo secco: l'aratura darebbe zolle grossolane e alti consumi. Nei suoli compattati è invece il momento giusto per una ripuntatura, che fessura meglio il suolo asciutto.".into()),
    }
    if pioggia_forte {
        livello = peggiore(livello, Semaforo::Arancio);
        testi.push(format!(
            "Previsti {} mm in 72 ore: evitare di lasciare suolo appena lavorato sui versanti, dove il rischio di erosione è massimo.",
            fmt_it(ctx.pioggia_72, 0)
        ));
    }
    if !ctx.giorni_lavorabili.is_empty() {
        let elenco: Vec<String> = ctx.giorni_lavorabili.iter().copied().map(giorno_breve_it).collect();
        testi.push(format!("Giorni con letto di semina in tempera: {}.", elenco.join(", ")));
    }

    ConsiglioGiorno {
        id: "lavorazioni",
        illustrazione: "cons-lavorazioni",
        titolo: "Aratura e lavorazioni",
        livello,
        dato: format!(
            "Acqua disponibile 9–27 cm: {} % · 3–9 cm: {} · pioggia 72 h {} mm",
            fmt_it((p.frazione * 100.0).max(0.0), 0),
            nome_stato_umidita(ctx.superficiale.stato),
            fmt_it(ctx.pioggia_72, 1)
        ),
        testi,
        colture: Vec::new(),
        dopo: Vec::new(),
        perche: "Un suolo plastico si deforma sotto ruote e versoio: i macropori collassano e si forma la suola di lavorazione, che limita radici e infiltrazione per anni.",
        link: Some("aratura"),
    }
}

fn consiglio_gelate(ctx: &ContestoAgro, schede: &[SchedaColturale]) -> ConsiglioGiorno {
    let t = ctx.t_min_72;
    let livello = if t <= 0.0 {
        Semaforo::Rosso
    } else if t <= 3.0 {
        Semaforo::Giallo
    } else {
        Semaforo::Verde
    };

    let sensibili: Vec<RifColtura> = schede
        .iter()
        .filter(|s| s.gelo.is_some_and(|g| in_campo(s, ctx.mese) && t <= g + 2.0))
        .map(|s| {
            let g = s.gelo.unwrap_or(0.0);
            RifColtura {
                id: s.id,
                nome: s.nome,
                classe: if t <= g { Some("attesa") } else { None },
                titolo: Some(format!("danni sotto {} °C", fmt_it(g, 0))),
            }
        })
        .collect();

    let mut testi = Vec::new();
    if livello == Semaforo::Verde {
        testi.push("Nessun rischio di gelo nelle prossime 72 ore.".into());
    } else {
        let quando = ctx.ora_min.map(ora_breve_it).unwrap_or_default();
        testi.push(format!("Minima prevista {} °C ({quando}).", fmt_it(t, 1)));
        testi.push("Coprire le orticole sensibili con tessuto non tessuto (guadagno di 1–3 °C), raccogliere i carciofi pronti, non lavorare il suolo la sera prima: un suolo compatto e umido cede più calore di uno appena smosso.".into());
    }
    if !sensibili.is_empty() {
        testi.push("Colture in campo vicine alla soglia di danno:".into());
    }

    ConsiglioGiorno {
        id: "gelate",
        illustrazione: "cons-gelate",
        titolo: "Gelate e protezione",
        livello,
        dato: format!("Minima 72 h {} °C · massima {} °C", fmt_it(t, 1), fmt_it(ctx.t_max_72, 1)),
        testi,
        colture: sensibili,
        dopo: Vec::new(),
        perche: "Le gelate per irraggiamento si formano con cielo sereno e calma di vento: il suolo perde calore verso il cielo e l'aria fredda, più densa, ristagna negli avvallamenti.",
        link: None,
    }
}

fn consiglio_acqua(ctx: &ContestoAgro) -> ConsiglioGiorno {
    let f = ctx.profondo.frazione;
    let fabbisogno = (ctx.et0_futura_7 * 1.1 - ctx.pioggia_efficace_futura_7).max(0.0);
    let livello = if f > 1.0 {
        Semaforo::Verde
    } else if f < 0.3 {
        Semaforo::Arancio
    } else if f < 0.5 {
        Semaforo::Giallo
    } else {
        Semaforo::Verde
    };

    let mut testi = Vec::new();
    if f > 1.0 {
        testi.push("Suolo sopra la capacità di campo: nessuna irrigazione, controllare i ristagni.".into());
    } else if f < 0.5 {
        testi.push("Nello strato radicale resta meno della metà dell'acqua disponibile: per le orticole e la medica irrigua è il momento di intervenire.".into());
    } else {
        testi.push("Riserva idrica sufficiente nello strato radicale.".into());
    }
    testi.push(format!(
        "Per una coltura a pieno sviluppo (Kc 1,1) nei prossimi {} giorni servono circa {} mm ({} L/m²) oltre alla pioggia efficace prevista.",
        ctx.giorni_futuri,
        fmt_it(fabbisogno, 0),
        fmt_it(fabbisogno, 0)
    ));

    ConsiglioGiorno {
        id: "acqua",
        illustrazione: "cons-acqua",
        titolo: "Acqua e irrigazione",
        livello,
        dato: format!(
            "Acqua disponibile 9–27 cm: {} % · ET0 prevista {} mm · pioggia {} mm",
            fmt_it((f * 100.0).max(0.0), 0),
            fmt_it(ctx.et0_futura_7, 0),
            fmt_it(ctx.pioggia_futura_7, 0)
        ),
        testi,
        colture: Vec::new(),
        dopo: Vec::new(),
        perche: "Secondo FAO-56 la maggior parte delle orticole inizia a ridurre la traspirazione quando si esaurisce il 30–50 % dell'acqua disponibile: irrigare prima di quella soglia evita lo stress.",
        link: Some("principi-orticoltura"),
    }
}

fn consiglio_azoto(ctx: &ContestoAgro) -> ConsiglioGiorno {
    let p = ctx.pioggia_72;
    let mut testi = Vec::new();
    let livello = if ctx.stato_semina.stato_umidita == StatoUmidita::Eccesso {
        testi.push("Suolo saturo: rinviare la distribuzione, l'azoto verrebbe perso per dilavamento o denitrificazione e il passaggio delle macchine compatterebbe il suolo.".into());
        Semaforo::Arancio
    } else if p > 25.0 {
        testi.push(format!(
            "Previsti {} mm: rinviare la concimazione in copertura. Piogge intense spingono i nitrati sotto le radici, soprattutto nei suoli sciolti.",
            fmt_it(p, 0)
        ));
        Semaforo::Arancio
    } else if p >= 5.0 {
        testi.push(format!(
            "Finestra favorevole: una pioggia moderata ({} mm in 72 ore) incorporerà il concime nel suolo, riducendo le perdite di ammoniaca dell'urea.",
            fmt_it(p, 0)
        ));
        Semaforo::Verde
    } else {
        testi.push("Pioggia scarsa: l'urea lasciata in superficie perde ammoniaca per volatilizzazione. Interrarla, irrigare con 10–15 mm subito dopo o preferire concimi nitrici-ammoniacali.".into());
        Semaforo::Giallo
    };

    if matches!(ctx.mese, 1..=4) {
        testi.push("Per cereali ed erbai è la stagione delle coperture: distribuire tra fine accestimento e inizio levata.".into());
    }

    ConsiglioGiorno {
        id: "azoto",
        illustrazione: "cons-azoto",
        titolo: "Concimazione azotata in copertura",
        livello,
        dato: format!("Pioggia 48 h {} mm · 72 h {} mm", fmt_it(ctx.pioggia_48, 1), fmt_it(p, 1)),
        testi,
        colture: Vec::new(),
        dopo: Vec::new(),
        perche: "Il nitrato è solubile e si muove con l'acqua; l'urea in superficie si trasforma in ammoniaca che si disperde nell'aria. Una pioggia di 5–20 mm porta l'azoto alle radici senza dilavarlo.",
        link: Some("avena"),
    }
}

fn consiglio_peronospora(ctx: &ContestoAgro, schede: &[SchedaColturale]) -> ConsiglioGiorno {
    let futuri: Vec<&PeriodoSmith> = ctx.smith.periodi.iter().filter(|p| p.fine >= ctx.oggi).collect();
    let recenti: Vec<&PeriodoSmith> = ctx.smith.periodi.iter().filter(|p| p.fine < ctx.oggi).collect();

    let mut testi = Vec::new();
    let livello = if !futuri.is_empty() {
        let el: Vec<String> = futuri
            .iter()
            .map(|p| format!("{}–{}", giorno_breve_it(p.inizio), giorno_breve_it(p.fine)))
            .collect();
        testi.push(format!(
            "Periodo di Smith in corso o previsto ({}): condizioni favorevoli alle infezioni di peronospora su pomodoro e patata. Proteggere la vegetazione prima dell'evento.",
            el.join(", ")
        ));
        Semaforo::Arancio
    } else if !recenti.is_empty() {
        let el: Vec<String> = recenti.iter().map(|p| giorno_breve_it(p.fine)).collect();
        testi.push(format!(
            "Periodo di Smith registrato negli ultimi giorni ({}): controllare le colture per le prime macchie nei prossimi 5–7 giorni.",
            el.join(", ")
        ));
        Semaforo::Giallo
    } else if ctx.ore_umide_72 >= 24 {
        testi.push(format!(
            "{} ore con umidità ≥ 90 % nelle prossime 72: rischio moderato di malattie fogliari. Evitare l'irrigazione a pioggia.",
            ctx.ore_umide_72
        ));
        Semaforo::Giallo
    } else {
        testi.push("Nessun periodo di Smith previsto: rischio di peronospora basso.".into());
        Semaforo::Verde
    };

    let colture: Vec<RifColtura> = schede
        .iter()
        .filter(|s| matches!(s.id, "pomodoro" | "patata") && in_campo(s, ctx.mese))
        .map(|s| RifColtura { id: s.id, nome: s.nome, classe: None, titolo: None })
        .collect();

    if colture.is_empty() {
        testi.push("Pomodoro e patata non sono di norma in campo in questo mese: indicazione utile per serre e cicli fuori stagione.".into());
    }

    ConsiglioGiorno {
        id: "peronospora",
        illustrazione: "cons-peronospora",
        titolo: "Peronospora di pomodoro e patata",
        livello,
        dato: format!("Ore con UR ≥ 90 % nelle prossime 72: {}", ctx.ore_umide_72),
        testi,
        colture,
        dopo: Vec::new(),
        perche: "Phytophthora infestans produce spore e infetta quando l'aria resta satura per molte ore con temperature miti: il periodo di Smith (2 giorni con minima ≥ 10 °C e almeno 11 ore con UR ≥ 90 %) ne è l'indicatore classico.",
        link: Some("pomodoro"),
    }
}

fn consiglio_ovini(ctx: &ContestoAgro) -> ConsiglioGiorno {
    let t = ctx.thi_max_72;
    let mut livello = if t >= 78.0 {
        Semaforo::Rosso
    } else if t >= 72.0 {
        Semaforo::Arancio
    } else if t >= 68.0 {
        Semaforo::Giallo
    } else {
        Semaforo::Verde
    };

    let mut testi = Vec::new();
    if t >= 72.0 {
        testi.push(format!(
            "THI massimo previsto {}: pascolo nelle ore fresche (alba, sera o notte), ombra e acqua fresca a volontà.",
            fmt_it(t, 1)
        ));
    } else {
        testi.push(format!("THI massimo previsto {}: nessuno stress da caldo.", fmt_it(t, 1)));
    }

    if !ctx.mattine_umide.is_empty() {
        livello = peggiore(livello, Semaforo::Giallo);
        let giorni: Vec<String> = ctx.mattine_umide.iter().copied().map(giorno_breve_it).collect();
        testi.push(format!(
            "Rugiada o brina al mattino ({}): portare il gregge su erbai ricchi di leguminose solo dopo che l'erba si è asciugata, e con fieno già nel rumine, per prevenire il meteorismo.",
            giorni.join(", ")
        ));
    }

    if ctx.giorni_bui >= 2 {
        livello = peggiore(livello, Semaforo::Giallo);
        testi.push(format!(
            "{} giorni freddi e poco soleggiati in arrivo: sugli erbai di graminacee concimati con azoto possono accumularsi nitrati. Pascolare nel pomeriggio e introdurre gradualmente.",
            ctx.giorni_bui
        ));
    }

    ConsiglioGiorno {
        id: "ovini",
        illustrazione: "cons-ovini",
        titolo: "Pecore al pascolo",
        livello,
        dato: format!("THI ora {} · max 72 h {}", fmt_it(ctx.thi_ora, 1), fmt_it(t, 1)),
        testi,
        colture: Vec::new(),
        dopo: Vec::new(),
        perche: "Il THI combina temperatura e umidità: oltre circa 72 la pecora riduce ingestione e latte per limitare il calore prodotto dalla digestione.",
        link: Some("rischi-sanitari"),
    }
}

fn consiglio_erbai(ctx: &ContestoAgro) -> Option<ConsiglioGiorno> {
    if !matches!(ctx.mese, 8..=12) {
        return None;
    }
    let pioggia = ctx.pioggia_passata_7 + ctx.pioggia_futura_3;
    let livello = if pioggia >= 20.0 {
        Semaforo::Verde
    } else if pioggia >= 10.0 {
        Semaforo::Giallo
    } else {
        Semaforo::Arancio
    };

    let mut testi = Vec::new();
    if pioggia >= 20.0 {
        testi.push(format!(
            "Pioggia sufficiente ({} mm tra ultimi 7 giorni e prossimi 3): buona finestra per seminare gli erbai autunno-vernini.",
            fmt_it(pioggia, 0)
        ));
    } else {
        testi.push(format!(
            "Solo {} mm tra ultimi 7 giorni e prossimi 3: la semina in asciutto rischia una falsa partenza.",
            fmt_it(pioggia, 0)
        ));
        if ctx.pioggia_futura_7 >= 20.0 {
            testi.push(format!(
                "La previsione a 7 giorni indica {} mm: si può seminare in asciutto poco prima dell'evento.",
                fmt_it(ctx.pioggia_futura_7, 0)
            ));
        } else {
            testi.push("Nessuna pioggia significativa nei prossimi 7 giorni: seminare solo con irrigazione di soccorso.".into());
        }
    }
    if ctx.mese >= 11 {
        testi.push("Gli erbai seminati ora arrivano all'inverno poco sviluppati: preferire specie precoci (orzo, loietto italico) e dosi di seme più alte.".into());
    }

    Some(ConsiglioGiorno {
        id: "erbai",
        illustrazione: "cons-erbai",
        titolo: "Semina degli erbai",
        livello,
        dato: format!(
            "Pioggia ultimi 7 giorni {} mm · prossimi 3 giorni {} mm",
            fmt_it(ctx.pioggia_passata_7, 0),
            fmt_it(ctx.pioggia_futura_3, 0)
        ),
        testi,
        colture: Vec::new(),
        dopo: Vec::new(),
        perche: "Per un'emergenza uniforme il seme deve restare in un suolo umido per diversi giorni: 20–25 mm cumulati bagnano lo strato di semina abbastanza a lungo.",
        link: Some("miscugli-erbai"),
    })
}

fn consiglio_fienagione(ctx: &ContestoAgro) -> Option<ConsiglioGiorno> {
    let n = ctx.finestra_secca.len();
    let stagione = (4..=9).contains(&ctx.mese);
    if !stagione && n < 3 {
        return None;
    }
    let livello = if n >= 4 {
        Semaforo::Verde
    } else if n == 3 {
        Semaforo::Giallo
    } else {
        Semaforo::Rosso
    };

    let mut testi = Vec::new();
    if n >= 4 {
        testi.push(format!(
            "{n} giorni asciutti consecutivi da domani (ET0 media {} mm/giorno): finestra adatta per il fieno.",
            fmt_it(ctx.et0_media_finestra_secca, 1)
        ));
    } else if n == 3 {
        testi.push("3 giorni asciutti: sufficienti per il fasciato, al limite per il fieno di leguminose.".into());
    } else {
        testi.push("Pioggia entro 2 giorni: non falciare per il fieno.".into());
    }
    if !stagione {
        testi.push("Fuori dalla stagione principale di fienagione (aprile–settembre).".into());
    }

    let dato = if let (Some(&inizio), Some(&fine)) = (ctx.finestra_secca.first(), ctx.finestra_secca.last()) {
        format!("Asciutto da {} a {}", giorno_breve_it(inizio), giorno_breve_it(fine))
    } else {
        "Pioggia prevista a breve".into()
    };

    Some(ConsiglioGiorno {
        id: "fieno",
        illustrazione: "cons-fieno",
        titolo: "Finestra di fienagione",
        livello,
        dato,
        testi,
        colture: Vec::new(),
        dopo: Vec::new(),
        perche: "Il foraggio deve scendere dal 75–80 % al 18–20 % di umidità: con buona evaporazione (ET0 alta) servono 3–4 giorni senza pioggia, che lava zuccheri e fa cadere le foglie.",
        link: Some("fieno-insilato"),
    })
}

/// Genera l'elenco ordinato dei consigli operativi del giorno.
pub fn consigli_oggi(ctx: &ContestoAgro, schede: &[SchedaColturale]) -> Vec<ConsiglioGiorno> {
    let mut out = vec![
        consiglio_semina(ctx, schede),
        consiglio_lavorazioni(ctx),
        consiglio_gelate(ctx, schede),
        consiglio_acqua(ctx),
        consiglio_azoto(ctx),
    ];
    if let Some(c) = consiglio_erbai(ctx) {
        out.push(c);
    }
    out.push(consiglio_ovini(ctx));
    out.push(consiglio_peronospora(ctx, schede));
    if let Some(c) = consiglio_fienagione(ctx) {
        out.push(c);
    }
    out
}

/// Gruppo di colture associate a un'operazione nel mese corrente.
#[derive(Debug, Clone, Serialize)]
pub struct GruppoOperazioneMese {
    pub chiave: &'static str,
    pub titolo: &'static str,
    pub icona: &'static str,
    pub colture: Vec<RifColtura>,
}

/// Operazioni colturali principali del mese attivo in Sardegna.
pub fn lavori_mese(mese: u32, schede: &[SchedaColturale]) -> Vec<GruppoOperazioneMese> {
    let operazioni: &[(&str, &str, &str, fn(&super::CalendarioColturale) -> &[u32])] = &[
        ("semina", "Semina in campo", "🌱", |c| c.semina),
        ("semenzaio", "Semina in semenzaio", "🪴", |c| c.semenzaio),
        ("trapianto", "Trapianto / impianto", "🌿", |c| c.trapianto),
        ("raccolta", "Raccolta", "🧺", |c| c.raccolta),
        ("utilizzo", "Pascolo / sfalcio", "🐑", |c| c.utilizzo),
    ];

    operazioni
        .iter()
        .filter_map(|&(chiave, titolo, icona, estrai)| {
            let colture: Vec<RifColtura> = schede
                .iter()
                .filter(|s| s.calendario.as_ref().is_some_and(|c| estrai(c).contains(&mese)))
                .map(|s| RifColtura { id: s.id, nome: s.nome, classe: None, titolo: None })
                .collect();
            (!colture.is_empty()).then_some(GruppoOperazioneMese { chiave, titolo, icona, colture })
        })
        .collect()
}

/// Risultato completo dell'analisi dell'Almanacco allegato a ogni `Report`.
#[derive(Debug, Clone, Serialize)]
pub struct AnalisiAlmanacco {
    pub mese: u32,
    pub nome_mese: &'static str,
    pub t_suolo_6cm: f64,
    pub t_min_72h: f64,
    pub consigli: Vec<ConsiglioGiorno>,
    pub lavori_mese: Vec<GruppoOperazioneMese>,
    pub stato_colture: BTreeMap<&'static str, Vec<ChipColtura>>,
}

/// Esegue l'analisi completa dell'Almanacco sul report corrente.
pub fn analizza_report(
    ora_corrente: NaiveDateTime,
    indice_ora: usize,
    ore: &[Ora],
    giorni: &[GiornoAgro],
    suolo_stato: &StatoSuolo,
    kpi: &Kpi,
    parametri: &ParametriAgro,
    schede: &[SchedaColturale],
) -> AnalisiAlmanacco {
    let ctx = costruisci_contesto(ora_corrente, indice_ora, ore, giorni, suolo_stato, kpi, parametri);
    let consigli = consigli_oggi(&ctx, schede);
    let lavori = lavori_mese(ctx.mese, schede);
    let stato_colture = schede
        .iter()
        .map(|s| (s.id, valuta_coltura(s, &ctx)))
        .collect();
    let idx_mese = (ctx.mese.saturating_sub(1) as usize).min(11);

    AnalisiAlmanacco {
        mese: ctx.mese,
        nome_mese: super::sezioni::NOMI_MESI[idx_mese],
        t_suolo_6cm: ctx.t6,
        t_min_72h: ctx.t_min_72,
        consigli,
        lavori_mese: lavori,
        stato_colture,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanacco::tutte_le_schede;

    #[test]
    fn verifica_presenza_in_campo() {
        let schede = tutte_le_schede();
        let pomodoro = schede.iter().find(|s| s.id == "pomodoro").unwrap();
        // Pomodoro: trapianto apr-mag (4-5), raccolta lug-ott (7-10) -> a giugno (6) è in campo!
        assert!(in_campo(pomodoro, 6));
        // A febbraio è solo in semenzaio protetto -> non in pieno campo
        assert!(!in_campo(pomodoro, 2));
        assert!(!in_campo(pomodoro, 12));
    }

    #[test]
    fn verifica_soglie_suolo_colture() {
        let schede = tutte_le_schede();
        let lattuga = schede.iter().find(|s| s.id == "lattuga").unwrap();
        assert_eq!(verifica_suolo(lattuga, 2.0).unwrap().motivo, "freddo");
        assert!(verifica_suolo(lattuga, 16.0).unwrap().ok);
        assert_eq!(verifica_suolo(lattuga, 27.0).unwrap().motivo, "caldo");
    }

    #[test]
    fn smith_rileva_due_giorni_consecutivi() {
        let base = NaiveDate::from_ymd_opt(2026, 5, 10).unwrap();
        let mut ore = Vec::new();
        for g in 0..3 {
            let d = base + chrono::Duration::days(g);
            for h in 0..24 {
                ore.push(Ora {
                    time: d.and_hms_opt(h, 0, 0).unwrap(),
                    t: if g < 2 { 14.0 } else { 8.0 },
                    rh: if h < 12 { 92.0 } else { 70.0 },
                    td: 12.0,
                    t_percepita: 14.0,
                    pioggia: 0.0,
                    pioggia_liquida: 0.0,
                    prob_pioggia: 0.0,
                    vento: 5.0,
                    vento_dir: 180.0,
                    raffiche: 8.0,
                    radiazione: 200.0,
                    dni: 100.0,
                    par: 400.0,
                    pressione: 1013.0,
                    nuvolosita: 50.0,
                    et0: 0.1,
                    vpd: 0.5,
                    suolo_t: [14.0; 4],
                    suolo_u: [0.25; 5],
                });
            }
        }
        let r = periodi_smith(&ore);
        assert_eq!(r.periodi.len(), 1);
        assert_eq!(r.periodi[0].inizio, base);
        assert_eq!(r.periodi[0].fine, base + chrono::Duration::days(1));
    }
}
