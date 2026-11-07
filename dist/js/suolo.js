// Soil profiler: stratigrafia interattiva con sezione SVG degli apparati radicali,
// termometri e barre di saturazione idrica calcolate dal motore Rust.
import { el, giornoBreve, num, pallino } from "./formato.js";
import { graficoSuolo } from "./grafici.js";

const T_MIN = -5;
const T_MAX = 35;
const U_MAX = 0.5;

// Per ogni profondità di temperatura, lo strato di umidità che la contiene.
const STRATO_PER_PROFONDITA = [0, 2, 3, 4];
const DESCRIZIONE = ["superficie / crosta", "letto di semina", "radici erbacee", "radici arboree / vite"];

const ETICHETTA_STATO = {
  secco: { testo: "Sotto PA (secco)", livello: "rosso" },
  stress: { testo: "Stress idrico (< 40 % AD)", livello: "giallo" },
  ottimale: { testo: "In tempera (40–100 % AD)", livello: "verde" },
  saturo: { testo: "Oltre CC (saturo)", livello: "arancio" },
};

const COLORE_STRATO_SVG = {
  secco: "#d9b99b",
  stress: "#c79d6e",
  ottimale: "#8f6840",
  saturo: "#5d7ea3",
};

const percento = (v, min, max) => `${Math.min(100, Math.max(0, ((v - min) / (max - min)) * 100))}%`;

const parser = new DOMParser();

function illustrazioneStratigrafia(s) {
  const stati = s.stati_strati ?? ["ottimale", "ottimale", "ottimale", "ottimale", "ottimale"];
  const fraz = s.frazione_disponibile ?? [0.5, 0.5, 0.5, 0.5, 0.5];
  const t = s.temperature;
  const col = (idx) => COLORE_STRATO_SVG[stati[idx]] ?? "#8f6840";
  const adPct = (idx) => `${Math.round(Math.max(0, Math.min(1.5, fraz[idx])) * 100)}%`;

  const markup = `<svg viewBox="0 0 680 156" fill="none" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Sezione stratigrafica del suolo e profondità degli apparati radicali">
    <rect width="680" height="156" rx="10" fill="#eef5ef" stroke="#d8dfd5"/>
    <!-- Cielo ed erba superficiale -->
    <rect x="1" y="1" width="678" height="28" rx="9" fill="#e3f2fd"/>
    <path d="M0 29h680" stroke="#1f8a4c" stroke-width="2.5"/>
    <!-- Strato 0-3 cm: crosta -->
    <rect x="1" y="29" width="678" height="22" fill="${col(0)}" fill-opacity="0.85"/>
    <!-- Strato 3-9 cm: letto di semina -->
    <rect x="1" y="51" width="678" height="28" fill="${col(2)}" fill-opacity="0.9"/>
    <!-- Strato 9-27 cm: radici erbacee -->
    <rect x="1" y="79" width="678" height="36" fill="${col(3)}" fill-opacity="0.92"/>
    <!-- Strato 27-81 cm: radici arboree / vite -->
    <rect x="1" y="115" width="678" height="40" rx="0" fill="${col(4)}" fill-opacity="0.95"/>
    <line x1="1" y1="51" x2="679" y2="51" stroke="#ffffff" stroke-opacity="0.45" stroke-dasharray="4 4"/>
    <line x1="1" y1="79" x2="679" y2="79" stroke="#ffffff" stroke-opacity="0.45" stroke-dasharray="4 4"/>
    <line x1="1" y1="115" x2="679" y2="115" stroke="#ffffff" stroke-opacity="0.45" stroke-dasharray="4 4"/>

    <!-- Quote profondità a sinistra -->
    <g font-size="10.5" font-weight="700" fill="#fff">
      <rect x="10" y="32" width="138" height="16" rx="4" fill="#17221b" fill-opacity="0.68"/>
      <text x="16" y="44">0 cm · Crosta (${num(t[0])} °C)</text>
      <rect x="10" y="56" width="166" height="18" rx="4" fill="#17221b" fill-opacity="0.68"/>
      <text x="16" y="68">6 cm · Letto semina (${num(t[1])} °C · ${adPct(2)} AD)</text>
      <rect x="10" y="88" width="174" height="18" rx="4" fill="#17221b" fill-opacity="0.68"/>
      <text x="16" y="100">18 cm · Erbacee (${num(t[2])} °C · ${adPct(3)} AD)</text>
      <rect x="10" y="126" width="174" height="18" rx="4" fill="#17221b" fill-opacity="0.68"/>
      <text x="16" y="138">54 cm · Arboree (${num(t[3])} °C · ${adPct(4)} AD)</text>
    </g>

    <!-- 1. Seme in germinazione (3-6 cm) -->
    <g transform="translate(230, 10)">
      <path d="M24 19V7M24 12c-4 0-7-3-7-6 4 0 7 2 7 6ZM24 10c4 0 7-3 7-6-4 0-7 2-7 6Z" stroke="#146c3a" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="24" cy="54" r="5" fill="#f2b01e" stroke="#5d3a1a" stroke-width="1.5"/>
      <path d="M24 49V19M24 59v12l-5 6M24 66l6 6" stroke="#fff9e6" stroke-width="2" stroke-linecap="round"/>
      <rect x="40" y="45" width="96" height="18" rx="4" fill="#ffffff" fill-opacity="0.88"/>
      <text x="46" y="57" font-size="10" font-weight="700" fill="#17221b">Seme / Ortive</text>
    </g>

    <!-- 2. Cereali e foraggere (fino a 27 cm) -->
    <g transform="translate(385, 6)">
      <path d="M26 23V4M20 23V8M32 23V8" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
      <path d="M26 23c-4 18-12 38-16 68M26 23v72M26 23c4 18 12 38 16 66M20 55l-8 22M32 55l8 22" stroke="#fff9e6" stroke-width="1.8" stroke-linecap="round"/>
      <rect x="46" y="76" width="108" height="18" rx="4" fill="#ffffff" fill-opacity="0.88"/>
      <text x="52" y="88" font-size="10" font-weight="700" fill="#17221b">Cereali / Foraggere</text>
    </g>

    <!-- 3. Vite, olivo, medica e canapa (radici profonde 54+ cm) -->
    <g transform="translate(545, 4)">
      <path d="M28 25V9" stroke="#5d3a1a" stroke-width="4" stroke-linecap="round"/>
      <circle cx="22" cy="9" r="6" fill="#2ea85c"/><circle cx="34" cy="9" r="6" fill="#2ea85c"/><circle cx="28" cy="5" r="6" fill="#1f8a4c"/>
      <path d="M28 25v118M28 48l-14 26M28 66l16 28M28 92l-18 36M28 105l16 32" stroke="#fff9e6" stroke-width="2.2" stroke-linecap="round"/>
      <rect x="-52" y="116" width="122" height="18" rx="4" fill="#ffffff" fill-opacity="0.9"/>
      <text x="-46" y="128" font-size="10" font-weight="700" fill="#17221b">Vite / Medica / Canapa</text>
    </g>
  </svg>`;

  const box = el("div", { class: "sezione-radici-svg" });
  const doc = parser.parseFromString(markup, "image/svg+xml");
  box.appendChild(document.importNode(doc.documentElement, true));
  return box;
}

