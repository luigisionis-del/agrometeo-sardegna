//! Data ingestion layer: client HTTP asincrono verso fonti Open Data senza chiavi.

use std::time::Duration;

use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::model::{DAILY_VARS, HOURLY_VARS, OpenMeteoForecast, OpenMeteoHistory};

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const HISTORY_URL: &str = "https://historical-forecast-api.open-meteo.com/v1/forecast";
const GEOCODING_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const RAINVIEWER_URL: &str = "https://api.rainviewer.com/public/weather-maps.json";
const TIMEZONE: &str = "Europe/Rome";

/// Giorni passati inclusi nel payload: coprono la pioggia a 48 h e il
/// bilancio idrico della settimana trascorsa.
pub const GIORNI_PASSATI: u32 = 7;
pub const GIORNI_PREVISIONE: u32 = 7;

/// Zoom massimo servito dai tile RainViewer pubblici: oltre il 7 restituiscono
/// un'immagine "zoom non supportato", quindi Leaflet deve ingrandire il livello 7.
pub const RADAR_MAX_ZOOM: u8 = 7;

/// Rettangolo di riferimento della Sardegna.
pub const SARDEGNA_LAT: (f64, f64) = (38.8, 41.35);
pub const SARDEGNA_LON: (f64, f64) = (8.05, 9.9);

pub fn client() -> reqwest::Client {
    let builder = reqwest::Client::builder()
        .user_agent(concat!("AgrometeoSardegna/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(25))
        .connect_timeout(Duration::from_secs(8));
    // Su Android il verificatore di piattaforma va inizializzato via JNI, altrimenti la prima
    // richiesta HTTPS termina l'app: si verificano i certificati con le radici Mozilla incluse.
    #[cfg(target_os = "android")]
    let builder = builder.tls_certs_only(
        webpki_root_certs::TLS_SERVER_ROOT_CERTS
            .iter()
            .filter_map(|c| reqwest::Certificate::from_der(c).ok()),
    );
    builder.build().expect("configurazione client HTTP")
}

async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    fonte: &'static str,
    url: &str,
    query: &[(&str, String)],
) -> Result<T> {
    let resp = client.get(url).query(query).send().await?;
    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        // Open-Meteo risponde {"error": true, "reason": "..."}
        let motivo = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v.get("reason").and_then(|r| r.as_str()).map(str::to_owned))
            .unwrap_or_else(|| body.chars().take(200).collect());
        return Err(AppError::Formato { fonte, dettaglio: format!("HTTP {status}: {motivo}") });
    }
    serde_json::from_str(&body).map_err(|e| AppError::Formato { fonte, dettaglio: e.to_string() })
}

fn coord(v: f64) -> String {
    format!("{v:.4}")
}

/// Livello 1: modello numerico suolo/aria/bilancio idrico.
pub async fn previsione(client: &reqwest::Client, lat: f64, lon: f64) -> Result<OpenMeteoForecast> {
    let query = [
        ("latitude", coord(lat)),
        ("longitude", coord(lon)),
        ("timezone", TIMEZONE.to_string()),
        ("past_days", GIORNI_PASSATI.to_string()),
        ("forecast_days", GIORNI_PREVISIONE.to_string()),
        ("hourly", HOURLY_VARS.join(",")),
        ("daily", DAILY_VARS.join(",")),
    ];
    get_json(client, "Open-Meteo", FORECAST_URL, &query).await
}

/// Serie giornaliera storica dei modelli (Historical Forecast API).
pub async fn storico_giornaliero(
    client: &reqwest::Client,
    lat: f64,
    lon: f64,
    da: NaiveDate,
    a: NaiveDate,
) -> Result<OpenMeteoHistory> {
    let query = [
        ("latitude", coord(lat)),
        ("longitude", coord(lon)),
        ("timezone", TIMEZONE.to_string()),
        ("start_date", da.to_string()),
        ("end_date", a.to_string()),
        ("daily", "temperature_2m_max,temperature_2m_min,precipitation_sum".to_string()),
    ];
    get_json(client, "Open-Meteo storico", HISTORY_URL, &query).await
}

#[derive(Debug, Clone, Serialize)]
pub struct Localita {
    pub nome: String,
    pub provincia: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub quota: Option<f64>,
}

#[derive(Deserialize)]
struct GeoRisposta {
    #[serde(default)]
    results: Vec<GeoRisultato>,
}

#[derive(Deserialize)]
struct GeoRisultato {
    name: String,
    latitude: f64,
    longitude: f64,
    elevation: Option<f64>,
    admin1: Option<String>,
    admin2: Option<String>,
}

pub fn in_sardegna(lat: f64, lon: f64) -> bool {
    (SARDEGNA_LAT.0..=SARDEGNA_LAT.1).contains(&lat) && (SARDEGNA_LON.0..=SARDEGNA_LON.1).contains(&lon)
}

