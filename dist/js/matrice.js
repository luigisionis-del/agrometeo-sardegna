// Matrice decisionale a semaforo (72 ore) con icone SVG descrittive e sintesi delle allerte.
import { SIMBOLO, dataLocale, el, giorno, ora, pallino, quando } from "./formato.js";
import { svg } from "./illustrazioni.js";
import { TV } from "./tv.js";

let report;
let selezionata;

export function initMatrice() {
  const box = document.getElementById("matrice");
  const mostra = (td) => {
    if (!td?.dataset.r) return;
    const riga = report.matrice.righe[Number(td.dataset.r)];
    const cella = riga.celle[Number(td.dataset.i)];
    document.getElementById("dettaglio-cella").replaceChildren(
      pallino(cella.livello),
      " ",
      el("strong", {}, `${riga.titolo} · ${quando(report.matrice.ore[Number(td.dataset.i)])}: ${cella.etichetta}`),
      ` — ${cella.motivo}`,
      riga.regola ? el("span", { class: "regola-cella" }, ` (${riga.regola})`) : null,
    );
    return true;
  };
  box.addEventListener("mouseover", (e) => mostra(e.target.closest("td")));
  box.addEventListener("focusin", (e) => mostra(e.target.closest("td")));
  box.addEventListener("click", (e) => {
    const td = e.target.closest("td");
    if (!mostra(td)) return;
    selezionata?.classList.remove("selezionata");
    selezionata = td;
    td.classList.add("selezionata");
  });
}

export function renderMatrice(r) {
  report = r;
  selezionata = null;
  const { ore, righe } = r.matrice;

  const giorni = [];
  ore.forEach((o, i) => {
    const d = o.slice(0, 10);
    if (giorni.at(-1)?.d === d) giorni.at(-1).n++;
    else giorni.push({ d, n: 1, i });
  });

  const tabella = el(
    "table",
    {},
    el(
      "thead",
      {},
      el("tr", {}, el("th", { class: "riga" }, "Indicatore"), ...giorni.map((g) => el("th", { class: "giorno", colspan: g.n }, giorno(g.d)))),
      el(
        "tr",
        {},
        el("th", { class: "riga" }, "Ora"),
        ...ore.map((o) => {
          const h = dataLocale(o).getHours();
          return el("th", { class: "ora" }, h % 3 === 0 ? String(h).padStart(2, "0") : "");
        }),
      ),
    ),
    el(
      "tbody",
      {},
      ...righe.map((riga, ri) =>
        el(
          "tr",
          {},
          el(
            "th",
            { class: "riga", scope: "row", title: riga.regola || riga.titolo },
            el("span", { class: "intestazione-riga-matrice" }, riga.icona ? svg(riga.icona, "icona-matrice") : null, el("span", {}, riga.titolo)),
          ),
          ...riga.celle.map((c, i) =>
            el(
              "td",
              {
                class: `s-${c.livello}${ore[i].endsWith("T00:00:00") && i > 0 ? " mezzanotte" : ""}`,
                "data-r": ri,
                "data-i": i,
                tabindex: TV ? 0 : null,
                title: `${ora(ore[i])} · ${c.etichetta} — ${c.motivo}`,
              },
              SIMBOLO[c.livello],
            ),
          ),
        ),
      ),
    ),
  );
  document.getElementById("matrice").replaceChildren(tabella);
}

export function renderAllerte(r) {
  const lista = document.getElementById("allerte");
  if (!r.allerte.length) {
    lista.replaceChildren(
      el(
        "li",
        { class: "l-verde" },
        pallino("verde"),
        el("div", {}, el("strong", {}, "Nessuna criticità prevista nelle prossime 72 ore"), el("div", { class: "motivo" }, "Condizioni regolari per le operazioni di campo.")),
      ),
    );
    return;
  }
  lista.replaceChildren(
    ...r.allerte.map((a) => {
      const periodo = a.ore > 0
        ? `${quando(a.inizio)} → ${a.inizio.slice(0, 10) === a.fine.slice(0, 10) ? ora(a.fine) : quando(a.fine)} (${a.ore} h)${a.episodi > 1 ? ` · ${a.episodi} episodi` : ""}`
        : "";
      return el(
        "li",
        { class: `l-${a.livello}` },
        pallino(a.livello),
        el("div", {}, el("strong", {}, a.titolo), periodo ? el("span", { class: "quando" }, periodo) : null, el("div", { class: "motivo" }, a.motivo)),
      );
    }),
  );
}
