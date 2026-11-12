//! 3.3 Finestra ottimale per i trattamenti fitosanitari (Spray Window).

use super::{PIOGGIA_ORARIA_MM, Semaforo, Valutazione};

/// Condizioni orarie per valutare un trattamento.
#[derive(Debug, Clone, Copy)]
pub struct CondizioniTrattamento {
    pub vento_kmh: f64,
    pub raffiche_kmh: f64,
    pub t_aria: f64,
    pub rh: f64,
    /// L'ora corrente o una delle 4 successive è piovosa.
    pub pioggia_entro_4h: bool,
    /// L'ora corrente o una delle 6 successive è piovosa.
    pub pioggia_entro_6h: bool,
}

/// Verde = ottimale, giallo = cautela, rosso = sconsigliato/inefficace.
pub fn finestra_trattamento(c: CondizioniTrattamento) -> Valutazione {
    let mut divieti = Vec::new();
    if c.vento_kmh > 15.0 {
        divieti.push(format!("vento {:.0} km/h (deriva)", c.vento_kmh));
    }
    if c.t_aria > 28.0 {
        divieti.push(format!("T {:.1} °C (fitotossicità/evaporazione)", c.t_aria));
    }
    if c.pioggia_entro_4h {
        divieti.push("pioggia entro 4 h (dilavamento)".to_string());
    }
    if !divieti.is_empty() {
        return Valutazione::new(Semaforo::Rosso, "No", divieti.join(", "));
    }

    let mut limiti = Vec::new();
    if c.vento_kmh >= 10.0 {
        limiti.push(format!("vento {:.0} km/h", c.vento_kmh));
    }
    if c.raffiche_kmh >= 15.0 {
        limiti.push(format!("raffiche {:.0} km/h", c.raffiche_kmh));
    }
    if !(12.0..=25.0).contains(&c.t_aria) {
        limiti.push(format!("T {:.1} °C", c.t_aria));
    }
    if !(40.0..=80.0).contains(&c.rh) {
        limiti.push(format!("RH {:.0} %", c.rh));
    }
    if c.pioggia_entro_6h {
        limiti.push("pioggia entro 6 h".to_string());
    }
    if limiti.is_empty() {
        Valutazione::new(Semaforo::Verde, "Sì", "Condizioni ottimali")
    } else {
        Valutazione::new(Semaforo::Giallo, "Cautela", limiti.join(", "))
    }
}

/// Vero se nell'ora `i` o nelle `n` ore successive è prevista pioggia.
pub fn pioggia_nelle_prossime(pioggia: &[f64], i: usize, n: usize) -> bool {
    let a = (i + n + 1).min(pioggia.len());
    pioggia[i..a].iter().any(|&p| p > PIOGGIA_ORARIA_MM)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> CondizioniTrattamento {
        CondizioniTrattamento {
            vento_kmh: 5.0,
            raffiche_kmh: 10.0,
            t_aria: 18.0,
            rh: 60.0,
            pioggia_entro_4h: false,
            pioggia_entro_6h: false,
        }
    }

    #[test]
    fn ottimale() {
        assert_eq!(finestra_trattamento(base()).livello, Semaforo::Verde);
    }

    #[test]
    fn divieti() {
        assert_eq!(finestra_trattamento(CondizioniTrattamento { vento_kmh: 16.0, ..base() }).livello, Semaforo::Rosso);
        assert_eq!(finestra_trattamento(CondizioniTrattamento { t_aria: 29.0, ..base() }).livello, Semaforo::Rosso);
        assert_eq!(
            finestra_trattamento(CondizioniTrattamento { pioggia_entro_4h: true, pioggia_entro_6h: true, ..base() }).livello,
            Semaforo::Rosso
        );
    }

    #[test]
    fn cautela() {
        assert_eq!(finestra_trattamento(CondizioniTrattamento { vento_kmh: 12.0, ..base() }).livello, Semaforo::Giallo);
        assert_eq!(finestra_trattamento(CondizioniTrattamento { rh: 85.0, ..base() }).livello, Semaforo::Giallo);
        assert_eq!(finestra_trattamento(CondizioniTrattamento { pioggia_entro_6h: true, ..base() }).livello, Semaforo::Giallo);
        assert_eq!(finestra_trattamento(CondizioniTrattamento { t_aria: 8.0, ..base() }).livello, Semaforo::Giallo);
    }

    #[test]
    fn finestra_pioggia() {
        let p = [0.0, 0.0, 0.0, 0.0, 0.5, 0.0];
        assert!(pioggia_nelle_prossime(&p, 0, 4));
        assert!(!pioggia_nelle_prossime(&p, 0, 3));
        assert!(!pioggia_nelle_prossime(&p, 5, 6), "fine serie");
    }
}
