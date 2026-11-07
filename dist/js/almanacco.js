// Almanacco colturale: rendering delle sezioni, schede colturali, illustrazioni SVG
// e consigli dinamici calcolati interamente dal motore Rust (`src-tauri/src/almanacco/`).
import { el, num, pallino } from "./formato.js";
import { svg } from "./illustrazioni.js";
import { stato as statoGlobale } from "./stato.js";

const CHIAVE_SEZIONE = "almanacco.sezione";
const $ = (id) => document.getElementById(id);

const OPERAZIONI = [
  ["semina", "Semina in campo"],
  ["semenzaio", "Semina in semenzaio"],
  ["trapianto", "Trapianto / impianto"],
  ["raccolta", "Raccolta"],
  ["utilizzo", "Pascolo / sfalcio"],
];

const ui = {
  sezione: "oggi",
  ricerca: "",
  analisi: null,
  aperte: new Set(),
};

const cat = () =>
  statoGlobale.catalogo ?? {
    sezioni: [],
    schede: [],
    mesi: ["Gen", "Feb", "Mar", "Apr", "Mag", "Giu", "Lug", "Ago", "Set", "Ott", "Nov", "Dic"],
    nomi_mesi: ["gennaio", "febbraio", "marzo", "aprile", "maggio", "giugno", "luglio", "agosto", "settembre", "ottobre", "novembre", "dicembre"],
    fonti: [],
    avvertenze: [],
    illustrazioni: {},
  };

function leggiPreferenza() {
  try {
    const v = localStorage.getItem(CHIAVE_SEZIONE);
    return cat().sezioni.some((s) => s.id === v) ? v : null;
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
  ui.sezione = leggiPreferenza() ?? "oggi";
  const cerca = $("cerca-almanacco");
  let timer;
  cerca.addEventListener("input", () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      ui.ricerca = cerca.value.trim().toLowerCase();
      render();
    }, 150);
  });
  renderIndice();
  render();
}

/** Nuovo report ricevuto dal backend Rust: aggiorna l'analisi e gli indicatori live. */
export function aggiornaAlmanacco(r) {
  ui.analisi = r?.almanacco ?? null;
  render();
}

/** Apre la scheda indicata nella sua sezione e scorre fino ad essa. */
export function apriScheda(id) {
  const s = cat().schede.find((x) => x.id === id);
  if (!s) return;
  ui.ricerca = "";
  $("cerca-almanacco").value = "";
  ui.aperte.add(id);
  vaiASezione(s.sezione, false);
  requestAnimationFrame(() => document.getElementById(`scheda-${id}`)?.scrollIntoView({ behavior: "smooth", block: "start" }));
}

function vaiASezione(id, inCima = true) {
  ui.sezione = id;
  salvaPreferenza(id);
  renderIndice();
  render();
  if (inCima) window.scrollTo({ top: 0 });
}

// ------------------------------------------------------------------ indice laterale

function renderIndice() {
  const { sezioni, schede } = cat();
  $("indice-almanacco").replaceChildren(
    ...sezioni.map((s) => {
      const n = schede.filter((x) => x.sezione === s.id).length;
      return el(
        "li",
        {},
        el(
          "button",
          {
            class: "voce-indice",
            type: "button",
            "aria-current": s.id === ui.sezione && !ui.ricerca ? "true" : null,
            onclick: () => vaiASezione(s.id),
          },
          el("span", { class: "em", "aria-hidden": "true" }, s.icona),
          s.titolo,
          n ? el("span", { class: "conteggio" }, n) : null,
        ),
      );
    }),
  );
}

// ------------------------------------------------------------------ contenuto principale

