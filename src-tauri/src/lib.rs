//! Agrometeo Sardegna: backend Tauri.
//!
//! La logica di dominio vive nel crate `agrometeo-core` (calcolo agronomico,
//! almanacco, fetch, cache); qui restano solo i comandi Tauri che il frontend
//! (`dist/`, solo rendering e visualizzazione SVG) invoca.

pub use agrometeo_core::{agro, almanacco, error, fetch, illustrazioni, model, report, stations, storage};

use std::sync::Mutex;
use std::time::{Duration, Instant};

use chrono::{Duration as ChronoDuration, Timelike, Utc};
use tauri::{Manager, State};

use almanacco::CatalogoAlmanacco;
use error::{AppError, Result};
use fetch::{Localita, PuntoVento, Radar};
use model::Luogo;
use report::{Fonte, ParametriAgro, Payload, Report};
use stations::{Confronto, CsvStazione, FonteOsservazioni};
use storage::{Archivio, SpazioLavoro};

/// Sotto questa età la cache viene usata senza interrogare la rete:
/// i modelli si aggiornano al più ogni ora.
const CACHE_FRESCA: ChronoDuration = ChronoDuration::minutes(15);
const VENTO_FRESCO: Duration = Duration::from_secs(20 * 60);
/// Limite di giorni per un confronto con la stazione.
const MAX_GIORNI_CONFRONTO: i64 = 400;

struct Stato {
    client: fetch::Client,
    archivio: Archivio,
    vento: Mutex<Option<(Instant, Vec<PuntoVento>)>>,
}

fn valida_coordinate(lat: f64, lon: f64) -> Result<()> {
    if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
        Ok(())
    } else {
        Err(AppError::Input(format!("Coordinate non valide: {lat}, {lon}")))
    }
}

#[tauri::command]
async fn report(stato: State<'_, Stato>, luogo: Luogo, parametri: ParametriAgro, aggiorna: bool) -> Result<Report> {
    valida_coordinate(luogo.lat, luogo.lon)?;
    let adesso = Utc::now().with_nanosecond(0).unwrap_or_else(Utc::now);
    let mut parametri_risolti = parametri.clone();
    parametri_risolti.risolvi_date(report::ora_locale(adesso, 3600).date());
    let serve_storico_da = parametri_risolti.inizio_storico();

    let cache = stato.archivio.leggi_payload(luogo.lat, luogo.lon);
    let storico_coperto = |p: &Payload| match (serve_storico_da, p.storico_da, &p.storico) {
        (None, _, _) => true,
        (Some(serve), Some(da), Some(_)) => da <= serve,
        _ => false,
    };

    if let Some(p) = &cache
        && !aggiorna
        && adesso - p.scaricato_il < CACHE_FRESCA
        && storico_coperto(p)
    {
        let p = Payload { luogo: luogo.clone(), ..p.clone() };
        return Ok(report::costruisci(&p, &parametri, adesso, Fonte::Rete, vec![]));
    }

    let mut avvisi = Vec::new();
    let previsione = match fetch::previsione(&stato.client, luogo.lat, luogo.lon).await {
        Ok(p) => p,
        Err(e) => {
            let Some(p) = cache else {
                return Err(AppError::NessunDato);
            };
            avvisi.push(format!("Connessione non disponibile, dati dalla cache. ({e})"));
            let p = Payload { luogo, ..p };
            return Ok(report::costruisci(&p, &parametri, adesso, Fonte::Cache, avvisi));
        }
    };

    let (mut storico, mut storico_da) = cache.map(|c| (c.storico, c.storico_da)).unwrap_or_default();
    if let Some(da) = serve_storico_da {
        let oggi = report::ora_locale(adesso, previsione.utc_offset_seconds).date();
        match fetch::storico_giornaliero(&stato.client, luogo.lat, luogo.lon, da, oggi).await {
            Ok(s) => {
                storico = Some(s);
                storico_da = Some(da);
            }
            Err(e) => avvisi.push(format!("Storico stagionale non aggiornato: {e}")),
        }
    }

    let payload = Payload { scaricato_il: adesso, luogo, previsione, storico, storico_da };
    if let Err(e) = stato.archivio.salva_payload(&payload) {
        avvisi.push(format!("Impossibile salvare la cache offline: {e}"));
    }
    Ok(report::costruisci(&payload, &parametri, adesso, Fonte::Rete, avvisi))
}

#[tauri::command]
async fn radar(stato: State<'_, Stato>) -> Result<Radar> {
    fetch::radar(&stato.client).await
}

#[tauri::command]
async fn vento(stato: State<'_, Stato>) -> Result<Vec<PuntoVento>> {
    if let Some((quando, punti)) = stato.vento.lock().expect("mutex vento").as_ref()
        && quando.elapsed() < VENTO_FRESCO
    {
        return Ok(punti.clone());
    }
    let punti = fetch::vento_griglia(&stato.client).await?;
    *stato.vento.lock().expect("mutex vento") = Some((Instant::now(), punti.clone()));
    Ok(punti)
}

#[tauri::command]
async fn cerca_localita(stato: State<'_, Stato>, testo: String) -> Result<Vec<Localita>> {
    let testo = testo.trim();
    if testo.chars().count() < 2 {
        return Ok(vec![]);
    }
    fetch::cerca_localita(&stato.client, testo).await
}

#[tauri::command]
fn spazio_lavoro(stato: State<'_, Stato>) -> SpazioLavoro {
    stato.archivio.leggi_spazio()
}

#[tauri::command]
fn salva_spazio_lavoro(stato: State<'_, Stato>, spazio: SpazioLavoro) -> Result<()> {
    stato.archivio.salva_spazio(&spazio)
}

#[tauri::command]
fn catalogo_almanacco() -> CatalogoAlmanacco {
    almanacco::catalogo()
}

#[tauri::command]
fn csv_esempio_stazione() -> String {
    let oggi = report::ora_locale(Utc::now(), 3600).date();
    stations::genera_csv_esempio(oggi)
}

#[tauri::command]
async fn confronta_stazione(stato: State<'_, Stato>, nome: String, csv: String, lat: f64, lon: f64) -> Result<Confronto> {
    valida_coordinate(lat, lon)?;
    let fonte = CsvStazione { nome, testo: &csv };
    let osservati = fonte.pioggia_giornaliera()?;
    let oggi = report::ora_locale(Utc::now(), 3600).date();
    let a = osservati.last().expect("almeno un giorno").data.min(oggi);
    let da = osservati[0].data.max(a - ChronoDuration::days(MAX_GIORNI_CONFRONTO));
    if da > a {
        return Err(AppError::Input("Le date del CSV sono tutte nel futuro".into()));
    }
    let modello = fetch::storico_giornaliero(&stato.client, lat, lon, da, a).await?;
    let nel_periodo: Vec<_> = osservati.into_iter().filter(|o| o.data >= da && o.data <= a).collect();
    Ok(stations::confronta(fonte.nome(), &nel_periodo, &modello))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            app.manage(Stato { client: fetch::client(), archivio: Archivio::new(dir)?, vento: Mutex::new(None) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            report,
            radar,
            vento,
            cerca_localita,
            spazio_lavoro,
            salva_spazio_lavoro,
            catalogo_almanacco,
            csv_esempio_stazione,
            confronta_stazione
        ])
        .run(tauri::generate_context!())
        .expect("errore all'avvio di Agrometeo Sardegna");
}
