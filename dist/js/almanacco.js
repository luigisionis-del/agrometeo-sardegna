// Almanacco colturale: indice per sezioni, schede colturali, consigli dinamici dal report.
import { AVVERTENZE, FONTI, MESI, SCHEDE, SEZIONI } from "./almanacco/dati.js";
import { consigliOggi, contesto, valutaColtura } from "./almanacco/analisi.js";
import { el, num, pallino } from "./formato.js";

const CHIAVE_SEZIONE = "almanacco.sezione";
const $ = (id) => document.getElementById(id);

const stato = {
  sezione: leggiPreferenza() ?? "oggi",
  ricerca: "",
  ctx: null,
  aperte: new Set(),
};

function leggiPreferenza() {
  try {
    const v = localStorage.getItem(CHIAVE_SEZIONE);
    return SEZIONI.some((s) => s.id === v) ? v : null;
  } catch {
    return null;
  }
}

function salvaPreferenza(id) {
  try {
    localStorage.setItem(CHIAVE_SEZIONE, id);
  } catch {
    /* archiviazione non disponibile: si riparte da «Oggi in campo» */
  }
}

export function initAlmanacco() {
  const cerca = $("cerca-almanacco");
  let timer;
  cerca.addEventListener("input", () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      stato.ricerca = cerca.value.trim().toLowerCase();
      render();
    }, 150);
  });
  renderIndice();
  render();
}

/** Nuovo report: aggiorna i consigli dinamici e gli indicatori delle schede. */
export function aggiornaAlmanacco(r) {
  try {
    stato.ctx = contesto(r);
  } catch (e) {
    console.error("Analisi almanacco:", e);
    stato.ctx = null;
  }
  render();
}

/** Apre la scheda indicata nella sua sezione. */
export function apriScheda(id) {
  const s = SCHEDE.find((x) => x.id === id);
  if (!s) return;
  stato.ricerca = "";
  $("cerca-almanacco").value = "";
  stato.aperte.add(id);
  vaiASezione(s.sezione, false);
  requestAnimationFrame(() => document.getElementById(`scheda-${id}`)?.scrollIntoView({ behavior: "smooth", block: "start" }));
}

function vaiASezione(id, inCima = true) {
  stato.sezione = id;
  salvaPreferenza(id);
  renderIndice();
  render();
  if (inCima) window.scrollTo({ top: 0 });
}

// ------------------------------------------------------------------ indice

function renderIndice() {
  $("indice-almanacco").replaceChildren(
    ...SEZIONI.map((s) => {
      const n = SCHEDE.filter((x) => x.sezione === s.id).length;
      return el(
        "li",
        {},
        el(
          "button",
          { class: "voce-indice", type: "button", "aria-current": s.id === stato.sezione && !stato.ricerca ? "true" : null, onclick: () => vaiASezione(s.id) },
          el("span", { class: "em", "aria-hidden": "true" }, s.icona),
          s.titolo,
          n ? el("span", { class: "conteggio" }, n) : null,
        ),
      );
    }),
  );
}

// ------------------------------------------------------------------ contenuto

function render() {
  const box = $("contenuto-almanacco");
  if (stato.ricerca) {
    renderIndice();
    box.replaceChildren(...renderRicerca(stato.ricerca));
    return;
  }
  const sezione = SEZIONI.find((s) => s.id === stato.sezione) ?? SEZIONI[0];
  const parti = [introSezione(sezione)];
  if (sezione.id === "oggi") parti.push(...renderOggi());
  else if (sezione.id === "fonti") parti.push(renderFonti());
  else {
    const schede = SCHEDE.filter((s) => s.sezione === sezione.id);
    const confronto = tabellaConfronto(schede);
    if (confronto) parti.push(confronto);
    parti.push(el("div", { class: "griglia-colture" }, ...schede.map(scheda)));
  }
  box.replaceChildren(...parti);
}

function introSezione(s) {
  return el(
    "section",
    { class: "pannello intro-sezione" },
    el("h2", {}, el("span", { "aria-hidden": "true" }, s.icona), s.titolo),
    el("p", {}, s.intro),
    s.principi ? el("div", { class: "principi" }, ...s.principi.map(([t, d]) => el("div", { class: "principio" }, el("b", {}, t), d))) : null,
  );
}

function testoScheda(s) {
  const parti = [s.nome, s.latino, s.famiglia, s.sintesi, ...(s.dati ?? []).flat(), ...(s.blocchi ?? []).flatMap((b) => [b.titolo, ...b.punti])];
  if (s.tabella) parti.push(...s.tabella.righe.flat());
  return parti.filter(Boolean).join(" ").toLowerCase();
}

