//! Motore algoritmico agronomico e zootecnico (sezione 3 della specifica).
//!
//! Tutte le funzioni di questo modulo sono pure: ricevono valori o serie
//! orarie già normalizzate e restituiscono indicatori, senza I/O. Questo le
//! rende testabili in isolamento e riutilizzabili (es. compilazione WASM).

pub mod disease;
pub mod frost_soil;
pub mod gdd;
pub mod spray;
pub mod thi;
pub mod water;
pub mod wetness;

use serde::{Deserialize, Serialize};

/// Soglia oltre la quale un'ora è considerata piovosa (mm/h).
/// Sotto 0.1 mm il segnale dei modelli è sostanzialmente rumore.
pub const PIOGGIA_ORARIA_MM: f64 = 0.1;

/// Livello di un semaforo decisionale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Semaforo {
    Verde,
    Giallo,
    Arancio,
    Rosso,
    /// Indice non applicabile (es. vite in riposo) o patogeno inibito.
    Inattivo,
}

/// Valutazione di un indice in un'ora: livello, etichetta breve e motivazione.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Valutazione {
    pub livello: Semaforo,
    pub etichetta: String,
    pub motivo: String,
}

impl Valutazione {
    pub fn new(livello: Semaforo, etichetta: impl Into<String>, motivo: impl Into<String>) -> Self {
        Self { livello, etichetta: etichetta.into(), motivo: motivo.into() }
    }
}

/// Somma dei valori nella finestra `[i - (n-1), i]` (troncata all'inizio della serie).
pub fn somma_indietro(serie: &[f64], i: usize, n: usize) -> f64 {
    let da = (i + 1).saturating_sub(n);
    serie[da..=i].iter().filter(|v| v.is_finite()).sum()
}

/// Media dei valori nella finestra `[i - (n-1), i]` (troncata all'inizio della serie).
pub fn media_indietro(serie: &[f64], i: usize, n: usize) -> f64 {
    let da = (i + 1).saturating_sub(n);
    media(&serie[da..=i])
}

/// Media dei valori finiti di una fetta; NaN se vuota.
pub fn media(valori: &[f64]) -> f64 {
    let (s, n) = valori
        .iter()
        .filter(|v| v.is_finite())
        .fold((0.0, 0usize), |(s, n), v| (s + v, n + 1));
    if n == 0 { f64::NAN } else { s / n as f64 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finestre_troncate_all_inizio() {
        let s = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(somma_indietro(&s, 1, 24), 3.0);
        assert_eq!(somma_indietro(&s, 3, 2), 7.0);
        assert_eq!(media_indietro(&s, 3, 4), 2.5);
        assert!(media(&[]).is_nan());
    }
}
