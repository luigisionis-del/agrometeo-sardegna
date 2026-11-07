//! Catalogo centralizzato di illustrazioni vettoriali (SVG) descrittive ed esplicative.
//!
//! Tutte le icone KPI, le icone delle righe della matrice 72 h, le illustrazioni
//! di sezione, i consigli operativi del giorno, le 36 schede botaniche/tecniche
//! e le infografiche scientifiche sono mantenute in moduli Rust dedicati:
//! - [`icone_kpi_matrice`]: icone KPI, righe matrice 72 h, sezioni e consigli operativi;
//! - [`colture`]: illustrazioni per le 36 schede dell'Almanacco colturale;
//! - [`infografiche`]: schemi tecnici esplicativi ad ampia larghezza.

pub mod colture;
pub mod icone_kpi_matrice;
pub mod infografiche;

use std::collections::BTreeMap;

/// Restituisce la mappa completa `id -> markup SVG` per tutte le illustrazioni dell'applicazione.
pub fn mappa_illustrazioni() -> BTreeMap<&'static str, &'static str> {
    let mut map = BTreeMap::new();
    for &(id, svg) in icone_kpi_matrice::ICONE_KPI_E_MATRICE {
        map.insert(id, svg);
    }
    for &(id, svg) in icone_kpi_matrice::ILLUSTRAZIONI_SEZIONI {
        map.insert(id, svg);
    }
    for &(id, svg) in icone_kpi_matrice::ILLUSTRAZIONI_CONSIGLI {
        map.insert(id, svg);
    }
    for &(id, svg) in colture::ILLUSTRAZIONI_COLTURE {
        map.insert(id, svg);
    }
    for &(id, svg) in infografiche::INFOGRAFICHE {
        map.insert(id, svg);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanacco;

    #[test]
    fn tutte_le_illustrazioni_sono_svg_validi_e_referenziati() {
        let map = mappa_illustrazioni();
        assert!(!map.is_empty());

        for (&id, &svg) in &map {
            let trimmed = svg.trim();
            assert!(
                trimmed.starts_with("<svg") && trimmed.ends_with("</svg>"),
                "L'illustrazione '{id}' non è un elemento <svg>...</svg> valido"
            );
        }

        // Verifica che ogni sezione dell'almanacco abbia la propria illustrazione nel catalogo
        let cat = almanacco::catalogo();
        for sez in cat.sezioni {
            assert!(
                map.contains_key(sez.illustrazione),
                "Illustrazione sezione mancante: {} (sezione {})",
                sez.illustrazione,
                sez.id
            );
        }

        // Verifica che ogni scheda colturale abbia illustrazione e (se presente) infografica nel catalogo
        for scheda in &cat.schede {
            assert!(
                map.contains_key(scheda.illustrazione),
                "Illustrazione scheda mancante: {} (scheda {})",
                scheda.illustrazione,
                scheda.id
            );
            if let Some(info_id) = scheda.infografica {
                assert!(
                    map.contains_key(info_id),
                    "Infografica scheda mancante: {} (scheda {})",
                    info_id,
                    scheda.id
                );
            }
        }

        // Verifica icone KPI, righe matrice 72 h, consigli e infografiche generali
        for kpi_id in [
            "kpi-temp",
            "kpi-rh",
            "kpi-vpd",
            "kpi-thi",
            "kpi-pioggia",
            "kpi-vento",
            "kpi-et0",
            "mat-trattamenti",
            "mat-bagnatura",
            "mat-peronospora",
            "mat-secondaria",
            "mat-oidio",
            "mat-thi",
            "mat-gelata",
            "mat-vpd",
            "cons-semina",
            "cons-lavorazioni",
            "cons-gelate",
            "cons-acqua",
            "cons-azoto",
            "cons-erbai",
            "cons-ovini",
            "cons-peronospora",
            "cons-fieno",
            "info-bilancio-idrico",
            "info-calibrazione",
        ] {
            assert!(map.contains_key(kpi_id), "Icona/infografica mancante: {kpi_id}");
        }
    }
}
