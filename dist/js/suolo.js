// Soil profiler: stratigrafia con termometri e barre di saturazione idrica.
import { el, giornoBreve, num, pallino } from "./formato.js";
import { graficoSuolo } from "./grafici.js";

const T_MIN = -5;
const T_MAX = 35;
const U_MAX = 0.5;

// Per ogni profondità di temperatura, lo strato di umidità che la contiene.
const STRATO_PER_PROFONDITA = [0, 2, 3, 4];
const DESCRIZIONE = ["superficie / crosta", "letto di semina", "radici erbacee", "radici arboree / vite"];

const percento = (v, min, max) => `${Math.min(100, Math.max(0, ((v - min) / (max - min)) * 100))}%`;

function barraTermica(t) {
  return el(
    "div",
    { class: "barra termo", title: `Temperatura ${num(t)} °C` },
    el("span", { style: `width:${percento(t, T_MIN, T_MAX)}` }),
    el("div", { class: "testo" }, `${num(t)} °C`),
  );
}

function barraAcqua(u, strato, suolo) {
  return el(
    "div",
    {
      class: "barra acqua",
      title: `Umidità ${strato[0]}–${strato[1]} cm: ${num(u, 3)} m³/m³ (appassimento ${suolo.punto_appassimento}, capacità di campo ${suolo.capacita_campo})`,
    },
    el("span", { style: `width:${percento(u, 0, U_MAX)}` }),
    el("i", { class: "tacca", style: `left:${percento(suolo.punto_appassimento, 0, U_MAX)}` }),
    el("i", { class: "tacca", style: `left:${percento(suolo.capacita_campo, 0, U_MAX)}` }),
    el("div", { class: "testo" }, `${num(u, 3)} m³/m³ · ${strato[0]}–${strato[1]} cm`),
  );
}

export function renderSuolo(r) {
  const s = r.suolo;
  const suolo = r.parametri.suolo;
  document.getElementById("profilo-suolo").replaceChildren(
    ...s.profondita_cm.map((p, i) => {
      const k = STRATO_PER_PROFONDITA[i];
      return el(
        "div",
        { class: "strato" },
        el("div", { class: "prof" }, `${p} cm`, el("small", {}, DESCRIZIONE[i])),
        barraTermica(s.temperature[i]),
        barraAcqua(s.umidita[k], s.strati_cm[k], suolo),
      );
    }),
  );

  const sem = s.semina;
  const prossimi = r.giorni.filter((g) => g.previsto).slice(0, 6);
  document.getElementById("semina").replaceChildren(
    el("div", { class: "riga" }, pallino(sem.valutazione.livello), el("strong", {}, `Lavorabilità/semina: ${sem.valutazione.etichetta}`)),
    el("div", { class: "nota" }, sem.valutazione.motivo),
    el(
      "div",
      { class: "riga" },
      `Cereali e foraggere autunno-vernine (> 8 °C): ${sem.autunno_vernine ? "✓ sì" : "✕ no"}`,
      " · ",
      `primaverili (> 12 °C): ${sem.primaverili ? "✓ sì" : "✕ no"}`,
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
