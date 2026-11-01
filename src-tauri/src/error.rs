use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Errore di rete: {0}")]
    Rete(String),
    #[error("Risposta non valida da {fonte}: {dettaglio}")]
    Formato { fonte: &'static str, dettaglio: String },
    #[error("Errore di archiviazione locale: {0}")]
    Archivio(String),
    #[error("Dati non disponibili: nessuna connessione e nessuna copia in cache per questa località")]
    NessunDato,
    #[error("{0}")]
    Input(String),
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Rete(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Archivio(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Archivio(e.to_string())
    }
}

/// I comandi Tauri restituiscono l'errore al frontend come messaggio leggibile.
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
