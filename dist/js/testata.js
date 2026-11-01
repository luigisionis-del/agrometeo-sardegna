// Header operativo: selettore località, ricerca, KPI immediati, età del dato.
import { api } from "./api.js";
import { el, eta, num, pallino, puntoCardinale } from "./formato.js";
import { salvaSpazio, stato } from "./stato.js";

const $ = (id) => document.getElementById(id);
const COORDINATE = /^\s*(-?\d{1,2}(?:[.,]\d+)?)\s*[,;\s]\s*(-?\d{1,3}(?:[.,]\d+)?)\s*$/;

let impostaLuogo;

export function initTestata(opzioni) {
  impostaLuogo = opzioni.impostaLuogo;
  $("luoghi").addEventListener("change", (e) => {
    const l = stato.spazio.luoghi[Number(e.target.value)];
    if (l) impostaLuogo(l);
  });
  $("salva-luogo").addEventListener("click", salvaPreset);
  $("rimuovi-luogo").addEventListener("click", rimuoviPreset);
  $("aggiorna").addEventListener("click", () => opzioni.aggiorna(true));
  initRicerca();
  setInterval(aggiornaOrologio, 15_000);
  aggiornaOrologio();
}

const stessoLuogo = (a, b) => Math.abs(a.lat - b.lat) < 1e-4 && Math.abs(a.lon - b.lon) < 1e-4;

export function renderLuoghi() {
  const { luoghi, luogo_attivo } = stato.spazio;
  const sel = $("luoghi");
  sel.replaceChildren();
  let selezionato = -1;
  luoghi.forEach((l, i) => {
    sel.append(el("option", { value: i }, l.nome));
    if (stessoLuogo(l, luogo_attivo)) selezionato = i;
  });
  if (selezionato < 0) {
    sel.append(el("option", { value: -1 }, `${luogo_attivo.nome} (non salvata)`));
    sel.value = "-1";
  } else {
    sel.value = String(selezionato);
  }
  $("rimuovi-luogo").disabled = selezionato < 0 || luoghi.length <= 1;
  $("salva-luogo").disabled = selezionato >= 0;
}

function salvaPreset() {
  const { luoghi, luogo_attivo } = stato.spazio;
  if (luoghi.some((l) => stessoLuogo(l, luogo_attivo))) return;
  luoghi.push({ ...luogo_attivo });
  salvaSpazio();
  renderLuoghi();
}

function rimuoviPreset() {
  const i = Number($("luoghi").value);
  if (i < 0 || stato.spazio.luoghi.length <= 1) return;
  stato.spazio.luoghi.splice(i, 1);
  salvaSpazio();
  renderLuoghi();
}

function initRicerca() {
  const input = $("ricerca");
  const lista = $("risultati");
  let timer;
  let richiesta = 0;

  const chiudi = () => { lista.hidden = true; lista.replaceChildren(); };
  const scegli = (luogo) => { chiudi(); input.value = ""; impostaLuogo(luogo); };

  const mostra = (voci) => {
    lista.replaceChildren(
      ...(voci.length
        ? voci.map((v) => el("li", { role: "option", onclick: () => scegli(v.luogo) }, v.titolo, " ", el("small", {}, v.sotto)))
        : [el("li", { "aria-disabled": "true" }, "Nessun risultato in Sardegna")]),
    );
    lista.hidden = false;
  };

  input.addEventListener("input", () => {
    clearTimeout(timer);
    const testo = input.value.trim();
    if (testo.length < 2) return chiudi();
    const c = testo.match(COORDINATE);
    if (c) {
      const lat = Number(c[1].replace(",", "."));
      const lon = Number(c[2].replace(",", "."));
      return mostra([{ titolo: `Coordinate ${lat.toFixed(4)}, ${lon.toFixed(4)}`, sotto: "", luogo: { nome: `Punto ${lat.toFixed(3)}, ${lon.toFixed(3)}`, lat, lon } }]);
    }
    timer = setTimeout(async () => {
      const id = ++richiesta;
      try {
        const r = await api.cercaLocalita(testo);
        if (id !== richiesta) return;
        mostra(r.map((l) => ({
          titolo: l.nome,
          sotto: [l.provincia, Number.isFinite(l.quota) ? `${Math.round(l.quota)} m` : null].filter(Boolean).join(" · "),
          luogo: { nome: l.nome, lat: l.lat, lon: l.lon },
        })));
      } catch (e) {
        if (id === richiesta) {
          lista.replaceChildren(el("li", { class: "errore" }, `Ricerca non disponibile: ${e}`));
          lista.hidden = false;
        }
      }
    }, 350);
  });
  input.addEventListener("keydown", (e) => {
    if (e.key === "Escape") chiudi();
    if (e.key === "Enter") lista.querySelector("li[role=option]")?.click();
  });
  document.addEventListener("click", (e) => { if (!e.target.closest(".ricerca")) chiudi(); });
}

function aggiornaOrologio() {
  $("orologio").textContent = new Date().toLocaleTimeString("it-IT", { hour: "2-digit", minute: "2-digit" });
  aggiornaBadge();
}

export function aggiornaBadge() {
  const r = stato.report;
  const b = $("badge-dati");
  if (!r) return;
  const minuti = (Date.now() - new Date(r.scaricato_il).getTime()) / 60000;
  const offline = r.fonte === "cache";
  b.className = `badge ${offline ? "offline" : minuti > 90 ? "vecchio" : "fresco"}`;
  b.textContent = `${offline ? "Offline · " : ""}dati di ${eta(r.scaricato_il)}`;
  b.title = `Scaricati il ${new Date(r.scaricato_il).toLocaleString("it-IT")} · modello a ${Math.round(r.quota_modello)} m s.l.m. (${r.lat_modello.toFixed(3)}, ${r.lon_modello.toFixed(3)})`;
}

function voce(etichetta, valore, unita, classe) {
  return el(
    "div",
    { class: "kpi-voce", title: classe?.motivo },
    el("div", { class: "etichetta" }, etichetta),
    el("div", { class: "valore" }, valore, el("span", { class: "unita" }, unita)),
    classe ? el("div", { class: "classe" }, pallino(classe.livello), classe.etichetta) : null,
  );
}

export function renderKpi(r) {
  const k = r.kpi;
  $("kpi").replaceChildren(
    voce("Temperatura", num(k.t), "°C", { livello: "inattivo", etichetta: `percepita ${num(k.t_percepita)} °C`, motivo: "" }),
    voce("Umidità rel.", num(k.rh, 0), "%", { livello: "inattivo", etichetta: `rugiada ${num(k.td)} °C`, motivo: "" }),
    voce("VPD", num(k.vpd, 2), "kPa", k.vpd_classe),
    voce("THI", num(k.thi, 1), "", k.thi_classe),
    voce("Pioggia 24 h", num(k.pioggia_24h), "mm"),
    voce("Vento", num(k.vento, 0), "km/h", { livello: "inattivo", etichetta: `${puntoCardinale(k.vento_dir)} · raffiche ${num(k.raffiche, 0)}`, motivo: "" }),
    voce("ET0 oggi", num(k.et0_oggi), "mm"),
  );
  const avvisi = $("avvisi");
  avvisi.hidden = r.avvisi.length === 0;
  avvisi.replaceChildren(...r.avvisi.map((a) => el("div", {}, "⚠ ", a)));
  aggiornaBadge();
}

export function mostraErrore(messaggio) {
  const avvisi = $("avvisi");
  avvisi.hidden = false;
  avvisi.replaceChildren(el("div", { class: "errore" }, "✕ ", messaggio));
}
