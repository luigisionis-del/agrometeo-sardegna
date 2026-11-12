//! Agrometeo Sardegna: motore di dominio indipendente dall'interfaccia.
//!
//! Strati: `fetch` (data ingestion) → `report` + `agro` + `almanacco` (calcolo
//! agronomico, funzioni pure) → `storage` (cache offline e spazio di lavoro).
//! Il crate non dipende da Tauri: compila e si testa su qualunque macchina
//! con la sola toolchain Rust, e può essere riusato da un server o da WASM.

pub mod agro;
pub mod almanacco;
pub mod error;
pub mod fetch;
pub mod illustrazioni;
pub mod model;
pub mod report;
pub mod stations;
pub mod storage;