function renderRicerca(q) {
  const parole = q.split(/\s+/).filter(Boolean);
  const trovate = SCHEDE.filter((s) => {
    const t = testoScheda(s);
    return parole.every((p) => t.includes(p));
  });
  const titolo = el(
    "section",
    { class: "pannello intro-sezione" },
    el("h2", {}, `Risultati per «${q}»`),
    el("p", {}, trovate.length ? `${trovate.length} schede trovate.` : "Nessuna scheda corrisponde alla ricerca."),
  );
  return [titolo, el("div", { class: "griglia-colture" }, ...trovate.map(scheda))];
}

// ------------------------------------------------------------------ oggi in campo

function etichettaColtura(c) {
  return el("button", { type: "button", class: `etichetta-coltura ${c.classe ?? ""}`, title: c.titolo ?? "", onclick: () => apriScheda(c.id) }, c.nome);
}

function cartaConsiglio(c) {
  return el(
    "article",
    { class: `consiglio l-${c.livello}` },
    el("div", { class: "testa" }, pallino(c.livello), el("h3", {}, c.titolo)),
    el("div", { class: "dato" }, c.dato),
    ...c.testi.map((t) => el("p", {}, t)),
    c.colture?.length ? el("div", { class: "etichette" }, ...c.colture.map(etichettaColtura)) : null,
    ...(c.dopo ?? []).map((d) => el("div", {}, el("p", { style: "margin-top:8px" }, d.testo), el("div", { class: "etichette" }, ...d.colture.map(etichettaColtura)))),
    el(
      "div",
      { class: "perche" },
      el("strong", {}, "Perché: "),
      c.perche,
      c.link ? [" ", el("a", { href: "#", onclick: (e) => { e.preventDefault(); apriScheda(c.link); } }, "Approfondisci →")] : null,
    ),
  );
}

function renderOggi() {
  const parti = [];
  if (!stato.ctx) {
    parti.push(el("section", { class: "pannello" }, el("p", { class: "vuoto" }, "I consigli del giorno compariranno appena saranno disponibili i dati della località.")));
  } else {
    parti.push(el("section", { class: "griglia-consigli" }, ...consigliOggi(stato.ctx, SCHEDE).map(cartaConsiglio)));
  }
  parti.push(calendarioMese(), calendarioAnnuale());
  return parti;
}

const meseCorrente = () => stato.ctx?.mese ?? new Date().getMonth() + 1;
const NOMI_MESI = ["gennaio", "febbraio", "marzo", "aprile", "maggio", "giugno", "luglio", "agosto", "settembre", "ottobre", "novembre", "dicembre"];
const OPERAZIONI = [
  ["semina", "Semina in campo"],
  ["semenzaio", "Semina in semenzaio"],
  ["trapianto", "Trapianto / impianto"],
  ["raccolta", "Raccolta"],
  ["utilizzo", "Pascolo / sfalcio"],
];

function calendarioMese() {
  const m = meseCorrente();
  const colonne = OPERAZIONI.map(([chiave, titolo]) => {
    const colture = SCHEDE.filter((s) => s.calendario?.[chiave]?.includes(m));
    return colture.length
      ? el("div", { class: "blocco" }, el("h4", {}, titolo), el("div", { class: "etichette" }, ...colture.map((s) => etichettaColtura({ id: s.id, nome: s.nome }))))
      : null;
  }).filter(Boolean);
  return el(
    "section",
    { class: "pannello" },
    el("h2", {}, `Lavori di ${NOMI_MESI[m - 1]} in Sardegna`),
    el("div", { class: "blocchi-coltura" }, ...colonne),
  );
}

function calendarioAnnuale() {
  const m = meseCorrente();
  const gruppi = SEZIONI.filter((s) => SCHEDE.some((x) => x.sezione === s.id && x.calendario));
  const righe = [];
  for (const g of gruppi) {
    righe.push(el("tr", {}, el("th", { class: "gruppo", colspan: 13 }, g.titolo)));
    for (const s of SCHEDE.filter((x) => x.sezione === g.id && x.calendario)) {
      righe.push(
        el(
          "tr",
          {},
          el("th", { class: "nome", scope: "row", onclick: () => apriScheda(s.id) }, s.nome),
          ...MESI.map((_, i) => el("td", { class: i + 1 === m ? "corrente" : null, title: descriviMese(s, i + 1) }, ...barreMese(s, i + 1))),
        ),
      );
    }
  }
  return el(
    "section",
    { class: "pannello" },
    el("h2", {}, "Calendario colturale annuale"),
    el(
      "div",
      { class: "calendario" },
      el(
        "table",
        {},
        el("thead", {}, el("tr", {}, el("th", {}), ...MESI.map((n, i) => el("th", { class: i + 1 === m ? "mese-corrente" : null }, n)))),
        el("tbody", {}, ...righe),
      ),
    ),
    legendaCalendario(),
  );
}

