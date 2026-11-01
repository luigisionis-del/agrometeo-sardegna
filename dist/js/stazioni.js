// Calibrazione locale: confronto tra pioggia osservata (CSV stazione) e modelli.
import { api } from "./api.js";
import { el, giorno, num } from "./formato.js";
import { stato } from "./stato.js";

export function initStazioni() {
  document.getElementById("stazione-csv").addEventListener("change", async (e) => {
    const file = e.target.files[0];
    e.target.value = "";
    if (!file) return;
    const box = document.getElementById("confronto");
    const nome = document.getElementById("stazione-nome").value.trim() || file.name;
    const { lat, lon, nome: luogo } = stato.spazio.luogo_attivo;
    box.replaceChildren(el("p", { class: "nota" }, "Confronto in corso…"));
    try {
      const c = await api.confrontaStazione(nome, await file.text(), lat, lon);
      renderConfronto(c, luogo);
    } catch (err) {
      box.replaceChildren(el("p", { class: "errore" }, String(err)));
    }
  });
}

function stat(etichetta, valore) {
  return el("div", {}, etichetta, el("b", {}, valore));
}

function renderConfronto(c, luogo) {
  const righe = [...c.righe].reverse().slice(0, 90);
  document.getElementById("confronto").replaceChildren(
    el("p", {}, `Stazione «${c.stazione}» confrontata con i modelli su «${luogo}».`),
    el(
      "div",
      { class: "statistiche" },
      stat("Giorni confrontati", c.giorni_confrontati),
      stat("Pioggia osservata", `${num(c.totale_osservato)} mm`),
      stat("Pioggia modello", `${num(c.totale_modello)} mm`),
      stat("Bias modello", c.bias_percento == null ? "—" : `${c.bias_percento > 0 ? "+" : ""}${num(c.bias_percento, 0)} %`),
      stat("Errore medio (MAE)", `${num(c.mae)} mm/g`),
      stat("Concordanza giorni ≥ 1 mm", `${num(c.concordanza_giorni_piovosi, 0)} %`),
    ),
    el(
      "div",
      { class: "scorri" },
      el(
        "table",
        { class: "dati" },
        el("thead", {}, el("tr", {}, el("th", {}, "Giorno"), el("th", {}, "Osservato mm"), el("th", {}, "Modello mm"), el("th", {}, "Differenza"))),
        el(
          "tbody",
          {},
          ...righe.map((r) =>
            el(
              "tr",
              {},
              el("td", {}, giorno(r.data)),
              el("td", {}, num(r.osservato)),
              el("td", {}, num(r.modello)),
              el("td", {}, r.modello == null ? "—" : num(r.modello - r.osservato)),
            ),
          ),
        ),
      ),
    ),
    c.righe.length > righe.length ? el("p", { class: "nota" }, `Mostrati gli ultimi ${righe.length} giorni su ${c.righe.length}.`) : null,
  );
}