function render() {
  const box = $("contenuto-almanacco");
  const { sezioni, schede } = cat();
  if (!sezioni.length) return;

  if (ui.ricerca) {
    renderIndice();
    box.replaceChildren(...renderRicerca(ui.ricerca));
    return;
  }
  const sezione = sezioni.find((s) => s.id === ui.sezione) ?? sezioni[0];
  const parti = [introSezione(sezione)];
  if (sezione.id === "oggi") parti.push(...renderOggi());
  else if (sezione.id === "fonti") parti.push(renderFonti());
  else {
    const schedeSezione = schede.filter((s) => s.sezione === sezione.id);
    const confronto = tabellaConfronto(schedeSezione);
    if (confronto) parti.push(confronto);
    parti.push(el("div", { class: "griglia-colture" }, ...schedeSezione.map(scheda)));
  }
  box.replaceChildren(...parti);
}

function introSezione(s) {
  return el(
    "section",
    { class: "pannello intro-sezione" },
    el(
      "div",
      { class: "intro-sezione-hero" },
      el(
        "div",
        { class: "intro-sezione-testo" },
        el("h2", {}, el("span", { "aria-hidden": "true" }, s.icona), s.titolo),
        el("p", {}, s.intro),
      ),
      s.illustrazione ? svg(s.illustrazione, "illustrazione-sezione") : null,
    ),
    s.principi?.length ? el("div", { class: "principi" }, ...s.principi.map(([t, d]) => el("div", { class: "principio" }, el("b", {}, t), d))) : null,
  );
}

function testoScheda(s) {
  const parti = [s.nome, s.latino, s.famiglia, s.sintesi, ...(s.dati ?? []).flat(), ...(s.blocchi ?? []).flatMap((b) => [b.titolo, ...b.punti])];
  if (s.tabella) parti.push(...s.tabella.righe.flat());
  return parti.filter(Boolean).join(" ").toLowerCase();
}