function barreMese(s, m) {
  const c = s.calendario;
  const sopra = ["semina", "trapianto", "semenzaio"].find((k) => c[k]?.includes(m));
  const sotto = ["raccolta", "utilizzo"].find((k) => c[k]?.includes(m));
  return [sopra ? el("i", { class: sopra === "semenzaio" ? "trapianto" : sopra }) : null, sotto ? el("i", { class: sotto }) : null];
}

function descriviMese(s, m) {
  const voci = OPERAZIONI.filter(([k]) => s.calendario[k]?.includes(m)).map(([, t]) => t.toLowerCase());
  return `${s.nome}, ${NOMI_MESI[m - 1]}: ${voci.length ? voci.join(", ") : "nessuna operazione principale"}`;
}

function legendaCalendario() {
  return el(
    "div",
    { class: "legenda-calendario" },
    el("span", {}, el("i", { style: "background:var(--serie-3)" }), "Semina in campo"),
    el("span", {}, el("i", { style: "background:#6cc49a" }), "Trapianto o semenzaio"),
    el("span", {}, el("i", { style: "background:var(--serie-4)" }), "Raccolta"),
    el("span", {}, el("i", { style: "background:#e8b84a" }), "Pascolo / sfalcio"),
  );
}

// ------------------------------------------------------------------ schede

function kcTesto(kc) {
  return kc.map((v) => num(v, 2)).join(" · ");
}

function tabellaConfronto(schede) {
  const conDati = schede.filter((s) => s.tSuolo && s.kc);
  if (conDati.length < 2) return null;
  const epoca = (s) => {
    const c = s.calendario ?? {};
    const mesi = [...new Set([...(c.semina ?? []), ...(c.trapianto ?? [])])];
    return mesi.length ? mesi.map((m) => MESI[m - 1]).join(", ") : "—";
  };
  return el(
    "section",
    { class: "pannello" },
    el("h2", {}, "Confronto rapido"),
    el(
      "div",
      { class: "scorri" },
      el(
        "table",
        { class: "tabella-almanacco" },
        el("thead", {}, el("tr", {}, ...["Coltura", "Semina / trapianto in campo", "T suolo min · ottimale", "Kc ini · metà · fine", "Danni da gelo"].map((h) => el("th", {}, h)))),
        el(
          "tbody",
          {},
          ...conDati.map((s) =>
            el(
              "tr",
              {},
              el("td", {}, el("a", { href: "#", onclick: (e) => { e.preventDefault(); apriScheda(s.id); } }, s.nome)),
              el("td", {}, epoca(s)),
              el("td", {}, `${s.tSuolo.min} °C · ${s.tSuolo.opt} °C${s.tSuolo.max != null ? ` (max ${s.tSuolo.max} °C)` : ""}`),
              el("td", {}, kcTesto(s.kc)),
              el("td", {}, s.gelo != null ? `sotto ${s.gelo} °C` : "—"),
            ),
          ),
        ),
      ),
    ),
    el("p", { class: "nota" }, "Kc da FAO-56 (o da letteratura dove indicato): moltiplicato per ET0 dà il consumo idrico della coltura. Impostalo nei Parametri per vedere il bilancio idrico della coltura."),
  );
}

function strisciaMesi(s) {
  const m = meseCorrente();
  return el(
    "div",
    { class: "striscia-mesi", "aria-label": "Calendario colturale" },
    ...MESI.map((n, i) => el("div", { class: i + 1 === m ? "corrente" : null, title: descriviMese(s, i + 1) }, ...barreMese(s, i + 1), n)),
  );
}

function blocco(b) {
  const icona = b.tipo === "scienza" ? "🔬" : b.tipo === "attenzione" ? "⚠️" : null;
  return el(
    "div",
    { class: `blocco ${b.tipo ?? ""}` },
    el("h4", {}, icona ? el("span", { "aria-hidden": "true" }, icona) : null, b.titolo),
    el("ul", {}, ...b.punti.map((p) => el("li", {}, p))),
  );
}

