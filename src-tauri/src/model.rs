//! Strutture dati: payload grezzi delle API e serie normalizzate.

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// Variabili orarie richieste a Open-Meteo (sezione 2.1).
pub const HOURLY_VARS: &[&str] = &[
    "temperature_2m",
    "relative_humidity_2m",
    "dew_point_2m",
    "apparent_temperature",
    "precipitation",
    "rain",
    "precipitation_probability",
    "wind_speed_10m",
    "wind_direction_10m",
    "wind_gusts_10m",
    "shortwave_radiation",
    "direct_normal_irradiance",
    "surface_pressure",
    "cloud_cover",
    "et0_fao_evapotranspiration",
    "vapour_pressure_deficit",
    "soil_temperature_0cm",
    "soil_temperature_6cm",
    "soil_temperature_18cm",
    "soil_temperature_54cm",
    "soil_moisture_0_to_1cm",
    "soil_moisture_1_to_3cm",
    "soil_moisture_3_to_9cm",
    "soil_moisture_9_to_27cm",
    "soil_moisture_27_to_81cm",
];

pub const DAILY_VARS: &[&str] = &[
    "temperature_2m_max",
    "temperature_2m_min",
    "precipitation_sum",
    "et0_fao_evapotranspiration",
    "sunshine_duration",
    "uv_index_max",
];

