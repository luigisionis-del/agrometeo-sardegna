//! 3.6 Indice bioclimatico zootecnico (Temperature Humidity Index).

use super::{Semaforo, Valutazione};

/// THI = (1.8 T + 32) − [(0.55 − 0.0055 RH) × (1.8 T − 26)]
pub fn thi(t_aria: f64, rh: f64) -> f64 {
    (1.8 * t_aria + 32.0) - (0.55 - 0.0055 * rh) * (1.8 * t_aria - 26.0)
}

pub fn classe_thi(valore: f64) -> Valutazione {
    let v = format!("THI {valore:.1}");
    if valore < 68.0 {
        Valutazione::new(Semaforo::Verde, "Benessere", v)
    } else if valore < 72.0 {
        Valutazione::new(Semaforo::Giallo, "Lieve", format!("{v}: disagio lieve"))
    } else if valore < 78.0 {
        Valutazione::new(
            Semaforo::Arancio,
            "Moderato",
            format!("{v}: stress moderato, pascolo notturno/mattutino e ombreggiamento"),
        )
    } else {
        Valutazione::new(Semaforo::Rosso, "Severo", format!("{v}: stress severo, emergenza termica"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formula() {
        // 30 °C, 50 % → 86 − 0.275 × 28 = 78.3
        assert!((thi(30.0, 50.0) - 78.3).abs() < 1e-9);
        // 20 °C, 60 % → 68 − 0.22 × 10 = 65.8
        assert!((thi(20.0, 60.0) - 65.8).abs() < 1e-9);
    }

    #[test]
    fn soglie() {
        assert_eq!(classe_thi(67.9).livello, Semaforo::Verde);
        assert_eq!(classe_thi(68.0).livello, Semaforo::Giallo);
        assert_eq!(classe_thi(72.0).livello, Semaforo::Arancio);
        assert_eq!(classe_thi(78.0).livello, Semaforo::Rosso);
    }
}