function barraTermica(t) {
  return el(
    "div",
    { class: "barra termo", title: `Temperatura ${num(t)} °C` },
    el("span", { style: `width:${percento(t, T_MIN, T_MAX)}` }),
    el("div", { class: "testo" }, `${num(t)} °C`),
  );
}

function barraAcqua(u, strato, suolo, statoStrato, fraz) {
  const info = ETICHETTA_STATO[statoStrato] ?? ETICHETTA_STATO.ottimale;
  const tempera = suolo.punto_appassimento + 0.4 * (suolo.capacita_campo - suolo.punto_appassimento);
  return el(
    "div",
    {
      class: "barra acqua",
      title: `Umidità ${strato[0]}–${strato[1]} cm: ${num(u, 3)} m³/m³ · ${info.testo} (PA ${suolo.punto_appassimento}, soglia tempera ${num(tempera, 2)}, CC ${suolo.capacita_campo})`,
    },
    el("span", { style: `width:${percento(u, 0, U_MAX)}` }),
    el("i", { class: "tacca pa", style: `left:${percento(suolo.punto_appassimento, 0, U_MAX)}`, title: "Punto di appassimento (PA)" }),
    el("i", { class: "tacca tempera", style: `left:${percento(tempera, 0, U_MAX)}`, title: "Soglia minima di tempera (40 % AD)" }),
    el("i", { class: "tacca cc", style: `left:${percento(suolo.capacita_campo, 0, U_MAX)}`, title: "Capacità di campo (CC)" }),
    el("div", { class: "testo" }, `${num(u, 3)} m³/m³ · ${Math.round(Math.max(0, fraz) * 100)}% AD`),
  );
}

export function renderSuolo(r) {
  const s = r.suolo;
  const suolo = r.parametri.suolo;
  const stati = s.stati_strati ?? ["ottimale", "ottimale", "ottimale", "ottimale", "ottimale"];
  const fraz = s.frazione_disponibile ?? [0.5, 0.5, 0.5, 0.5, 0.5];

  document.getElementById("profilo-suolo").replaceChildren(
    illustrazioneStratigrafia(s),
    ...s.profondita_cm.map((p, i) => {
      const k = STRATO_PER_PROFONDITA[i];
      const infoStato = ETICHETTA_STATO[stati[k]] ?? ETICHETTA_STATO.ottimale;
      return el(
        "div",
        { class: "strato" },
        el(
          "div",
          { class: "prof" },
          el("span", { class: "prof-quota" }, pallino(infoStato.livello), `${p} cm`),
          el("small", {}, `${DESCRIZIONE[i]} (${s.strati_cm[k][0]}–${s.strati_cm[k][1]} cm)`),
        ),
        barraTermica(s.temperature[i]),
        barraAcqua(s.umidita[k], s.strati_cm[k], suolo, stati[k], fraz[k]),
      );
    }),
  );

  const sem = s.semina;
  const prossimi = r.giorni.filter((g) => g.previsto).slice(0, 6);
  document.getElementById("semina").replaceChildren(
    el("div", { class: "riga" }, pallino(sem.valutazione.livello), el("strong", {}, `Lavorabilità e semina: ${sem.valutazione.etichetta}`)),
    el("div", { class: "nota" }, sem.valutazione.motivo),
    el(
      "div",
      { class: "riga badge-termici-suolo" },
      el("span", { class: `pill-suolo ${sem.autunno_vernine ? "ok" : "ko"}` }, `${sem.autunno_vernine ? "✓" : "✕"} Autunno-vernine (> 8 °C a 6 cm)`),
      el("span", { class: `pill-suolo ${sem.primaverili ? "ok" : "ko"}` }, `${sem.primaverili ? "✓" : "✕"} Primaverili-estive (> 12 °C a 6 cm)`),
    ),
    el(
      "div",
      { class: "giorni-semina" },
      ...prossimi.map((g) =>
        el("span", { title: g.semina.valutazione.motivo }, pallino(g.semina.valutazione.livello), `${giornoBreve(g.data)} ${g.semina.valutazione.etichetta}`),
      ),
    ),
  );
  graficoSuolo(r);
}
