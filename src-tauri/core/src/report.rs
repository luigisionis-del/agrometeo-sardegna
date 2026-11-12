//! Assemblaggio del report decisionale: dai payload grezzi agli indicatori.
//!
//! `costruisci` è deterministica (riceve anche l'istante corrente), così il
//! report può essere ricalcolato offline dalla cache con parametri diversi.

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

use crate::agro::disease::{self, IngressoPrimaria};
use crate::agro::frost_soil::{self, IdoneitaSemina, ParametriSuolo, StatoUmidita};
use crate::agro::gdd::{self, GiornoGdd, T_BASE_FORAGGERE, T_BASE_VITE};
use crate::agro::spray::{self, CondizioniTrattamento};
use crate::agro::water::{self, ParametriPioggia};
use crate::agro::wetness;
use crate::agro::{Semaforo, Valutazione, media, media_indietro, somma_indietro, thi};
use crate::almanacco::{self, AnalisiAlmanacco};
use crate::model::{Giorno, Luogo, OpenMeteoForecast, OpenMeteoHistory, Ora, parse_data};

/// Parametri agronomici scelti dall'utente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ParametriAgro {
    /// Lunghezza media dei germogli di vite (cm); 0 = riposo vegetativo.
    pub germogli_cm: f64,
    /// Coefficiente colturale per il bilancio idrico.
    pub kc: f64,
    pub pioggia: ParametriPioggia,
    pub suolo: ParametriSuolo,
    /// Inizio del conteggio GDD per la vite (predefinito: 1 aprile).
    pub inizio_vite: Option<NaiveDate>,
    /// Inizio del conteggio GDD per erbai/foraggere (predefinito: 1 ottobre).
    pub inizio_foraggere: Option<NaiveDate>,
    /// Orizzonte della matrice decisionale (ore).
    pub ore_matrice: usize,
}

impl Default for ParametriAgro {
    fn default() -> Self {
        Self {
            germogli_cm: 0.0,
            kc: 0.7,
            pioggia: ParametriPioggia::default(),
            suolo: ParametriSuolo::default(),
            inizio_vite: None,
            inizio_foraggere: None,
            ore_matrice: 72,
        }
    }
}

/// Ultimo inizio di stagione (giorno/mese) non successivo a `oggi`.
pub fn inizio_stagione(oggi: NaiveDate, mese: u32, giorno: u32) -> NaiveDate {
    let quest_anno = NaiveDate::from_ymd_opt(oggi.year(), mese, giorno).expect("data valida");
    if quest_anno <= oggi { quest_anno } else { NaiveDate::from_ymd_opt(oggi.year() - 1, mese, giorno).expect("data valida") }
}

impl ParametriAgro {
    pub fn risolvi_date(&mut self, oggi: NaiveDate) {
        self.inizio_vite.get_or_insert_with(|| inizio_stagione(oggi, 4, 1));
        self.inizio_foraggere.get_or_insert_with(|| inizio_stagione(oggi, 10, 1));
    }

    /// Prima data per cui servono dati storici giornalieri.
    pub fn inizio_storico(&self) -> Option<NaiveDate> {
        [self.inizio_vite, self.inizio_foraggere].into_iter().flatten().min()
    }

    fn vite_in_vegetazione(&self) -> bool {
        self.germogli_cm > 0.0
    }
}

/// Dati scaricati per una località, così come vengono salvati in cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub scaricato_il: DateTime<Utc>,
    pub luogo: Luogo,
    pub previsione: OpenMeteoForecast,
    #[serde(default)]
    pub storico: Option<OpenMeteoHistory>,
    #[serde(default)]
    pub storico_da: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Fonte {
    Rete,
    Cache,
}

