//! 3.5 Bilancio idrico semplificato e stress stomatico (VPD).

use super::{Semaforo, Valutazione};
use serde::{Deserialize, Serialize};

/// Parametri della pioggia efficace: sotto `soglia_mm` la pioggia è
/// considerata intercettata/evaporata; oltre, ne infiltra la frazione `efficienza`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ParametriPioggia {
    pub soglia_mm: f64,
    pub efficienza: f64,
}

impl Default for ParametriPioggia {
    fn default() -> Self {
        Self { soglia_mm: 2.0, efficienza: 0.8 }
    }
}

pub fn pioggia_efficace(pioggia_mm: f64, p: ParametriPioggia) -> f64 {
    if pioggia_mm < p.soglia_mm { 0.0 } else { pioggia_mm * p.efficienza }
}

/// ΔW = Pioggia_effettiva − ET0 × Kc (mm/giorno).
pub fn delta_w(pioggia_eff: f64, et0: f64, kc: f64) -> f64 {
    pioggia_eff - et0 * kc
}

/// Classi VPD (kPa). La fascia 1.2–1.6 kPa, non coperta dalla specifica,
/// è riportata come transizione verso lo stress.
pub fn classe_vpd(vpd: f64) -> Valutazione {
    let v = format!("VPD {vpd:.2} kPa");
    if vpd < 0.4 {
        Valutazione::new(Semaforo::Giallo, "Basso", format!("{v}: rischio condensa e patogeni, traspirazione bloccata"))
    } else if vpd <= 1.2 {
        Valutazione::new(Semaforo::Verde, "Ottimale", format!("{v}: traspirazione e fotosintesi ottimali"))
    } else if vpd <= 1.6 {
        Valutazione::new(Semaforo::Arancio, "Elevato", format!("{v}: transizione verso lo stress"))
    } else {
        Valutazione::new(Semaforo::Rosso, "Stress", format!("{v}: stress idrico, chiusura stomatica"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bilancio() {
        let p = ParametriPioggia::default();
        assert_eq!(pioggia_efficace(1.5, p), 0.0);
        assert_eq!(pioggia_efficace(10.0, p), 8.0);
        assert!((delta_w(8.0, 4.0, 0.7) - 5.2).abs() < 1e-9);
    }

    #[test]
    fn soglie_vpd() {
        assert_eq!(classe_vpd(0.3).livello, Semaforo::Giallo);
        assert_eq!(classe_vpd(0.4).livello, Semaforo::Verde);
        assert_eq!(classe_vpd(1.2).livello, Semaforo::Verde);
        assert_eq!(classe_vpd(1.4).livello, Semaforo::Arancio);
        assert_eq!(classe_vpd(1.7).livello, Semaforo::Rosso);
    }
}
