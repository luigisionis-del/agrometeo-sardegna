//! Almanacco colturale per la Sardegna: catalogo agronomico e analisi dinamica.
//!
//! Organizzato in sottomoduli tematici per facilitare la manutenzione da parte
//! di agronomi e sviluppatori:
//! - [`sezioni`]: struttura delle sezioni, mesi, bibliografia e avvertenze;
//! - [`suolo_cereali`]: lavorazioni del terreno, avena e orzo;
//! - [`foraggere_ovini`]: trifogli, erba medica, loietto e gestione foraggera per la pecora Sarda;
//! - [`orticole_canapa`]: 19 schede orticole e canapa industriale;
//! - [`analisi`]: funzioni pure che incrociano il report meteo/suolo con le soglie colturali.

pub mod analisi;
pub mod foraggere_ovini;
pub mod orticole_canapa;
pub mod sezioni;
pub mod suolo_cereali;

use std::collections::BTreeMap;

use serde::Serialize;

pub use analisi::{
    AnalisiAlmanacco, ChipColtura, ConsiglioGiorno, ContestoAgro, GruppoDopo, GruppoOperazioneMese, PeriodoSmith,
    RifColtura, VerificaSuolo, analizza_report, consigli_oggi, costruisci_contesto, in_campo, lavori_mese,
    periodi_smith, valuta_coltura, verifica_suolo,
};

/// Sezione tematica dell'Almanacco colturale.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct SezioneAlmanacco {
    pub id: &'static str,
    pub titolo: &'static str,
    pub icona: &'static str,
    pub illustrazione: &'static str,
    pub intro: &'static str,
    pub principi: &'static [(&'static str, &'static str)],
}

/// Soglie termiche del suolo a 6 cm (°C) per semina o trapianto in pieno campo.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct SoglieSuolo {
    pub min: f64,
    pub opt: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// Mesi (1–12) in cui si svolgono le principali operazioni colturali in Sardegna.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CalendarioColturale {
    pub semina: &'static [u32],
    pub semenzaio: &'static [u32],
    pub trapianto: &'static [u32],
    pub raccolta: &'static [u32],
    pub utilizzo: &'static [u32],
}

/// Blocco informativo o scientifico all'interno di una scheda colturale.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct BloccoScheda {
    pub titolo: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo: Option<&'static str>,
    pub punti: &'static [&'static str],
}

/// Tabella strutturata all'interno di una scheda (es. miscugli, fienagione, fabbisogni).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct TabellaScheda {
    pub titolo: &'static str,
    pub intestazioni: &'static [&'static str],
    pub righe: &'static [&'static [&'static str]],
}

/// Scheda tecnica o colturale completa.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct SchedaColturale {
    pub id: &'static str,
    pub nome: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latino: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub famiglia: Option<&'static str>,
    pub sezione: &'static str,
    /// Identificativo dell'illustrazione SVG descrittiva associata alla scheda.
    pub illustrazione: &'static str,
    /// Identificativo opzionale di un'infografica tecnica esplicativa dedicata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infografica: Option<&'static str>,
    pub legale: bool,
    pub sintesi: &'static str,
    #[serde(rename = "tSuolo", skip_serializing_if = "Option::is_none")]
    pub t_suolo: Option<SoglieSuolo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gelo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendario: Option<CalendarioColturale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<[f64; 3]>,
    pub dati: &'static [(&'static str, &'static str)],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabella: Option<TabellaScheda>,
    pub blocchi: &'static [BloccoScheda],
}

/// Catalogo statico completo dell'Almanacco colturale e delle illustrazioni SVG restituito al frontend.
#[derive(Debug, Clone, Serialize)]
pub struct CatalogoAlmanacco {
    pub sezioni: &'static [SezioneAlmanacco],
    pub schede: Vec<SchedaColturale>,
    pub mesi: [&'static str; 12],
    pub nomi_mesi: [&'static str; 12],
    pub fonti: &'static [&'static str],
    pub avvertenze: &'static [&'static str],
    pub illustrazioni: BTreeMap<&'static str, &'static str>,
}

/// Raccoglie in ordine tutte le schede colturali e tecniche dei sottomoduli.
pub fn tutte_le_schede() -> Vec<SchedaColturale> {
    let mut out = Vec::with_capacity(
        suolo_cereali::SCHEDE_SUOLO_CEREALI.len()
            + foraggere_ovini::SCHEDE_FORAGGERE_OVINI.len()
            + orticole_canapa::SCHEDE_ORTICOLE_CANAPA.len(),
    );
    out.extend_from_slice(suolo_cereali::SCHEDE_SUOLO_CEREALI);
    out.extend_from_slice(foraggere_ovini::SCHEDE_FORAGGERE_OVINI);
    out.extend_from_slice(orticole_canapa::SCHEDE_ORTICOLE_CANAPA);
    out
}

/// Restituisce il catalogo completo dell'Almanacco colturale comprensivo delle illustrazioni SVG.
pub fn catalogo() -> CatalogoAlmanacco {
    CatalogoAlmanacco {
        sezioni: sezioni::SEZIONI,
        schede: tutte_le_schede(),
        mesi: sezioni::MESI,
        nomi_mesi: sezioni::NOMI_MESI,
        fonti: sezioni::FONTI,
        avvertenze: sezioni::AVVERTENZE,
        illustrazioni: crate::illustrazioni::mappa_illustrazioni(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrita_catalogo() {
        let cat = catalogo();
        assert_eq!(cat.sezioni.len(), 8);
        assert_eq!(cat.schede.len(), 36);
        let sezioni_valide: std::collections::BTreeSet<&str> = cat.sezioni.iter().map(|s| s.id).collect();
        let mut id_visti = std::collections::BTreeSet::new();
        for s in &cat.schede {
            assert!(sezioni_valide.contains(s.sezione), "sezione ignota per {}", s.id);
            assert!(id_visti.insert(s.id), "id scheda duplicato: {}", s.id);
            assert!(!s.illustrazione.is_empty(), "illustrazione mancante per {}", s.id);
        }
    }
}
