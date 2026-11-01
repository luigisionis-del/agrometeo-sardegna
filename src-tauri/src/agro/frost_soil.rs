//! 3.7 Allerta gelate e termometria del suolo per semina e lavorabilità.

use super::{Semaforo, Valutazione};
use serde::{Deserialize, Serialize};

/// Gelata per irraggiamento: T ≤ 3 °C, Td ≤ 0 °C, nuvolosità < 25 %, vento < 8 km/h.
pub fn gelata_irraggiamento(t_aria: f64, t_rugiada: f64, nuvolosita: f64, vento_kmh: f64) -> bool {
    t_aria <= 3.0 && t_rugiada <= 0.0 && nuvolosita < 25.0 && vento_kmh < 8.0
}

/// Semaforo orario gelate: rosso per allerta da irraggiamento o T ≤ 0 °C,
/// giallo per T ≤ 3 °C senza le condizioni di cielo sereno e calma di vento.
pub fn allerta_gelata(t_aria: f64, t_rugiada: f64, nuvolosita: f64, vento_kmh: f64) -> Valutazione {
    let v = format!("T {t_aria:.1} °C, Td {t_rugiada:.1} °C, nubi {nuvolosita:.0} %, vento {vento_kmh:.0} km/h");
    if gelata_irraggiamento(t_aria, t_rugiada, nuvolosita, vento_kmh) {
        Valutazione::new(Semaforo::Rosso, "Irragg.", format!("Gelata per irraggiamento: {v}"))
    } else if t_aria <= 0.0 {
        Valutazione::new(Semaforo::Rosso, "Gelo", format!("Temperatura sotto zero: {v}"))
    } else if t_aria <= 3.0 {
        Valutazione::new(Semaforo::Giallo, "Freddo", format!("Prossimo alla soglia: {v}"))
    } else {
        Valutazione::new(Semaforo::Verde, "No", v)
    }
}

/// Caratteristiche idrologiche del suolo (m³/m³), da adattare alla tessitura.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ParametriSuolo {
    pub punto_appassimento: f64,
    pub capacita_campo: f64,
}

impl Default for ParametriSuolo {
    /// Valori indicativi per un suolo franco.
    fn default() -> Self {
        Self { punto_appassimento: 0.12, capacita_campo: 0.32 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatoUmidita {
    Secco,
    Tempera,
    Eccesso,
}

/// Il suolo è "in tempera" tra il 40 % dell'acqua disponibile e la capacità
/// di campo; sopra la capacità di campo si ha ristagno, sotto è troppo secco.
pub fn stato_umidita(umidita: f64, p: ParametriSuolo) -> StatoUmidita {
    let soglia_tempera = p.punto_appassimento + 0.4 * (p.capacita_campo - p.punto_appassimento);
    if umidita > p.capacita_campo {
        StatoUmidita::Eccesso
    } else if umidita >= soglia_tempera {
        StatoUmidita::Tempera
    } else {
        StatoUmidita::Secco
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IdoneitaSemina {
    pub t_suolo_6cm: f64,
    pub umidita_3_9cm: f64,
    pub stato_umidita: StatoUmidita,
    /// Cereali e foraggere autunno-vernine: T suolo 6 cm > 8 °C.
    pub autunno_vernine: bool,
    /// Colture primaverili: T suolo 6 cm > 12 °C.
    pub primaverili: bool,
    pub lavorabile: bool,
    pub valutazione: Valutazione,
}

pub fn idoneita_semina(t_suolo_6cm: f64, umidita_3_9cm: f64, p: ParametriSuolo) -> IdoneitaSemina {
    let stato = stato_umidita(umidita_3_9cm, p);
    let autunno_vernine = t_suolo_6cm > 8.0;
    let primaverili = t_suolo_6cm > 12.0;
    let lavorabile = stato == StatoUmidita::Tempera;
    let v = format!("T suolo 6 cm {t_suolo_6cm:.1} °C, umidità 3–9 cm {umidita_3_9cm:.3} m³/m³");
    let valutazione = match (stato, autunno_vernine) {
        (StatoUmidita::Tempera, true) => Valutazione::new(Semaforo::Verde, "Idoneo", format!("Suolo in tempera: {v}")),
        (StatoUmidita::Eccesso, _) => {
            Valutazione::new(Semaforo::Rosso, "Ristagno", format!("Eccesso idrico, non lavorare: {v}"))
        }
        (StatoUmidita::Secco, _) => Valutazione::new(Semaforo::Giallo, "Secco", format!("Suolo troppo asciutto: {v}")),
        (StatoUmidita::Tempera, false) => Valutazione::new(Semaforo::Giallo, "Freddo", format!("Suolo troppo freddo: {v}")),
    };
    IdoneitaSemina { t_suolo_6cm, umidita_3_9cm, stato_umidita: stato, autunno_vernine, primaverili, lavorabile, valutazione }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gelata() {
        assert!(gelata_irraggiamento(2.0, -1.0, 10.0, 3.0));
        assert!(!gelata_irraggiamento(2.0, -1.0, 40.0, 3.0), "cielo coperto");
        assert!(!gelata_irraggiamento(2.0, -1.0, 10.0, 9.0), "ventilato");
        assert_eq!(allerta_gelata(-1.0, -3.0, 90.0, 20.0).livello, Semaforo::Rosso);
        assert_eq!(allerta_gelata(2.5, 1.0, 90.0, 20.0).livello, Semaforo::Giallo);
        assert_eq!(allerta_gelata(6.0, 1.0, 0.0, 0.0).livello, Semaforo::Verde);
    }

    #[test]
    fn umidita_suolo() {
        let p = ParametriSuolo::default(); // tempera da 0.20 a 0.32
        assert_eq!(stato_umidita(0.15, p), StatoUmidita::Secco);
        assert_eq!(stato_umidita(0.25, p), StatoUmidita::Tempera);
        assert_eq!(stato_umidita(0.35, p), StatoUmidita::Eccesso);
    }

    #[test]
    fn semina() {
        let p = ParametriSuolo::default();
        let s = idoneita_semina(10.0, 0.25, p);
        assert!(s.autunno_vernine && !s.primaverili && s.lavorabile);
        assert_eq!(s.valutazione.livello, Semaforo::Verde);
        assert_eq!(idoneita_semina(6.0, 0.25, p).valutazione.livello, Semaforo::Giallo);
        assert_eq!(idoneita_semina(14.0, 0.4, p).valutazione.livello, Semaforo::Rosso);
    }
}
