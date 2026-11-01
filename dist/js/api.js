// Unico punto di contatto con il backend Rust: nessuna chiamata HTTP dal frontend.
const invoke = (...args) => window.__TAURI__.core.invoke(...args);

export const api = {
  report: (luogo, parametri, aggiorna = false) => invoke("report", { luogo, parametri, aggiorna }),
  radar: () => invoke("radar"),
  vento: () => invoke("vento"),
  cercaLocalita: (testo) => invoke("cerca_localita", { testo }),
  spazioLavoro: () => invoke("spazio_lavoro"),
  salvaSpazioLavoro: (spazio) => invoke("salva_spazio_lavoro", { spazio }),
  confrontaStazione: (nome, csv, lat, lon) => invoke("confronta_stazione", { nome, csv, lat, lon }),
};
