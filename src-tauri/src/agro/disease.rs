//! 3.2 Modelli fitopatologici: Peronospora (Plasmopara viticola) e Oidio (Erysiphe necator).

use super::{Semaforo, Valutazione};

/// Ingressi per la regola dei "3 dieci" (infezione primaria) in un'ora.
#[derive(Debug, Clone, Copy)]
pub struct IngressoPrimaria {
    /// Temperatura media dell'aria nelle 24 h precedenti (°C).
    pub t_media_24h: f64,
    /// Temperatura del suolo a 6 cm (°C).
    pub t_suolo_6cm: f64,
    /// Pioggia cumulata nelle 48 h precedenti (mm).
    pub pioggia_48h: f64,
    /// Lunghezza dei germogli (cm), impostata dall'utente.
    pub germogli_cm: f64,
}

/// Infezione primaria: rosso quando le tre condizioni sono soddisfatte;
/// giallo quando, con germogli > 10 cm, ne è soddisfatta una sola delle due meteo.
pub fn peronospora_primaria(i: IngressoPrimaria) -> Valutazione {
    if i.germogli_cm <= 10.0 {
        return Valutazione::new(Semaforo::Inattivo, "n.a.", "Germogli ≤ 10 cm: regola dei 3 dieci non applicabile");
    }
    let termica = i.t_media_24h >= 10.0 || i.t_suolo_6cm >= 10.0;
    let pioggia = i.pioggia_48h >= 10.0;
    let dettaglio = format!(
        "T media 24h {:.1} °C, T suolo 6cm {:.1} °C, pioggia 48h {:.1} mm",
        i.t_media_24h, i.t_suolo_6cm, i.pioggia_48h
    );
    match (termica, pioggia) {
        (true, true) => Valutazione::new(Semaforo::Rosso, "Infezione", format!("3 dieci soddisfatta: {dettaglio}")),
        (true, false) | (false, true) => {
            Valutazione::new(Semaforo::Giallo, "Parziale", format!("2 condizioni su 3: {dettaglio}"))
        }
        (false, false) => Valutazione::new(Semaforo::Verde, "Basso", dettaglio),
    }
}

/// Infezioni secondarie (Goidanich semplificato), valutate sul periodo di
/// bagnatura in corso fino all'ora considerata.
///
/// * rosso: LWD ≥ 6 h con T media in bagnatura 18–24 °C (specifica);
/// * giallo: LWD ≥ 6 h con T 10–28 °C, oppure LWD ≥ 3 h con T 18–24 °C.
pub fn peronospora_secondaria(ore_bagnatura: u32, t_media_bagnatura: f64, vite_in_vegetazione: bool) -> Valutazione {
    if !vite_in_vegetazione {
        return Valutazione::new(Semaforo::Inattivo, "n.a.", "Vite in riposo vegetativo");
    }
    if ore_bagnatura == 0 {
        return Valutazione::new(Semaforo::Verde, "Asciutto", "Foglia asciutta");
    }
    let t = t_media_bagnatura;
    let dettaglio = format!("{ore_bagnatura} h di bagnatura, T media {t:.1} °C");
    let ottimale = (18.0..=24.0).contains(&t);
    if ore_bagnatura >= 6 && ottimale {
        Valutazione::new(Semaforo::Rosso, "Alto", dettaglio)
    } else if (ore_bagnatura >= 6 && (10.0..=28.0).contains(&t)) || (ore_bagnatura >= 3 && ottimale) {
        Valutazione::new(Semaforo::Giallo, "Medio", dettaglio)
    } else {
        Valutazione::new(Semaforo::Verde, "Basso", dettaglio)
    }
}

/// Rischio oidio orario.
///
/// * inibito: T > 33 °C con forte insolazione (radiazione ≥ 600 W/m²);
/// * rosso: T 20–27 °C, RH > 45 % e pioggia 24 h < 2 mm (specifica);
/// * giallo: T 15–30 °C con RH > 45 % (attività fungina possibile);
/// * verde: altrimenti.
pub fn oidio(t: f64, rh: f64, pioggia_24h: f64, radiazione: f64, vite_in_vegetazione: bool) -> Valutazione {
    if !vite_in_vegetazione {
        return Valutazione::new(Semaforo::Inattivo, "n.a.", "Vite in riposo vegetativo");
    }
    let dettaglio = format!("T {t:.1} °C, RH {rh:.0} %, pioggia 24h {pioggia_24h:.1} mm");
    if t > 33.0 && radiazione >= 600.0 {
        return Valutazione::new(Semaforo::Inattivo, "Inibito", format!("Blocco termico: {dettaglio}, radiazione {radiazione:.0} W/m²"));
    }
    if (20.0..=27.0).contains(&t) && rh > 45.0 && pioggia_24h < 2.0 {
        Valutazione::new(Semaforo::Rosso, "Alto", dettaglio)
    } else if (15.0..=30.0).contains(&t) && rh > 45.0 {
        Valutazione::new(Semaforo::Giallo, "Medio", dettaglio)
    } else {
        Valutazione::new(Semaforo::Verde, "Basso", dettaglio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn primaria(t: f64, suolo: f64, pioggia: f64, germogli: f64) -> Semaforo {
        peronospora_primaria(IngressoPrimaria { t_media_24h: t, t_suolo_6cm: suolo, pioggia_48h: pioggia, germogli_cm: germogli }).livello
    }

    #[test]
    fn regola_tre_dieci() {
        assert_eq!(primaria(12.0, 8.0, 10.0, 12.0), Semaforo::Rosso);
        assert_eq!(primaria(8.0, 10.5, 15.0, 12.0), Semaforo::Rosso, "basta il suolo a 10 °C");
        assert_eq!(primaria(12.0, 8.0, 9.9, 12.0), Semaforo::Giallo);
        assert_eq!(primaria(8.0, 8.0, 3.0, 12.0), Semaforo::Verde);
        assert_eq!(primaria(15.0, 15.0, 30.0, 10.0), Semaforo::Inattivo, "germogli non oltre 10 cm");
    }

    #[test]
    fn goidanich_semplificato() {
        assert_eq!(peronospora_secondaria(6, 20.0, true).livello, Semaforo::Rosso);
        assert_eq!(peronospora_secondaria(5, 20.0, true).livello, Semaforo::Giallo);
        assert_eq!(peronospora_secondaria(8, 12.0, true).livello, Semaforo::Giallo);
        assert_eq!(peronospora_secondaria(2, 20.0, true).livello, Semaforo::Verde);
        assert_eq!(peronospora_secondaria(0, 20.0, true).livello, Semaforo::Verde);
        assert_eq!(peronospora_secondaria(10, 20.0, false).livello, Semaforo::Inattivo);
    }

    #[test]
    fn oidio_soglie() {
        assert_eq!(oidio(24.0, 60.0, 0.0, 500.0, true).livello, Semaforo::Rosso);
        assert_eq!(oidio(24.0, 60.0, 5.0, 500.0, true).livello, Semaforo::Giallo, "pioggia dilavante");
        assert_eq!(oidio(24.0, 40.0, 0.0, 500.0, true).livello, Semaforo::Verde);
        assert_eq!(oidio(34.0, 30.0, 0.0, 800.0, true).etichetta, "Inibito");
        assert_eq!(oidio(34.0, 50.0, 0.0, 200.0, true).livello, Semaforo::Verde, "caldo senza forte insolazione");
    }
}
