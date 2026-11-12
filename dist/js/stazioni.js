// Calibrazione locale: confronto tra pioggia osservata (CSV stazione o esempio ARPAS) e modelli.
import { api } from "./api.js";
import { el, giorno, num } from "./formato.js";
import { stato } from "./stato.js";

async function eseguiConfronto(nome, testoCsv) {
  const box = document.getElementById("confronto");
  const { lat, lon, nome: luogo } = stato.spazio.luogo_attivo;
  box.replaceChildren(el("p", { class: "nota" }, "Confronto con l'archivio storico del modello in corso…"));
  try {
    const c = await api.confrontaStazione(nome, testoCsv, lat, lon);
    renderConfronto(c, luogo);
  } catch (err) {
    box.replaceChildren(el("p", { class: "errore" }, String(err)));
  }
}

export function initStazioni() {
  document.getElementById("stazione-csv").addEventListener("change", async (e) => {
    const file = e.target.files[0];
    e.target.value = "";
    if (!file) return;
    const nome = document.getElementById("stazione-nome").value.trim() || file.name.replace(/\.csv$/i, "");
    await eseguiConfronto(nome, await file.text());
  });

  const btnEsempio = document.getElementById("stazione-esempio");
  if (btnEsempio) {
    btnEsempio.addEventListener("click", async () => {
      const campoNome = document.getElementById("stazione-nome");
      if (!campoNome.value.trim()) campoNome.value = "Esempio dimostrativo (dati sintetici, 30 gg)";
      const csv = await api.csvEsempioStazione();
      await eseguiConfronto(campoNome.value.trim(), csv);
    });
  }
}

function stat(etichetta, valore, nota) {
  return el("div", { title: nota || etichetta }, etichetta, el("b", {}, valore));
}

function barreGiorniPiovosi(righe) {
  const piovosi = righe.filter((r) => r.osservato >= 0.5 || (r.modello ?? 0) >= 0.5).slice(0, 14);
  if (!piovosi.length) return null;
  const maxMm = Math.max(10, ...piovosi.flatMap((r) => [r.osservato, r.modello ?? 0]));
  return el(
    "div",
    { class: "confronto-barre" },
    el("h3", {}, "Confronto visivo degli episodi piovosi (Osservato vs Modello)"),
    el(
      "div",
      { class: "griglia-barre-pioggia" },
      ...piovosi.map((r) => {
        const ossPct = Math.min(100, (r.osservato / maxMm) * 100);
        const modPct = Math.min(100, ((r.modello ?? 0) / maxMm) * 100);
        return el(
          "div",
          { class: "riga-barra-pioggia" },
          el("div", { class: "data-pioggia" }, giorno(r.data)),
          el(
            "div",
            { class: "doppio-istogramma" },
            el(
              "div",
              { class: "istogramma-voce" },
              el("span", { class: "barra-oss", style: `width:${ossPct}%` }),
              el("small", {}, `Stazione: ${num(r.osservato)} mm`),
            ),
            el(
              "div",
              { class: "istogramma-voce" },
              el("span", { class: "barra-mod", style: `width:${modPct}%` }),
              el("small", {}, `Modello: ${num(r.modello)} mm`),
            ),
          ),
        );
      }),
    ),
  );
}

function renderConfronto(c, luogo) {
  const righe = [...c.righe].reverse().slice(0, 90);
  document.getElementById("confronto").replaceChildren(
    el("p", { class: "titolo-confronto" }, `Stazione «${c.stazione}» confrontata con i modelli su «${luogo}».`),
    el(
      "div",
      { class: "statistiche" },
      stat("Giorni confrontati", c.giorni_confrontati, "Numero di giorni validi sovrapposti tra CSV e archivio modello"),
      stat("Pioggia osservata", `${num(c.totale_osservato)} mm`, "Cumulata misurata dal pluviometro locale"),
      stat("Pioggia modello", `${num(c.totale_modello)} mm`, "Cumulata stimata dal modello meteorologico sulla cella"),
      stat("Bias modello", c.bias_percento == null ? "—" : `${c.bias_percento > 0 ? "+" : ""}${num(c.bias_percento, 0)} %`, "Sovrastima (+) o sottostima (−) percentuale del modello"),
      stat("Errore medio (MAE)", `${num(c.mae)} mm/g`, "Scarto assoluto medio giornaliero in millimetri"),
      stat("Concordanza giorni ≥ 1 mm", `${num(c.concordanza_giorni_piovosi, 0)} %`, "Percentuale di giorni in cui stazione e modello concordano su pioggia/secco"),
    ),
    barreGiorniPiovosi(righe),
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
