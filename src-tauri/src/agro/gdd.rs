//! 3.4 Somma termica e gradi giorno (Growing Degree Days).

use chrono::NaiveDate;
use serde::Serialize;

pub const T_BASE_VITE: f64 = 10.0;
pub const T_BASE_FORAGGERE: f64 = 5.0;

/// GDD = max(0, (Tmax + Tmin) / 2 − Tbase).
pub fn gradi_giorno(t_max: f64, t_min: f64, t_base: f64) -> f64 {
    ((t_max + t_min) / 2.0 - t_base).max(0.0)
}

#[derive(Debug, Clone, Serialize)]
pub struct GiornoGdd {
    pub data: NaiveDate,
    pub gdd: f64,
    pub cumulato: f64,
    /// Giorno di previsione (non ancora osservato).
    pub previsto: bool,
}

/// GDD giornalieri e cumulati a partire da `inizio` (incluso).
/// `giorni` deve essere ordinato per data: `(data, tmax, tmin)`.
pub fn cumulato(giorni: &[(NaiveDate, f64, f64)], t_base: f64, inizio: NaiveDate, oggi: NaiveDate) -> Vec<GiornoGdd> {
    let mut cum = 0.0;
    giorni
        .iter()
        .filter(|(d, tmax, tmin)| *d >= inizio && tmax.is_finite() && tmin.is_finite())
        .map(|&(data, tmax, tmin)| {
            let gdd = gradi_giorno(tmax, tmin, t_base);
            cum += gdd;
            GiornoGdd { data, gdd, cumulato: cum, previsto: data > oggi }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formula() {
        assert_eq!(gradi_giorno(25.0, 15.0, T_BASE_VITE), 10.0);
        assert_eq!(gradi_giorno(12.0, 4.0, T_BASE_VITE), 0.0);
        assert_eq!(gradi_giorno(12.0, 4.0, T_BASE_FORAGGERE), 3.0);
    }

    #[test]
    fn cumulo_da_data_inizio() {
        let d = |g| NaiveDate::from_ymd_opt(2026, 4, g).unwrap();
        let giorni = [(d(1), 20.0, 10.0), (d(2), 22.0, 12.0), (d(3), 24.0, 14.0)];
        let r = cumulato(&giorni, T_BASE_VITE, d(2), d(2));
        assert_eq!(r.len(), 2);
        assert_eq!(r[1].cumulato, 7.0 + 9.0);
        assert!(!r[0].previsto && r[1].previsto);
    }
}