/// Ricerca comuni e località sarde (Open-Meteo Geocoding, dati GeoNames).
pub async fn cerca_localita(client: &reqwest::Client, testo: &str) -> Result<Vec<Localita>> {
    let query = [
        ("name", testo.to_string()),
        ("count", "30".to_string()),
        ("language", "it".to_string()),
        ("countryCode", "IT".to_string()),
    ];
    let r: GeoRisposta = get_json(client, "Geocoding", GEOCODING_URL, &query).await?;
    Ok(r.results
        .into_iter()
        .filter(|g| g.admin1.as_deref() == Some("Sardegna") || in_sardegna(g.latitude, g.longitude))
        .map(|g| Localita { nome: g.name, provincia: g.admin2, lat: g.latitude, lon: g.longitude, quota: g.elevation })
        .collect())
}

/// Livello 2: fotogrammi radar RainViewer.
#[derive(Debug, Clone, Serialize)]
pub struct Radar {
    pub host: String,
    pub fotogrammi: Vec<FotogrammaRadar>,
    pub zoom_max: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FotogrammaRadar {
    pub time: i64,
    pub path: String,
    #[serde(default)]
    pub nowcast: bool,
}

#[derive(Deserialize)]
struct RainViewerMaps {
    host: String,
    radar: RainViewerRadar,
}

#[derive(Deserialize)]
struct RainViewerRadar {
    #[serde(default)]
    past: Vec<FotogrammaRadar>,
    #[serde(default)]
    nowcast: Vec<FotogrammaRadar>,
}

pub async fn radar(client: &reqwest::Client) -> Result<Radar> {
    let maps: RainViewerMaps = get_json(client, "RainViewer", RAINVIEWER_URL, &[]).await?;
    let mut fotogrammi = maps.radar.past;
    // Il nowcast non è più fornito dall'API pubblica, ma se torna disponibile lo usiamo.
    fotogrammi.extend(maps.radar.nowcast.into_iter().map(|f| FotogrammaRadar { nowcast: true, ..f }));
    Ok(Radar { host: maps.host, fotogrammi, zoom_max: RADAR_MAX_ZOOM })
}

/// Vento attuale su una griglia regolare sopra l'isola.
#[derive(Debug, Clone, Serialize)]
pub struct PuntoVento {
    pub lat: f64,
    pub lon: f64,
    pub velocita: f64,
    pub direzione: f64,
    pub raffiche: f64,
}

#[derive(Deserialize)]
struct VentoRisposta {
    latitude: f64,
    longitude: f64,
    current: VentoCorrente,
}

#[derive(Deserialize)]
struct VentoCorrente {
    wind_speed_10m: Option<f64>,
    wind_direction_10m: Option<f64>,
    wind_gusts_10m: Option<f64>,
}

pub fn griglia_vento() -> Vec<(f64, f64)> {
    let lats = [39.0, 39.45, 39.9, 40.35, 40.8, 41.2];
    let lons = [8.4, 8.85, 9.3, 9.7];
    lats.iter().flat_map(|&la| lons.iter().map(move |&lo| (la, lo))).collect()
}

pub async fn vento_griglia(client: &reqwest::Client) -> Result<Vec<PuntoVento>> {
    let punti = griglia_vento();
    let lat: Vec<String> = punti.iter().map(|p| coord(p.0)).collect();
    let lon: Vec<String> = punti.iter().map(|p| coord(p.1)).collect();
    let query = [
        ("latitude", lat.join(",")),
        ("longitude", lon.join(",")),
        ("timezone", TIMEZONE.to_string()),
        ("current", "wind_speed_10m,wind_direction_10m,wind_gusts_10m".to_string()),
    ];
    let r: Vec<VentoRisposta> = get_json(client, "Open-Meteo vento", FORECAST_URL, &query).await?;
    Ok(r.into_iter()
        .filter_map(|v| {
            Some(PuntoVento {
                lat: v.latitude,
                lon: v.longitude,
                velocita: v.current.wind_speed_10m?,
                direzione: v.current.wind_direction_10m?,
                raffiche: v.current.wind_gusts_10m.unwrap_or(f64::NAN),
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test di rete: `cargo test -- --ignored`.
    #[tokio::test]
    #[ignore]
    async fn api_reali() {
        let c = client();
        let p = previsione(&c, 39.79, 9.08).await.unwrap();
        assert_eq!(p.hourly.time.len(), ((GIORNI_PASSATI + GIORNI_PREVISIONE) * 24) as usize);
        assert!(!radar(&c).await.unwrap().fotogrammi.is_empty());
        assert!(cerca_localita(&c, "Seulo").await.unwrap().iter().any(|l| l.nome == "Seulo"));
        assert_eq!(vento_griglia(&c).await.unwrap().len(), griglia_vento().len());
    }
}
