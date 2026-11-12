//! Livello 3: calibrazione locale con le osservazioni delle stazioni (es. rete ARPAS).
//!
//! Le fonti osservative implementano `FonteOsservazioni`; l'importatore CSV
//! riconosce automaticamente separatore, colonna data e colonna pioggia, così
//! da accettare le esportazioni ARPAS e quelle di stazioni aziendali.

use std::collections::BTreeMap;

use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use crate::error::{AppError, Result};
use crate::model::OpenMeteoHistory;

/// Pioggia giornaliera osservata.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct PioggiaOsservata {
    pub data: NaiveDate,
    pub mm: f64,
}

pub trait FonteOsservazioni {
    fn nome(&self) -> &str;
    fn pioggia_giornaliera(&self) -> Result<Vec<PioggiaOsservata>>;
}

/// File CSV con una colonna data/ora e una colonna di precipitazione.
/// Le righe orarie o sub-orarie vengono sommate per giorno.
pub struct CsvStazione<'a> {
    pub nome: String,
    pub testo: &'a str,
}

const FORMATI_DATA_ORA: &[&str] =
    &["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M:%S", "%Y-%m-%dT%H:%M", "%d/%m/%Y %H:%M:%S", "%d/%m/%Y %H:%M"];
const FORMATI_DATA: &[&str] = &["%Y-%m-%d", "%d/%m/%Y", "%d-%m-%Y", "%Y%m%d"];

pub fn parse_data_flessibile(s: &str) -> Option<NaiveDate> {
    let s = s.trim().trim_matches('"');
    FORMATI_DATA_ORA
        .iter()
        .find_map(|f| NaiveDateTime::parse_from_str(s, f).ok().map(|d| d.date()))
        .or_else(|| FORMATI_DATA.iter().find_map(|f| NaiveDate::parse_from_str(s, f).ok()))
}

fn parse_numero(s: &str) -> Option<f64> {
    let s = s.trim().trim_matches('"').replace(',', ".");
    if s.is_empty() { None } else { s.parse().ok().filter(|v: &f64| v.is_finite()) }
}

impl CsvStazione<'_> {
    fn separatore(intestazione: &str) -> char {
        [';', '\t', ','].into_iter().max_by_key(|c| intestazione.matches(*c).count()).unwrap_or(';')
    }
}

