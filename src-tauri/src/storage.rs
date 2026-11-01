//! Persistenza locale: cache dei payload (resilienza offline) e spazio di lavoro.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::model::Luogo;
use crate::report::{ParametriAgro, Payload};

pub struct Archivio {
    dir: PathBuf,
}

/// Preferenze dell'utente: località preimpostate, parametri, appezzamenti e layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SpazioLavoro {
    pub luoghi: Vec<Luogo>,
    pub luogo_attivo: Luogo,
    pub parametri: ParametriAgro,
    /// FeatureCollection GeoJSON degli appezzamenti (vigneto, pascolo, seminativo…).
    pub appezzamenti: serde_json::Value,
    pub layer: ImpostazioniLayer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ImpostazioniLayer {
    pub radar_opacita: f64,
    pub vento: bool,
    /// Layer WMS opzionale (es. mosaico radar DPC VMI/SRI).
    pub wms_attivo: bool,
    pub wms_url: String,
    pub wms_layer: String,
    pub wms_opacita: f64,
    pub base: String,
    /// Versione delle preferenze grafiche: i file senza questo campo (0)
    /// vengono migrati una sola volta alla base cartografica chiara.
    #[serde(default)]
    pub versione_tema: u32,
}

/// Versione corrente delle preferenze grafiche (1 = tema chiaro).
pub const VERSIONE_TEMA: u32 = 1;

impl Default for ImpostazioniLayer {
    fn default() -> Self {
        Self {
            radar_opacita: 0.7,
            vento: true,
            wms_attivo: false,
            wms_url: "https://radar-geowms.protezionecivile.it/service/wms".into(),
            wms_layer: "radar:vmi".into(),
            wms_opacita: 0.6,
            base: "chiaro".into(),
            versione_tema: VERSIONE_TEMA,
        }
    }
}

pub fn luoghi_predefiniti() -> Vec<Luogo> {
    vec![
        Luogo { nome: "Sarcidano (Isili)".into(), lat: 39.79, lon: 9.08 },
        Luogo { nome: "Seulo".into(), lat: 39.8696, lon: 9.2361 },
    ]
}

impl Default for SpazioLavoro {
    fn default() -> Self {
        let luoghi = luoghi_predefiniti();
        Self {
            luogo_attivo: luoghi[0].clone(),
            luoghi,
            parametri: ParametriAgro::default(),
            appezzamenti: serde_json::json!({ "type": "FeatureCollection", "features": [] }),
            layer: ImpostazioniLayer::default(),
        }
    }
}

/// Chiave di cache: coordinate arrotondate a ~1 km (risoluzione dei modelli).
pub fn chiave_luogo(lat: f64, lon: f64) -> String {
    format!("{:.2}_{:.2}", lat, lon).replace('-', "m")
}

impl Archivio {
    pub fn new(dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(dir.join("cache"))?;
        Ok(Self { dir })
    }

    fn file_cache(&self, lat: f64, lon: f64) -> PathBuf {
        self.dir.join("cache").join(format!("previsione_{}.json", chiave_luogo(lat, lon)))
    }

    pub fn leggi_payload(&self, lat: f64, lon: f64) -> Option<Payload> {
        leggi_json(&self.file_cache(lat, lon))
    }

    pub fn salva_payload(&self, p: &Payload) -> Result<()> {
        scrivi_json(&self.file_cache(p.luogo.lat, p.luogo.lon), p)
    }

    pub fn leggi_spazio(&self) -> SpazioLavoro {
        let mut s: SpazioLavoro = leggi_json(&self.dir.join("spazio_lavoro.json")).unwrap_or_default();
        migra_tema(&mut s.layer);
        s
    }

    pub fn salva_spazio(&self, s: &SpazioLavoro) -> Result<()> {
        scrivi_json(&self.dir.join("spazio_lavoro.json"), s)
    }
}

/// Le preferenze salvate prima del tema chiaro passano una volta sola dalla
/// base scura (allora predefinita) a quella chiara; una scelta successiva
/// dell'utente viene poi rispettata.
fn migra_tema(layer: &mut ImpostazioniLayer) {
    if layer.versione_tema < VERSIONE_TEMA {
        if layer.base == "scuro" {
            layer.base = "chiaro".into();
        }
        layer.versione_tema = VERSIONE_TEMA;
    }
}

fn leggi_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Option<T> {
    let testo = fs::read_to_string(path).ok()?;
    serde_json::from_str(&testo).ok()
}

/// Scrittura atomica: file temporaneo + rename, per non lasciare JSON troncati.
fn scrivi_json<T: Serialize>(path: &Path, v: &T) -> Result<()> {
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec(v)?)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spazio_lavoro_roundtrip_e_campi_mancanti() {
        let dir = std::env::temp_dir().join(format!("agrometeo-test-{}", std::process::id()));
        let a = Archivio::new(dir.clone()).unwrap();
        assert_eq!(a.leggi_spazio().luoghi.len(), 2);
        let mut s = SpazioLavoro::default();
        s.parametri.kc = 0.45;
        a.salva_spazio(&s).unwrap();
        assert_eq!(a.leggi_spazio().parametri.kc, 0.45);
        // un file parziale (versioni precedenti) viene completato con i default
        fs::write(dir.join("spazio_lavoro.json"), r#"{"parametri":{"kc":0.9}}"#).unwrap();
        let s = a.leggi_spazio();
        assert_eq!(s.parametri.kc, 0.9);
        assert_eq!(s.parametri.ore_matrice, 72);
        // la base scura delle versioni precedenti migra una sola volta a quella chiara
        fs::write(dir.join("spazio_lavoro.json"), r#"{"layer":{"base":"scuro"}}"#).unwrap();
        assert_eq!(a.leggi_spazio().layer.base, "chiaro");
        fs::write(dir.join("spazio_lavoro.json"), r#"{"layer":{"base":"scuro","versione_tema":1}}"#).unwrap();
        assert_eq!(a.leggi_spazio().layer.base, "scuro");
        fs::remove_dir_all(dir).unwrap();
    }
}