function renderRicerca(q) {
  const parole = q.split(/\s+/).filter(Boolean);
  const trovate = cat().schede.filter((s) => {
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
    el(
      "div",
      { class: "testa-consiglio-con-icona" },
      c.illustrazione ? svg(c.illustrazione, "icona-consiglio") : null,
      el(
        "div",
        { class: "testa-consiglio-testo" },
        el("div", { class: "testa" }, pallino(c.livello), el("h3", {}, c.titolo)),
        el("div", { class: "dato" }, c.dato),
      ),
    ),
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
  if (!ui.analisi) {
    parti.push(el("section", { class: "pannello" }, el("p", { class: "vuoto" }, "I consigli del giorno compariranno appena saranno disponibili i dati della località.")));
  } else {
    parti.push(el("section", { class: "griglia-consigli" }, ...ui.analisi.consigli.map(cartaConsiglio)));
  }
  parti.push(calendarioMese(), calendarioAnnuale());
  return parti;
}

const meseCorrente = () => ui.analisi?.contesto?.mese ?? new Date().getMonth() + 1;

function calendarioMese() {
  const m = meseCorrente();
  const { nomi_mesi, schede } = cat();
  const gruppi =
    ui.analisi?.lavori_mese ??
    OPERAZIONI.map(([chiave, titolo]) => ({
      chiave,
      titolo,
      colture: schede.filter((s) => s.calendario?.[chiave]?.includes(m)).map((s) => ({ id: s.id, nome: s.nome })),
    })).filter((g) => g.colture.length > 0);

  return el(
    "section",
    { class: "pannello" },
    el("h2", {}, `Lavori di ${nomi_mesi[m - 1]} in Sardegna`),
    el(
      "div",
      { class: "blocchi-coltura" },
      ...gruppi.map((g) => el("div", { class: "blocco" }, el("h4", {}, g.titolo), el("div", { class: "etichette" }, ...g.colture.map(etichettaColtura)))),
    ),
  );
}

function calendarioAnnuale() {
  const m = meseCorrente();
  const { sezioni, schede, mesi } = cat();
  const gruppi = sezioni.filter((s) => schede.some((x) => x.sezione === s.id && x.calendario));
  const righe = [];
  for (const g of gruppi) {
    righe.push(el("tr", {}, el("th", { class: "gruppo", colspan: 13 }, g.titolo)));
    for (const s of schede.filter((x) => x.sezione === g.id && x.calendario)) {
      righe.push(
        el(
          "tr",
          {},
          el("th", { class: "nome", scope: "row", onclick: () => apriScheda(s.id) }, s.nome),
          ...mesi.map((_, i) => el("td", { class: i + 1 === m ? "corrente" : null, title: descriviMese(s, i + 1) }, ...barreMese(s, i + 1))),
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
        el("thead", {}, el("tr", {}, el("th", {}), ...mesi.map((n, i) => el("th", { class: i + 1 === m ? "mese-corrente" : null }, n)))),
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
  const { nomi_mesi } = cat();
  const voci = OPERAZIONI.filter(([k]) => s.calendario[k]?.includes(m)).map(([, t]) => t.toLowerCase());
  return `${s.nome}, ${nomi_mesi[m - 1]}: ${voci.length ? voci.join(", ") : "nessuna operazione principale"}`;
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

// ------------------------------------------------------------------ schede colturali

function kcTesto(kc) {
  return kc.map((v) => num(v, 2)).join(" · ");
}

function tabellaConfronto(schede) {
  const conDati = schede.filter((s) => s.tSuolo && s.kc);
  if (conDati.length < 2) return null;
  const { mesi } = cat();
  const epoca = (s) => {
    const c = s.calendario ?? {};
    const mList = [...new Set([...(c.semina ?? []), ...(c.trapianto ?? [])])];
    return mList.length ? mList.map((m) => mesi[m - 1]).join(", ") : "—";
  };
  return el(
    "section",
    { class: "pannello" },
    el("h2", {}, "Confronto rapido delle soglie agronomiche"),
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

function barraTermicaColtura(s) {
  if (!s.tSuolo) return null;
  const tSuoloAttuale = ui.analisi?.contesto?.t_suolo_6;
  const min = s.tSuolo.min;
  const max = s.tSuolo.max ?? 35;
  const scalaMin = 0;
  const scalaMax = 38;
  const pct = (v) => Math.max(0, Math.min(100, ((v - scalaMin) / (scalaMax - scalaMin)) * 100));

  const fasciaInizio = pct(min);
  const fasciaFine = pct(max);
  const cursore = Number.isFinite(tSuoloAttuale) ? pct(tSuoloAttuale) : null;
  const ok = Number.isFinite(tSuoloAttuale) && tSuoloAttuale >= min && (s.tSuolo.max == null || tSuoloAttuale <= s.tSuolo.max);

  return el(
    "div",
    { class: "termometro-coltura" },
    el(
      "div",
      { class: "termometro-testata" },
      el("span", {}, `Finestra termica suolo (6 cm): min ${min} °C · ottimale ${s.tSuolo.opt} °C${s.tSuolo.max != null ? ` · max ${s.tSuolo.max} °C` : ""}`),
      Number.isFinite(tSuoloAttuale)
        ? el("strong", { class: ok ? "t-ok" : "t-ko" }, `Oggi a 6 cm: ${num(tSuoloAttuale)} °C`)
        : null,
    ),
    el(
      "div",
      { class: "termometro-pista" },
      el("div", { class: "termometro-fascia", style: `left:${fasciaInizio}%;width:${Math.max(4, fasciaFine - fasciaInizio)}%` }),
      cursore != null
        ? el("div", { class: `termometro-cursore ${ok ? "ok" : "ko"}`, style: `left:${cursore}%`, title: `Suolo oggi: ${num(tSuoloAttuale)} °C` })
        : null,
    ),
  );
}

function strisciaMesi(s) {
  const m = meseCorrente();
  const { mesi } = cat();
  return el(
    "div",
    { class: "striscia-mesi", "aria-label": "Calendario colturale" },
    ...mesi.map((n, i) => el("div", { class: i + 1 === m ? "corrente" : null, title: descriviMese(s, i + 1) }, ...barreMese(s, i + 1), n)),
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
      el(
        "div",
        { class: "testata-legale" },
        s.illustrazione ? svg(s.illustrazione, "illustrazione-scheda") : null,
        el(
          "div",
          {},
          el("h3", {}, `⚖️ ${s.nome}`),
          el("p", {}, s.sintesi),
        ),
      ),
      ...s.blocchi.map((b) => el("div", {}, el("strong", {}, b.titolo), el("ul", {}, ...b.punti.map((p) => el("li", {}, p))))),
    );
  }

  const chip = ui.analisi?.stato_colture?.[s.id] ?? [];
  const dati = [...(s.dati ?? [])];
  if (s.kc && !dati.some(([k]) => k === "Kc")) dati.push(["Kc ini · metà · fine", kcTesto(s.kc)]);
  if (s.gelo != null) dati.push(["Danni da gelo", `sotto ${s.gelo} °C`]);

  const d = el(
    "details",
    { class: "pannello coltura", id: `scheda-${s.id}`, open: ui.aperte.has(s.id) },
    el(
      "summary",
      {},
      s.illustrazione ? svg(s.illustrazione, "illustrazione-scheda") : null,
      el(
        "div",
        { class: "sommario-coltura-testo" },
        el(
          "div",
          { class: "titolo-coltura" },
          el("h3", {}, s.nome),
          s.latino ? el("span", { class: "latino" }, s.latino) : null,
          s.famiglia ? el("span", { class: "famiglia" }, s.famiglia) : null,
        ),
        el("p", { class: "sintesi" }, s.sintesi),
        chip.length ? el("div", { class: "stato-live" }, ...chip.map((c) => el("span", { class: "chip" }, pallino(c.livello), c.testo))) : null,
      ),
      el("span", { class: "freccia", "aria-hidden": "true" }, "▶"),
    ),
    el(
      "div",
      { class: "corpo-coltura" },
      s.infografica ? svg(s.infografica, "infografica-scheda") : null,
      barraTermicaColtura(s),
      dati.length ? el("dl", { class: "dati-rapidi" }, ...dati.map(([k, v]) => el("div", {}, el("dt", {}, k), el("dd", {}, v)))) : null,
      s.calendario ? strisciaMesi(s) : null,
      s.calendario ? legendaCalendario() : null,
      el("div", { class: "blocchi-coltura", style: s.calendario ? "margin-top:14px" : null }, s.tabella ? tabellaScheda(s.tabella) : null, ...(s.blocchi ?? []).map(blocco)),
    ),
  );
  d.addEventListener("toggle", () => (d.open ? ui.aperte.add(s.id) : ui.aperte.delete(s.id)));
  return d;
}

// ------------------------------------------------------------------ fonti

function renderFonti() {
  const { fonti, avvertenze } = cat();
  return el(
    "div",
    { class: "griglia-colture" },
    el(
      "section",
      { class: "pannello" },
      el("h2", {}, "Come il motore Rust incrocia dati meteo e soglie colturali"),
      el(
        "ul",
        { class: "fonti" },
        el("li", {}, "Temperatura del suolo a 6 cm → confronto con la temperatura minima, ottimale e massima di germinazione o di trapianto di ogni coltura."),
        el("li", {}, "Umidità degli strati 3–9 cm e 9–27 cm → stato di tempera per semina e aratura (soglia: 40 % dell'acqua disponibile, come nel cruscotto)."),
        el("li", {}, "Temperatura minima delle prossime 72 ore → confronto con la soglia critica di danno da gelo delle colture presumibilmente in campo nel mese."),
        el("li", {}, "Pioggia prevista e passata → finestre per concimazione azotata, semina degli erbai, fienagione e rischio di erosione."),
        el("li", {}, "Temperatura e umidità oraria → THI per gli ovini al pascolo, periodo di Smith per la peronospora, rugiada mattutina per il meteorismo."),
        el("li", {}, "ET0 e pioggia efficace previste → fabbisogno irriguo settimanale di una coltura a pieno sviluppo."),
      ),
    ),
    el("section", { class: "pannello" }, el("h2", {}, "Fonti principali"), el("ol", { class: "fonti" }, ...fonti.map((f) => el("li", {}, f)))),
    el("section", { class: "pannello" }, el("h2", {}, "Avvertenze"), el("ul", { class: "fonti" }, ...avvertenze.map((a) => el("li", {}, a)))),
  );
}