impl FonteOsservazioni for CsvStazione<'_> {
    fn nome(&self) -> &str {
        &self.nome
    }

    fn pioggia_giornaliera(&self) -> Result<Vec<PioggiaOsservata>> {
        let mut righe = self.testo.lines().map(str::trim).filter(|l| !l.is_empty());
        let intestazione = righe.next().ok_or_else(|| AppError::Input("Il file CSV è vuoto".into()))?;
        let sep = Self::separatore(intestazione);
        let colonne: Vec<String> =
            intestazione.split(sep).map(|c| c.trim().trim_matches('"').to_lowercase()).collect();
        let trova = |chiavi: &[&str]| colonne.iter().position(|c| chiavi.iter().any(|k| c.contains(k)));
        let col_data = trova(&["data", "date", "time", "giorno"])
            .ok_or_else(|| AppError::Input("Colonna data non trovata (attesa: data/date/time)".into()))?;
        let col_pioggia = trova(&["prec", "piogg", "rain", "mm"])
            .ok_or_else(|| AppError::Input("Colonna pioggia non trovata (attesa: precipitazione/pioggia/rain/mm)".into()))?;

        let mut per_giorno: BTreeMap<NaiveDate, f64> = BTreeMap::new();
        for riga in righe {
            let campi: Vec<&str> = riga.split(sep).collect();
            let (Some(d), Some(mm)) = (
                campi.get(col_data).and_then(|c| parse_data_flessibile(c)),
                campi.get(col_pioggia).and_then(|c| parse_numero(c)),
            ) else {
                continue;
            };
            *per_giorno.entry(d).or_default() += mm;
        }
        if per_giorno.is_empty() {
            return Err(AppError::Input("Nessuna riga valida (data + pioggia) nel CSV".into()));
        }
        Ok(per_giorno.into_iter().map(|(data, mm)| PioggiaOsservata { data, mm }).collect())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RigaConfronto {
    pub data: NaiveDate,
    pub osservato: f64,
    pub modello: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Confronto {
    pub stazione: String,
    pub righe: Vec<RigaConfronto>,
    pub giorni_confrontati: usize,
    pub totale_osservato: f64,
    pub totale_modello: f64,
    /// (modello − osservato) / osservato, in percentuale.
    pub bias_percento: Option<f64>,
    /// Errore medio assoluto giornaliero (mm).
    pub mae: f64,
    /// Percentuale di giorni in cui modello e stazione concordano su pioggia ≥ 1 mm.
    pub concordanza_giorni_piovosi: f64,
}

pub fn confronta(stazione: &str, oss: &[PioggiaOsservata], modello: &OpenMeteoHistory) -> Confronto {
    let mod_per_data: BTreeMap<NaiveDate, f64> = modello
        .daily
        .time
        .iter()
        .zip(&modello.daily.precipitation_sum)
        .filter_map(|(d, v)| Some((crate::model::parse_data(d)?, (*v)?)))
        .collect();
    let righe: Vec<RigaConfronto> = oss
        .iter()
        .map(|o| RigaConfronto { data: o.data, osservato: o.mm, modello: mod_per_data.get(&o.data).copied() })
        .collect();
    let coppie: Vec<(f64, f64)> = righe.iter().filter_map(|r| Some((r.osservato, r.modello?))).collect();
    let n = coppie.len();
    let tot_o: f64 = coppie.iter().map(|c| c.0).sum();
    let tot_m: f64 = coppie.iter().map(|c| c.1).sum();
    let mae = if n == 0 { f64::NAN } else { coppie.iter().map(|(o, m)| (m - o).abs()).sum::<f64>() / n as f64 };
    let concordi = coppie.iter().filter(|(o, m)| (*o >= 1.0) == (*m >= 1.0)).count();
    Confronto {
        stazione: stazione.to_string(),
        righe,
        giorni_confrontati: n,
        totale_osservato: tot_o,
        totale_modello: tot_m,
        bias_percento: (tot_o > 0.0).then(|| (tot_m - tot_o) / tot_o * 100.0),
        mae,
        concordanza_giorni_piovosi: if n == 0 { f64::NAN } else { concordi as f64 / n as f64 * 100.0 },
    }
}

/// Genera un tracciato CSV dimostrativo, con il tracciato delle esportazioni ARPAS,
/// sugli ultimi 30 giorni per provare subito la calibrazione locale.
/// I valori sono sintetici: non provengono da ARPAS né da alcuna stazione reale.
pub fn genera_csv_esempio(oggi: NaiveDate) -> String {
    let mut righe = vec!["Data;Precipitazione (mm);Note".to_string()];
    // Serie pluviometrica realistica su 30 giorni con alcuni eventi autunnali/primaverili
    let campione: [f64; 30] = [
        0.0, 0.0, 4.2, 18.6, 2.4, 0.0, 0.0, 0.0, 1.2, 0.0,
        0.0, 8.5, 14.0, 0.4, 0.0, 0.0, 0.0, 0.0, 6.8, 22.4,
        3.0, 0.0, 0.0, 0.0, 0.0, 11.2, 1.8, 0.0, 0.0, 0.0,
    ];
    for (i, &mm) in campione.iter().enumerate() {
        let giorni_indietro = (campione.len() - i) as i64;
        let data = oggi - chrono::Duration::days(giorni_indietro);
        let mm_it = format!("{mm:.1}").replace('.', ",");
        righe.push(format!("{};{mm_it};Dato sintetico dimostrativo", data.format("%d/%m/%Y")));
    }
    righe.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DailyHistoryRaw;

    #[test]
    fn csv_arpas_orario_virgola_decimale() {
        let testo = "Data;Ora;Precipitazione (mm);Temperatura\n01/10/2026 00:00;x;0,2;12\n01/10/2026 01:00;x;1,4;12\n02/10/2026 00:00;x;0;11\n";
        let s = CsvStazione { nome: "Seulo".into(), testo };
        let p = s.pioggia_giornaliera().unwrap();
        assert_eq!(p.len(), 2);
        assert!((p[0].mm - 1.6).abs() < 1e-9);
    }

    #[test]
    fn csv_iso_virgola() {
        let s = CsvStazione { nome: "x".into(), testo: "date,rain_mm\n2026-10-01,5.5\n2026-10-02,\n" };
        assert_eq!(s.pioggia_giornaliera().unwrap(), vec![PioggiaOsservata {
            data: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            mm: 5.5
        }]);
        assert!(CsvStazione { nome: "x".into(), testo: "a,b\n1,2" }.pioggia_giornaliera().is_err());
    }

    #[test]
    fn statistiche_confronto() {
        let d = |g| NaiveDate::from_ymd_opt(2026, 10, g).unwrap();
        let oss = [PioggiaOsservata { data: d(1), mm: 10.0 }, PioggiaOsservata { data: d(2), mm: 0.0 }];
        let modello = OpenMeteoHistory {
            daily: DailyHistoryRaw {
                time: vec!["2026-10-01".into(), "2026-10-02".into()],
                temperature_2m_max: vec![],
                temperature_2m_min: vec![],
                precipitation_sum: vec![Some(8.0), Some(2.0)],
            },
        };
        let c = confronta("x", &oss, &modello);
        assert_eq!(c.giorni_confrontati, 2);
        assert_eq!(c.bias_percento, Some(0.0));
        assert_eq!(c.mae, 2.0);
        assert_eq!(c.concordanza_giorni_piovosi, 50.0);
    }
}