pub type Serie = Vec<Option<f64>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoForecast {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub utc_offset_seconds: i32,
    pub hourly: HourlyRaw,
    pub daily: DailyRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyRaw {
    pub time: Vec<String>,
    pub temperature_2m: Serie,
    pub relative_humidity_2m: Serie,
    pub dew_point_2m: Serie,
    pub apparent_temperature: Serie,
    pub precipitation: Serie,
    pub rain: Serie,
    pub precipitation_probability: Serie,
    pub wind_speed_10m: Serie,
    pub wind_direction_10m: Serie,
    pub wind_gusts_10m: Serie,
    pub shortwave_radiation: Serie,
    pub direct_normal_irradiance: Serie,
    pub surface_pressure: Serie,
    pub cloud_cover: Serie,
    pub et0_fao_evapotranspiration: Serie,
    pub vapour_pressure_deficit: Serie,
    pub soil_temperature_0cm: Serie,
    pub soil_temperature_6cm: Serie,
    pub soil_temperature_18cm: Serie,
    pub soil_temperature_54cm: Serie,
    pub soil_moisture_0_to_1cm: Serie,
    pub soil_moisture_1_to_3cm: Serie,
    pub soil_moisture_3_to_9cm: Serie,
    pub soil_moisture_9_to_27cm: Serie,
    pub soil_moisture_27_to_81cm: Serie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRaw {
    pub time: Vec<String>,
    pub temperature_2m_max: Serie,
    pub temperature_2m_min: Serie,
    pub precipitation_sum: Serie,
    pub et0_fao_evapotranspiration: Serie,
    pub sunshine_duration: Serie,
    pub uv_index_max: Serie,
}

/// Serie giornaliera storica (Historical Forecast API), usata per GDD
/// stagionali e per il confronto con le stazioni.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoHistory {
    pub daily: DailyHistoryRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyHistoryRaw {
    pub time: Vec<String>,
    #[serde(default)]
    pub temperature_2m_max: Serie,
    #[serde(default)]
    pub temperature_2m_min: Serie,
    #[serde(default)]
    pub precipitation_sum: Serie,
}

/// Un'ora di dati normalizzati (valori mancanti interpolati).
#[derive(Debug, Clone, Serialize)]
pub struct Ora {
    pub time: NaiveDateTime,
    pub t: f64,
    pub rh: f64,
    pub td: f64,
    pub t_percepita: f64,
    pub pioggia: f64,
    pub pioggia_liquida: f64,
    pub prob_pioggia: f64,
    pub vento: f64,
    pub vento_dir: f64,
    pub raffiche: f64,
    pub radiazione: f64,
    pub dni: f64,
    /// PAR stimata (µmol/m²/s) ≈ 2.02 × radiazione globale (W/m²).
    pub par: f64,
    pub pressione: f64,
    pub nuvolosita: f64,
    pub et0: f64,
    pub vpd: f64,
    /// Temperatura suolo a 0, 6, 18, 54 cm.
    pub suolo_t: [f64; 4],
    /// Umidità volumetrica 0–1, 1–3, 3–9, 9–27, 27–81 cm.
    pub suolo_u: [f64; 5],
}

#[derive(Debug, Clone, Serialize)]
pub struct Giorno {
    pub data: NaiveDate,
    pub t_max: f64,
    pub t_min: f64,
    pub pioggia: f64,
    pub et0: f64,
    pub ore_sole: f64,
    pub uv_max: f64,
}

/// Interpola linearmente i valori mancanti; agli estremi ripete il valore
/// valido più vicino. Una serie interamente vuota diventa NaN.
pub fn riempi_lacune(serie: &[Option<f64>]) -> Vec<f64> {
    let validi: Vec<(usize, f64)> = serie.iter().enumerate().filter_map(|(i, v)| v.map(|v| (i, v))).collect();
    if validi.is_empty() {
        return vec![f64::NAN; serie.len()];
    }
    let mut out = Vec::with_capacity(serie.len());
    let mut k = 0;
    for i in 0..serie.len() {
        while k + 1 < validi.len() && validi[k + 1].0 <= i {
            k += 1;
        }
        let (i0, v0) = validi[k];
        out.push(match validi.get(k + 1) {
            _ if i <= i0 => v0,
            Some(&(i1, v1)) => v0 + (v1 - v0) * (i - i0) as f64 / (i1 - i0) as f64,
            None => v0,
        });
    }
    out
}

pub fn parse_ora(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok()
}

pub fn parse_data(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

impl HourlyRaw {
    pub fn normalizza(&self) -> Vec<Ora> {
        let f = riempi_lacune;
        let t = f(&self.temperature_2m);
        let rh = f(&self.relative_humidity_2m);
        let td = f(&self.dew_point_2m);
        let tp = f(&self.apparent_temperature);
        let pioggia = f(&self.precipitation);
        let pl = f(&self.rain);
        let pp = f(&self.precipitation_probability);
        let vento = f(&self.wind_speed_10m);
        let vdir = f(&self.wind_direction_10m);
        let raff = f(&self.wind_gusts_10m);
        let sw = f(&self.shortwave_radiation);
        let dni = f(&self.direct_normal_irradiance);
        let pres = f(&self.surface_pressure);
        let nub = f(&self.cloud_cover);
        let et0 = f(&self.et0_fao_evapotranspiration);
        let vpd = f(&self.vapour_pressure_deficit);
        let st = [
            f(&self.soil_temperature_0cm),
            f(&self.soil_temperature_6cm),
            f(&self.soil_temperature_18cm),
            f(&self.soil_temperature_54cm),
        ];
        let su = [
            f(&self.soil_moisture_0_to_1cm),
            f(&self.soil_moisture_1_to_3cm),
            f(&self.soil_moisture_3_to_9cm),
            f(&self.soil_moisture_9_to_27cm),
            f(&self.soil_moisture_27_to_81cm),
        ];
        self.time
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                Some(Ora {
                    time: parse_ora(s)?,
                    t: t[i],
                    rh: rh[i],
                    td: td[i],
                    t_percepita: tp[i],
                    pioggia: pioggia[i].max(0.0),
                    pioggia_liquida: pl[i].max(0.0),
                    prob_pioggia: pp[i],
                    vento: vento[i],
                    vento_dir: vdir[i],
                    raffiche: raff[i],
                    radiazione: sw[i],
                    dni: dni[i],
                    par: sw[i] * 2.02,
                    pressione: pres[i],
                    nuvolosita: nub[i],
                    et0: et0[i],
                    vpd: vpd[i],
                    suolo_t: st.each_ref().map(|s| s[i]),
                    suolo_u: su.each_ref().map(|s| s[i]),
                })
            })
            .collect()
    }
}

impl DailyRaw {
    pub fn normalizza(&self) -> Vec<Giorno> {
        let v = |s: &Serie, i: usize| s.get(i).copied().flatten().unwrap_or(f64::NAN);
        self.time
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                Some(Giorno {
                    data: parse_data(s)?,
                    t_max: v(&self.temperature_2m_max, i),
                    t_min: v(&self.temperature_2m_min, i),
                    pioggia: v(&self.precipitation_sum, i),
                    et0: v(&self.et0_fao_evapotranspiration, i),
                    ore_sole: v(&self.sunshine_duration, i) / 3600.0,
                    uv_max: v(&self.uv_index_max, i),
                })
            })
            .collect()
    }
}

/// Località monitorata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Luogo {
    pub nome: String,
    pub lat: f64,
    pub lon: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolazione_lacune() {
        let s = [None, Some(1.0), None, None, Some(4.0), None];
        assert_eq!(riempi_lacune(&s), vec![1.0, 1.0, 2.0, 3.0, 4.0, 4.0]);
        assert!(riempi_lacune(&[None, None]).iter().all(|v| v.is_nan()));
    }

    #[test]
    fn fixture_open_meteo() {
        let raw: OpenMeteoForecast =
            serde_json::from_str(include_str!("../tests/fixtures/open_meteo_seulo.json")).unwrap();
        let ore = raw.hourly.normalizza();
        assert_eq!(ore.len(), 14 * 24);
        assert!(ore.iter().all(|o| o.t.is_finite() && o.suolo_u[4].is_finite()));
        assert_eq!(raw.daily.normalizza().len(), 14);
    }
}