function tabellaScheda(t) {
  return el(
    "div",
    { class: "blocco", style: "grid-column:1/-1" },
    el("h4", {}, t.titolo),
    el(
      "div",
      { class: "scorri" },
      el(
        "table",
        { class: "tabella-almanacco" },
        el("thead", {}, el("tr", {}, ...t.intestazioni.map((h) => el("th", {}, h)))),
        el("tbody", {}, ...t.righe.map((r) => el("tr", {}, ...r.map((c) => el("td", {}, c))))),
      ),
    ),
  );
}

function scheda(s) {
  if (s.legale) {
    return el(
      "section",
      { class: "riquadro-legale", id: `scheda-${s.id}` },
      el("h3", {}, `⚖️ ${s.nome}`),
      el("p", {}, s.sintesi),
      ...s.blocchi.map((b) => el("div", {}, el("strong", {}, b.titolo), el("ul", {}, ...b.punti.map((p) => el("li", {}, p))))),
    );
  }
  const chip = valutaColtura(s, stato.ctx);
  const dati = [...(s.dati ?? [])];
  if (s.kc && !dati.some(([k]) => k === "Kc")) dati.push(["Kc ini · metà · fine", kcTesto(s.kc)]);
  if (s.gelo != null) dati.push(["Danni da gelo", `sotto ${s.gelo} °C`]);

  const d = el(
    "details",
    { class: "pannello coltura", id: `scheda-${s.id}`, open: stato.aperte.has(s.id) },
    el(
      "summary",
      {},
      el(
        "div",
        { class: "titolo-coltura" },
        el("h3", {}, s.nome),
        s.latino ? el("span", { class: "latino" }, s.latino) : null,
        s.famiglia ? el("span", { class: "famiglia" }, s.famiglia) : null,
      ),
      el("span", { class: "freccia", "aria-hidden": "true" }, "▶"),
      el("p", { class: "sintesi" }, s.sintesi),
      chip.length ? el("div", { class: "stato-live" }, ...chip.map((c) => el("span", { class: "chip" }, pallino(c.livello), c.testo))) : null,
    ),
    el(
      "div",
      { class: "corpo-coltura" },
      dati.length ? el("dl", { class: "dati-rapidi" }, ...dati.map(([k, v]) => el("div", {}, el("dt", {}, k), el("dd", {}, v)))) : null,
      s.calendario ? strisciaMesi(s) : null,
      s.calendario ? legendaCalendario() : null,
      el("div", { class: "blocchi-coltura", style: s.calendario ? "margin-top:14px" : null }, s.tabella ? tabellaScheda(s.tabella) : null, ...(s.blocchi ?? []).map(blocco)),
    ),
  );
  d.addEventListener("toggle", () => (d.open ? stato.aperte.add(s.id) : stato.aperte.delete(s.id)));
  return d;
}

// ------------------------------------------------------------------ fonti

function renderFonti() {
  return el(
    "div",
    { class: "griglia-colture" },
    el(
      "section",
      { class: "pannello" },
      el("h2", {}, "Come l'almanacco usa i dati meteo"),
      el(
        "ul",
        { class: "fonti" },
        el("li", {}, "Temperatura del suolo a 6 cm → confronto con la temperatura minima di germinazione o di trapianto di ogni coltura."),
        el("li", {}, "Umidità degli strati 3–9 cm e 9–27 cm → stato di tempera per semina e aratura (soglia: 40 % dell'acqua disponibile, come nel cruscotto)."),
        el("li", {}, "Temperatura minima delle prossime 72 ore → confronto con la soglia di danno da gelo delle colture presumibilmente in campo nel mese."),
        el("li", {}, "Pioggia prevista e passata → finestre per concimazione azotata, semina degli erbai, fienagione e rischio di erosione."),
        el("li", {}, "Temperatura e umidità oraria → THI per gli ovini, periodo di Smith per la peronospora, rugiada mattutina per il meteorismo."),
        el("li", {}, "ET0 e pioggia efficace previste → fabbisogno irriguo di una coltura a pieno sviluppo."),
      ),
    ),
    el("section", { class: "pannello" }, el("h2", {}, "Fonti principali"), el("ol", { class: "fonti" }, ...FONTI.map((f) => el("li", {}, f)))),
    el("section", { class: "pannello" }, el("h2", {}, "Avvertenze"), el("ul", { class: "fonti" }, ...AVVERTENZE.map((a) => el("li", {}, a)))),
  );
}