#[derive(Debug, Clone, Serialize)]
pub struct Kpi {
    pub t: f64,
    pub t_percepita: f64,
    pub rh: f64,
    pub td: f64,
    pub vpd: f64,
    pub thi: f64,
    pub pioggia_24h: f64,
    pub vento: f64,
    pub vento_dir: f64,
    pub raffiche: f64,
    pub pressione: f64,
    pub nuvolosita: f64,
    pub et0_oggi: f64,
    pub vpd_classe: Valutazione,
    pub thi_classe: Valutazione,
}

#[derive(Debug, Clone, Serialize)]
pub struct RigaMatrice {
    pub id: &'static str,
    pub titolo: &'static str,
    pub icona: &'static str,
    pub regola: &'static str,
    pub celle: Vec<Valutazione>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Matrice {
    pub ore: Vec<NaiveDateTime>,
    pub righe: Vec<RigaMatrice>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GiornoAgro {
    #[serde(flatten)]
    pub meteo: Giorno,
    pub pioggia_efficace: f64,
    pub etc: f64,
    pub delta_w: f64,
    pub bilancio_cumulato: f64,
    pub gdd_vite: f64,
    pub gdd_foraggere: f64,
    /// Produzione fotovoltaica stimata su piano orizzontale (kWh per kWp, PR 0.8).
    pub fv_kwh_kwp: f64,
    pub semina: IdoneitaSemina,
    pub previsto: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SommaTermica {
    pub coltura: &'static str,
    pub t_base: f64,
    pub inizio: NaiveDate,
    pub totale_oggi: f64,
    pub totale_previsto: f64,
    /// Vero se lo storico non copre l'intera stagione (cumulato parziale).
    pub parziale: bool,
    pub serie: Vec<GiornoGdd>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatoSuolo {
    pub profondita_cm: [u32; 4],
    pub temperature: [f64; 4],
    pub strati_cm: [(u32, u32); 5],
    pub umidita: [f64; 5],
    /// Frazione di acqua disponibile `(u - PA) / (CC - PA)` per ciascuno dei 5 strati.
    pub frazione_disponibile: [f64; 5],
    /// Stato idrico (`secco`, `tempera`, `eccesso`) per ciascuno dei 5 strati.
    pub stati_strati: [StatoUmidita; 5],
    pub semina: IdoneitaSemina,
}

#[derive(Debug, Clone, Serialize)]
pub struct Allerta {
    pub indice: &'static str,
    pub titolo: String,
    pub livello: Semaforo,
    pub inizio: NaiveDateTime,
    pub fine: NaiveDateTime,
    pub ore: usize,
    pub episodi: usize,
    pub motivo: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub luogo: Luogo,
    pub quota_modello: f64,
    pub lat_modello: f64,
    pub lon_modello: f64,
    pub fonte: Fonte,
    pub scaricato_il: DateTime<Utc>,
    pub generato_il: DateTime<Utc>,
    pub avvisi: Vec<String>,
    pub ora_corrente: NaiveDateTime,
    pub indice_ora: usize,
    pub kpi: Kpi,
    pub ore: Vec<Ora>,
    pub matrice: Matrice,
    pub giorni: Vec<GiornoAgro>,
    pub gdd: Vec<SommaTermica>,
    pub suolo: StatoSuolo,
    pub allerte: Vec<Allerta>,
    pub almanacco: AnalisiAlmanacco,
    pub parametri: ParametriAgro,
}

/// Ora locale al secondo (senza frazioni, che WebKit non interpreta).
pub fn ora_locale(adesso: DateTime<Utc>, utc_offset_seconds: i32) -> NaiveDateTime {
    let t = adesso.naive_utc() + Duration::seconds(utc_offset_seconds as i64);
    t.with_nanosecond(0).unwrap_or(t)
}

pub fn costruisci(
    payload: &Payload,
    parametri: &ParametriAgro,
    adesso: DateTime<Utc>,
    fonte: Fonte,
    mut avvisi: Vec<String>,
) -> Report {
    let prev = &payload.previsione;
    let ore = prev.hourly.normalizza();
    let giorni_meteo = prev.daily.normalizza();
    let ora_corrente = ora_locale(adesso, prev.utc_offset_seconds);
    let oggi = ora_corrente.date();

    let mut parametri = parametri.clone();
    parametri.risolvi_date(oggi);

    let indice_ora = ore.iter().rposition(|o| o.time <= ora_corrente).unwrap_or_default();
    if ore.last().is_some_and(|o| o.time < ora_corrente) {
        avvisi.push("I dati in cache sono più vecchi dell'orizzonte di previsione: aggiornare appena c'è rete.".into());
    }

    let t: Vec<f64> = ore.iter().map(|o| o.t).collect();
    let pioggia: Vec<f64> = ore.iter().map(|o| o.pioggia).collect();
    let suolo6: Vec<f64> = ore.iter().map(|o| o.suolo_t[1]).collect();
    let bagnata: Vec<bool> = ore.iter().map(|o| wetness::foglia_bagnata(o.pioggia, o.rh, o.t, o.td)).collect();
    let lwd = wetness::ore_consecutive(&bagnata);

    let fine = (indice_ora + parametri.ore_matrice).min(ore.len());
    let finestra = indice_ora..fine;
    let vegetazione = parametri.vite_in_vegetazione();

    let celle = |f: &dyn Fn(usize) -> Valutazione| finestra.clone().map(f).collect::<Vec<_>>();

    let trattamenti = celle(&|i| {
        let o = &ore[i];
        spray::finestra_trattamento(CondizioniTrattamento {
            vento_kmh: o.vento,
            raffiche_kmh: o.raffiche,
            t_aria: o.t,
            rh: o.rh,
            pioggia_entro_4h: spray::pioggia_nelle_prossime(&pioggia, i, 4),
            pioggia_entro_6h: spray::pioggia_nelle_prossime(&pioggia, i, 6),
        })
    });
    let bagnatura = celle(&|i| match lwd[i] {
        0 => Valutazione::new(Semaforo::Verde, "Asciutta", format!("RH {:.0} %, T−Td {:.1} °C", ore[i].rh, ore[i].t - ore[i].td)),
        n => Valutazione::new(
            if n >= 6 { Semaforo::Arancio } else { Semaforo::Giallo },
            format!("{n} h"),
            format!(
                "Foglia bagnata da {n} h (pioggia {:.1} mm, RH {:.0} %, T−Td {:.1} °C)",
                ore[i].pioggia,
                ore[i].rh,
                ore[i].t - ore[i].td
            ),
        ),
    });
    let primaria = celle(&|i| {
        disease::peronospora_primaria(IngressoPrimaria {
            t_media_24h: media_indietro(&t, i, 24),
            t_suolo_6cm: suolo6[i],
            pioggia_48h: somma_indietro(&pioggia, i, 48),
            germogli_cm: parametri.germogli_cm,
        })
    });
    let secondaria = celle(&|i| {
        let n = lwd[i] as usize;
        let t_bagnatura = if n == 0 { f64::NAN } else { media(&t[i + 1 - n..=i]) };
        disease::peronospora_secondaria(lwd[i], t_bagnatura, vegetazione)
    });
    let oidio = celle(&|i| {
        let o = &ore[i];
        disease::oidio(o.t, o.rh, somma_indietro(&pioggia, i, 24), o.radiazione, vegetazione)
    });
    let stress_thi = celle(&|i| thi::classe_thi(thi::thi(ore[i].t, ore[i].rh)));
    let gelata = celle(&|i| {
        let o = &ore[i];
        frost_soil::allerta_gelata(o.t, o.td, o.nuvolosita, o.vento)
    });
    let vpd = celle(&|i| water::classe_vpd(ore[i].vpd));

    let matrice = Matrice {
        ore: finestra.clone().map(|i| ore[i].time).collect(),
        righe: vec![
            RigaMatrice {
                id: "trattamenti",
                titolo: "Finestra trattamenti",
                icona: "mat-trattamenti",
                regola: "Ottimale con vento < 10 km/h, raffiche < 15 km/h, T 12–25 °C, UR 40–80 % e assenza di pioggia nelle 6 h successive.",
                celle: trattamenti,
            },
            RigaMatrice {
                id: "bagnatura",
                titolo: "Bagnatura fogliare",
                icona: "mat-bagnatura",
                regola: "Foglia bagnata con pioggia > 0,1 mm/h, UR ≥ 90 % o scarto di rugiada T−Td ≤ 2,0 °C; arancio oltre 6 ore consecutive.",
                celle: bagnatura,
            },
            RigaMatrice {
                id: "peronospora_primaria",
                titolo: "Peronospora primaria",
                icona: "mat-peronospora",
                regola: "Regola dei 3 dieci: germogli > 10 cm, T media 24h o T suolo 6cm ≥ 10 °C e pioggia cumulata 48h ≥ 10 mm.",
                celle: primaria,
            },
            RigaMatrice {
                id: "peronospora_secondaria",
                titolo: "Peronospora secondaria",
                icona: "mat-secondaria",
                regola: "Modello Goidanich semplificato: rischio alto con bagnatura fogliare ≥ 6 h e temperatura media 18–24 °C.",
                celle: secondaria,
            },
            RigaMatrice {
                id: "oidio",
                titolo: "Oidio",
                icona: "mat-oidio",
                regola: "Rischio alto con T 20–27 °C, UR > 45 % e pioggia 24h < 2 mm; inibito con T > 33 °C e radiazione ≥ 600 W/m².",
                celle: oidio,
            },
            RigaMatrice {
                id: "thi",
                titolo: "Stress THI ovini/caprini",
                icona: "mat-thi",
                regola: "Indice NRC temperatura-umidità: benessere < 68, disagio lieve 68–72, stress moderato 72–78, severo ≥ 78.",
                celle: stress_thi,
            },
            RigaMatrice {
                id: "gelata",
                titolo: "Gelate",
                icona: "mat-gelata",
                regola: "Allerta rossa con T ≤ 0 °C o gelata per irraggiamento (T ≤ 3 °C, Td ≤ 0 °C, nubi < 25 %, vento < 8 km/h).",
                celle: gelata,
            },
            RigaMatrice {
                id: "vpd",
                titolo: "VPD stomatico",
                icona: "mat-vpd",
                regola: "Deficit di pressione di vapore: ottimale 0,4–1,2 kPa; sotto 0,4 condensa, oltre 1,6 kPa chiusura stomatica.",
                celle: vpd,
            },
        ],
    };

    let kpi = {
        let o = &ore[indice_ora];
        let thi_v = thi::thi(o.t, o.rh);
        Kpi {
            t: o.t,
            t_percepita: o.t_percepita,
            rh: o.rh,
            td: o.td,
            vpd: o.vpd,
            thi: thi_v,
            pioggia_24h: somma_indietro(&pioggia, indice_ora, 24),
            vento: o.vento,
            vento_dir: o.vento_dir,
            raffiche: o.raffiche,
            pressione: o.pressione,
            nuvolosita: o.nuvolosita,
            et0_oggi: giorni_meteo.iter().find(|g| g.data == oggi).map_or(f64::NAN, |g| g.et0),
            vpd_classe: water::classe_vpd(o.vpd),
            thi_classe: thi::classe_thi(thi_v),
        }
    };

    let giorni = giorni_agro(&giorni_meteo, &ore, &parametri, oggi);
    let gdd = somme_termiche(payload, &giorni_meteo, &parametri, oggi, &mut avvisi);

    let o = &ore[indice_ora];
    let denom_suolo = (parametri.suolo.capacita_campo - parametri.suolo.punto_appassimento).max(1e-6);
    let suolo = StatoSuolo {
        profondita_cm: [0, 6, 18, 54],
        temperature: o.suolo_t,
        strati_cm: [(0, 1), (1, 3), (3, 9), (9, 27), (27, 81)],
        umidita: o.suolo_u,
        frazione_disponibile: o.suolo_u.map(|u| (u - parametri.suolo.punto_appassimento) / denom_suolo),
        stati_strati: o.suolo_u.map(|u| frost_soil::stato_umidita(u, parametri.suolo)),
        semina: frost_soil::idoneita_semina(o.suolo_t[1], o.suolo_u[2], parametri.suolo),
    };

    let allerte = allerte(&matrice);
    let schede_almanacco = almanacco::tutte_le_schede();
    let almanacco = almanacco::analizza_report(
        ora_corrente,
        indice_ora,
        &ore,
        &giorni,
        &suolo,
        &kpi,
        &parametri,
        &schede_almanacco,
    );

    Report {
        luogo: payload.luogo.clone(),
        quota_modello: prev.elevation,
        lat_modello: prev.latitude,
        lon_modello: prev.longitude,
        fonte,
        scaricato_il: payload.scaricato_il,
        generato_il: adesso,
        avvisi,
        ora_corrente,
        indice_ora,
        kpi,
        ore,
        matrice,
        giorni,
        gdd,
        suolo,
        allerte,
        almanacco,
        parametri,
    }
}

fn giorni_agro(giorni: &[Giorno], ore: &[Ora], p: &ParametriAgro, oggi: NaiveDate) -> Vec<GiornoAgro> {
    let mut cumulato = 0.0;
    giorni
        .iter()
        .map(|g| {
            let del_giorno: Vec<&Ora> = ore.iter().filter(|o| o.time.date() == g.data).collect();
            let t6 = media(&del_giorno.iter().map(|o| o.suolo_t[1]).collect::<Vec<_>>());
            let u39 = media(&del_giorno.iter().map(|o| o.suolo_u[2]).collect::<Vec<_>>());
            let radiazione_wh: f64 = del_giorno.iter().map(|o| o.radiazione).filter(|v| v.is_finite()).sum();
            let pioggia_efficace = water::pioggia_efficace(g.pioggia, p.pioggia);
            let delta_w = water::delta_w(pioggia_efficace, g.et0, p.kc);
            cumulato += delta_w;
            GiornoAgro {
                meteo: g.clone(),
                pioggia_efficace,
                etc: g.et0 * p.kc,
                delta_w,
                bilancio_cumulato: cumulato,
                gdd_vite: gdd::gradi_giorno(g.t_max, g.t_min, T_BASE_VITE),
                gdd_foraggere: gdd::gradi_giorno(g.t_max, g.t_min, T_BASE_FORAGGERE),
                fv_kwh_kwp: radiazione_wh / 1000.0 * 0.8,
                semina: frost_soil::idoneita_semina(t6, u39, p.suolo),
                previsto: g.data > oggi,
            }
        })
        .collect()
}

/// Unisce storico e previsione (la previsione prevale sulle date comuni)
/// e calcola i GDD stagionali per vite e foraggere.
fn somme_termiche(
    payload: &Payload,
    giorni_meteo: &[Giorno],
    p: &ParametriAgro,
    oggi: NaiveDate,
    avvisi: &mut Vec<String>,
) -> Vec<SommaTermica> {
    let mut serie: std::collections::BTreeMap<NaiveDate, (f64, f64)> = Default::default();
    if let Some(s) = &payload.storico {
        for (i, d) in s.daily.time.iter().enumerate() {
            let (Some(d), Some(Some(tmax)), Some(Some(tmin))) =
                (parse_data(d), s.daily.temperature_2m_max.get(i), s.daily.temperature_2m_min.get(i))
            else {
                continue;
            };
            serie.insert(d, (*tmax, *tmin));
        }
    }
    for g in giorni_meteo.iter().filter(|g| g.t_max.is_finite() && g.t_min.is_finite()) {
        serie.insert(g.data, (g.t_max, g.t_min));
    }
    let giorni: Vec<(NaiveDate, f64, f64)> = serie.into_iter().map(|(d, (a, b))| (d, a, b)).collect();
    let primo = giorni.first().map(|g| g.0);

    let mut calcola = |coltura: &'static str, t_base: f64, inizio: NaiveDate| {
        let serie = gdd::cumulato(&giorni, t_base, inizio, oggi);
        let parziale = primo.is_none_or(|d| d > inizio);
        if parziale {
            avvisi.push(format!(
                "GDD {coltura}: storico non disponibile dal {inizio}, il cumulato è parziale (dal {}).",
                primo.map_or("-".into(), |d| d.to_string())
            ));
        }
        SommaTermica {
            coltura,
            t_base,
            inizio,
            totale_oggi: serie.iter().rfind(|g| !g.previsto).map_or(0.0, |g| g.cumulato),
            totale_previsto: serie.last().map_or(0.0, |g| g.cumulato),
            parziale,
            serie,
        }
    };
    vec![
        calcola("Vite", T_BASE_VITE, p.inizio_vite.unwrap_or(oggi)),
        calcola("Erbai e foraggere", T_BASE_FORAGGERE, p.inizio_foraggere.unwrap_or(oggi)),
    ]
}

/// Episodi contigui di celle che soddisfano `pred`: (inizio, fine) inclusivi.
fn episodi(celle: &[Valutazione], pred: impl Fn(&Valutazione) -> bool) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut inizio = None;
    for (i, c) in celle.iter().enumerate() {
        match (pred(c), inizio) {
            (true, None) => inizio = Some(i),
            (false, Some(a)) => {
                out.push((a, i - 1));
                inizio = None;
            }
            _ => {}
        }
    }
    if let Some(a) = inizio {
        out.push((a, celle.len() - 1));
    }
    out
}

/// Sintesi dei primi eventi rilevanti nell'orizzonte della matrice.
fn allerte(m: &Matrice) -> Vec<Allerta> {
    let riga = |id: &str| m.righe.iter().find(|r| r.id == id).expect("riga matrice");
    let mut out = Vec::new();
    let mut aggiungi = |id: &'static str, titolo: &str, pred: &dyn Fn(&Valutazione) -> bool, min_ore: usize| {
        let r = riga(id);
        let ep: Vec<_> = episodi(&r.celle, pred).into_iter().filter(|(a, b)| b - a + 1 >= min_ore).collect();
        if let Some(&(a, b)) = ep.first() {
            let peggiore = r.celle[a..=b]
                .iter()
                .max_by_key(|c| match c.livello {
                    Semaforo::Rosso => 3,
                    Semaforo::Arancio => 2,
                    Semaforo::Giallo => 1,
                    _ => 0,
                })
                .expect("episodio non vuoto");
            out.push(Allerta {
                indice: id,
                titolo: titolo.to_string(),
                livello: peggiore.livello,
                inizio: m.ore[a],
                fine: m.ore[b],
                ore: b - a + 1,
                episodi: ep.len(),
                motivo: peggiore.motivo.clone(),
            });
        }
    };
    let rosso = |c: &Valutazione| c.livello == Semaforo::Rosso;
    aggiungi("gelata", "Rischio gelata", &rosso, 1);
    aggiungi("peronospora_primaria", "Peronospora: infezione primaria", &rosso, 1);
    aggiungi("peronospora_secondaria", "Peronospora: infezioni secondarie", &rosso, 1);
    aggiungi("oidio", "Oidio: rischio alto", &rosso, 1);
    aggiungi("thi", "Stress da caldo ovini/caprini", &|c| matches!(c.livello, Semaforo::Arancio | Semaforo::Rosso), 1);
    aggiungi("vpd", "Stress idrico/stomatico (VPD)", &rosso, 2);
    aggiungi("trattamenti", "Prossima finestra di trattamento ottimale", &|c| c.livello == Semaforo::Verde, 2);
    if !out.iter().any(|a| a.indice == "trattamenti") && !m.ore.is_empty() {
        out.push(Allerta {
            indice: "trattamenti",
            titolo: "Nessuna finestra di trattamento ottimale (≥ 2 h)".into(),
            livello: Semaforo::Giallo,
            inizio: m.ore[0],
            fine: *m.ore.last().expect("ore"),
            ore: 0,
            episodi: 0,
            motivo: format!("Nelle prossime {} ore non ci sono 2 ore consecutive ottimali.", m.ore.len()),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload() -> Payload {
        Payload {
            scaricato_il: Utc::now(),
            luogo: Luogo { nome: "Test".into(), lat: 39.79, lon: 9.08 },
            previsione: serde_json::from_str(include_str!("../tests/fixtures/open_meteo_seulo.json")).unwrap(),
            storico: None,
            storico_da: None,
        }
    }

    #[test]
    fn date_stagione() {
        let d = |m, g| NaiveDate::from_ymd_opt(2026, m, g).unwrap();
        assert_eq!(inizio_stagione(d(11, 1), 4, 1), d(4, 1));
        assert_eq!(inizio_stagione(d(3, 1), 4, 1), NaiveDate::from_ymd_opt(2025, 4, 1).unwrap());
        assert_eq!(inizio_stagione(d(10, 1), 10, 1), d(10, 1));
    }

    #[test]
    fn report_da_fixture() {
        let p = payload();
        // La fixture parte il 2026-10-25 00:00 locale (UTC+1): "adesso" = 1 novembre, 12:30 locali.
        let adesso = DateTime::parse_from_rfc3339("2026-11-01T11:30:00Z").unwrap().with_timezone(&Utc);
        let params = ParametriAgro { germogli_cm: 15.0, ..Default::default() };
        let r = costruisci(&p, &params, adesso, Fonte::Rete, vec![]);

        assert_eq!(r.ora_corrente.format("%Y-%m-%d %H:%M").to_string(), "2026-11-01 12:30");
        assert_eq!(r.ore[r.indice_ora].time.format("%H:%M").to_string(), "12:00");
        assert_eq!(r.matrice.ore.len(), 72);
        assert!(r.matrice.righe.iter().all(|riga| riga.celle.len() == 72));
        assert_eq!(r.giorni.len(), 14);
        assert!(r.giorni.iter().filter(|g| g.previsto).count() == 6);
        assert!(r.kpi.t.is_finite() && r.kpi.thi.is_finite());
        // senza storico il cumulato GDD della vite è parziale
        assert!(r.gdd[0].parziale);
        assert!(r.gdd[1].totale_previsto >= r.gdd[1].totale_oggi);
        assert!(r.allerte.iter().any(|a| a.indice == "trattamenti"));
        // vite in vegetazione: gli indici fitopatologici sono attivi
        assert!(r.matrice.righe[4].celle.iter().all(|c| c.etichetta != "n.a."));
    }

    #[test]
    fn report_vite_in_riposo() {
        let adesso = DateTime::parse_from_rfc3339("2026-11-01T11:30:00Z").unwrap().with_timezone(&Utc);
        let r = costruisci(&payload(), &ParametriAgro::default(), adesso, Fonte::Cache, vec![]);
        let oidio = r.matrice.righe.iter().find(|r| r.id == "oidio").unwrap();
        assert!(oidio.celle.iter().all(|c| c.livello == Semaforo::Inattivo));
    }

    #[test]
    fn episodi_contigui() {
        let v = |l| Valutazione::new(l, "", "");
        let c = [v(Semaforo::Rosso), v(Semaforo::Verde), v(Semaforo::Rosso), v(Semaforo::Rosso)];
        assert_eq!(episodi(&c, |c| c.livello == Semaforo::Rosso), vec![(0, 0), (2, 3)]);
    }
}
