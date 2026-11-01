//! 3.1 Stima della bagnatura fogliare (Leaf Wetness Duration).

use super::PIOGGIA_ORARIA_MM;

pub const RH_BAGNATURA: f64 = 87.0;
pub const DELTA_RUGIADA_MAX: f64 = 1.5;

/// `foglia_bagnata` se piove (> 0.1 mm), se RH ≥ 87 % oppure se lo scarto
/// temperatura-rugiada è ≤ 1.5 °C (condensazione attiva sulla lamina).
pub fn foglia_bagnata(pioggia_mm: f64, rh: f64, t_aria: f64, t_rugiada: f64) -> bool {
    pioggia_mm > PIOGGIA_ORARIA_MM || rh >= RH_BAGNATURA || (t_aria - t_rugiada) <= DELTA_RUGIADA_MAX
}

/// Per ogni ora, numero di ore consecutive di bagnatura terminate in quell'ora
/// (0 se la foglia è asciutta).
pub fn ore_consecutive(bagnata: &[bool]) -> Vec<u32> {
    let mut run = 0;
    bagnata
        .iter()
        .map(|&b| {
            run = if b { run + 1 } else { 0 };
            run
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condizioni_singole() {
        assert!(foglia_bagnata(0.2, 50.0, 20.0, 5.0), "pioggia");
        assert!(foglia_bagnata(0.0, 87.0, 20.0, 5.0), "umidità");
        assert!(foglia_bagnata(0.0, 60.0, 10.0, 8.5), "rugiada");
        assert!(!foglia_bagnata(0.1, 86.9, 20.0, 18.4), "tutte sotto soglia");
    }

    #[test]
    fn conteggio_ore_consecutive() {
        let b = [true, true, false, true, true, true];
        assert_eq!(ore_consecutive(&b), vec![1, 2, 0, 1, 2, 3]);
    }
}
