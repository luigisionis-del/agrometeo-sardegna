// Avvio dell'applicazione: carica lo spazio di lavoro e coordina i moduli di rendering.
import { aggiornaAlmanacco, initAlmanacco } from "./almanacco.js";
import { api } from "./api.js";
import { graficoBilancio, graficoGdd, graficoRadiazione, graficoTemperatura, tabellaGiorni } from "./grafici.js";
import { initImpostazioni } from "./impostazioni.js";
import { applicaLayer, centraSuLuogo, initMappa } from "./mappa.js";
import { initMatrice, renderAllerte, renderMatrice } from "./matrice.js";
import { initNavigazione } from "./navigazione.js";
import { aggiornaReport, salvaSpazio, stato, suReport } from "./stato.js";
import { initStazioni } from "./stazioni.js";
import { renderSuolo } from "./suolo.js";
import { initTestata, mostraErrore, renderKpi, renderLuoghi } from "./testata.js";

const AUTO_AGGIORNAMENTO_MS = 15 * 60_000;
let richiesta = 0;

async function carica(aggiorna = false) {
  const id = ++richiesta;
  const pulsante = document.getElementById("aggiorna");
  pulsante.disabled = true;
  document.querySelector("main").classList.add("caricamento");
  try {
    const r = await api.report(stato.spazio.luogo_attivo, stato.spazio.parametri, aggiorna);
    if (id === richiesta) aggiornaReport(r);
  } catch (e) {
    if (id === richiesta) mostraErrore(String(e));
  } finally {
    if (id === richiesta) {
      pulsante.disabled = false;
      document.querySelector("main").classList.remove("caricamento");
    }
  }
}

function impostaLuogo(luogo) {
  stato.spazio.luogo_attivo = { nome: luogo.nome, lat: luogo.lat, lon: luogo.lon };
  salvaSpazio();
  renderLuoghi();
  centraSuLuogo(stato.spazio.luogo_attivo);
  carica();
}

function renderTutto(r) {
  renderKpi(r);
  renderAllerte(r);
  renderSuolo(r);
  renderMatrice(r);
  graficoBilancio(r);
  graficoTemperatura(r);
  graficoRadiazione(r);
  graficoGdd(r);
  tabellaGiorni(r);
  aggiornaAlmanacco(r);
}

async function avvia() {
  initNavigazione();
  initAlmanacco();
  stato.spazio = await api.spazioLavoro();
  suReport(renderTutto);
  initTestata({ impostaLuogo, aggiorna: carica });
  initMatrice();
  initMappa({ impostaLuogo });
  initImpostazioni({ onParametri: () => carica(), onLayer: applicaLayer });
  initStazioni();
  renderLuoghi();
  centraSuLuogo(stato.spazio.luogo_attivo, false);
  await carica();
  setInterval(() => carica(), AUTO_AGGIORNAMENTO_MS);
}

avvia().catch((e) => mostraErrore(`Avvio non riuscito: ${e}`));
