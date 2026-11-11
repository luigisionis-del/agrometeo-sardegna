// Mappa radar live e GIS: RainViewer animato, WMS opzionale, vento, appezzamenti GeoJSON.
import { api } from "./api.js";
import { el, num, puntoCardinale } from "./formato.js";
import { salvaSpazio, stato } from "./stato.js";
import { mostraErrore } from "./testata.js";
import { TV } from "./tv.js";

const $ = (id) => document.getElementById(id);
const CENTRO_SARDEGNA = [40.0, 9.0];
const RADAR_INTERVALLO_MS = 5 * 60_000;
const VENTO_INTERVALLO_MS = 20 * 60_000;

const BASI = {
  chiaro: {
    url: "https://server.arcgisonline.com/ArcGIS/rest/services/Canvas/World_Light_Gray_Base/MapServer/tile/{z}/{y}/{x}",
    etichette: "https://server.arcgisonline.com/ArcGIS/rest/services/Canvas/World_Light_Gray_Reference/MapServer/tile/{z}/{y}/{x}",
    opzioni: { maxZoom: 16, attribution: "Tiles © Esri — Esri, HERE, Garmin, © OpenStreetMap contributors" },
  },
  scuro: {
    url: "https://server.arcgisonline.com/ArcGIS/rest/services/Canvas/World_Dark_Gray_Base/MapServer/tile/{z}/{y}/{x}",
    // etichette su un livello separato, sopra il radar
    etichette: "https://server.arcgisonline.com/ArcGIS/rest/services/Canvas/World_Dark_Gray_Reference/MapServer/tile/{z}/{y}/{x}",
    opzioni: { maxZoom: 16, attribution: "Tiles © Esri — Esri, HERE, Garmin, © OpenStreetMap contributors" },
  },
  osm: {
    url: "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
    opzioni: { maxZoom: 19, attribution: "© OpenStreetMap contributors" },
  },
  topo: {
    url: "https://{s}.tile.opentopomap.org/{z}/{x}/{y}.png",
    opzioni: { subdomains: "abc", maxZoom: 17, attribution: "© OpenStreetMap contributors, SRTM · © OpenTopoMap (CC-BY-SA)" },
  },
  s2: {
    url: "https://tiles.maps.eox.at/wmts/1.0.0/s2cloudless-2023_3857/default/g/{z}/{y}/{x}.jpg",
    opzioni: {
      maxZoom: 17,
      attribution: "Sentinel-2 cloudless 2023 by EOX IT Services GmbH (contiene dati Copernicus Sentinel modificati)",
    },
  },
};

const COLORI_TIPO = {
  vigneto: "#8e44c9",
  pascolo: "#1f9d55",
  seminativo: "#c98500",
  erbaio: "#5a9e1c",
  orto: "#d4581f",
  oliveto: "#5f7f6c",
  altro: "#66736b",
};

let mappa;
let base;
let etichette;
let wms;
let marcatoreLuogo;
let livelloAppezzamenti;
let livelloVento;
let impostaLuogo;

const radar = { fotogrammi: [], livelli: [], indice: 0, timer: null, host: "", zoomMax: 7 };
const disegno = { attivo: false, punti: [], linea: null, vertici: null };

export function initMappa(opzioni) {
  impostaLuogo = opzioni.impostaLuogo;
  // col telecomando le frecce servono a spostare il fuoco: la mappa non le cattura per lo scorrimento
  mappa = L.map("mappa", { zoomControl: true, attributionControl: true, keyboard: !TV }).setView(CENTRO_SARDEGNA, 8);
  mappa.createPane("radar").style.zIndex = 350;
  mappa.createPane("etichette").style.zIndex = 380;
  mappa.getPane("etichette").style.pointerEvents = "none";
  mappa.createPane("vento").style.zIndex = 450;

  livelloAppezzamenti = L.geoJSON(null, { style: stileAppezzamento, onEachFeature: popupAppezzamento }).addTo(mappa);
  livelloVento = L.layerGroup().addTo(mappa);
  marcatoreLuogo = L.circleMarker(CENTRO_SARDEGNA, { radius: 9, color: "#fff", weight: 3, fillColor: "#1f8a4c", fillOpacity: 1 }).addTo(mappa);

  mappa.on("click", clickMappa);
  applicaLayer();
  renderAppezzamenti();
  initRadar();
  initDisegno();
  aggiornaVento();
  setInterval(aggiornaVento, VENTO_INTERVALLO_MS);
  new ResizeObserver(() => mappa.invalidateSize()).observe($("mappa"));
}

/** Base cartografica, WMS e vento secondo le impostazioni salvate. */
export function applicaLayer() {
  const l = stato.spazio.layer;
  const scelta = BASI[l.base] ?? BASI.chiaro;
  base?.remove();
  etichette?.remove();
  base = L.tileLayer(scelta.url, scelta.opzioni).addTo(mappa);
  base.bringToBack();
  etichette = scelta.etichette ? L.tileLayer(scelta.etichette, { pane: "etichette", maxZoom: scelta.opzioni.maxZoom }).addTo(mappa) : null;

  wms?.remove();
  wms = null;
  if (l.wms_attivo && l.wms_url && l.wms_layer) {
    wms = L.tileLayer
      .wms(l.wms_url, { layers: l.wms_layer, format: "image/png", transparent: true, opacity: l.wms_opacita, pane: "radar", attribution: "Radar DPC / ARPAS" })
      .addTo(mappa);
  }
  if (l.vento) aggiornaVento();
  else livelloVento.clearLayers();
  for (const livello of radar.livelli) livello.setOpacity(0);
  radar.livelli[radar.indice]?.setOpacity(l.radar_opacita);
  $("radar-opacita").value = Math.round(l.radar_opacita * 100);
}

export function centraSuLuogo(luogo, sposta = true) {
  marcatoreLuogo.setLatLng([luogo.lat, luogo.lon]).bindTooltip(luogo.nome);
  if (sposta && !mappa.getBounds().pad(-0.1).contains([luogo.lat, luogo.lon])) mappa.panTo([luogo.lat, luogo.lon]);
}

function clickMappa(e) {
  if (disegno.attivo) return aggiungiVertice(e.latlng);
  const { lat, lng } = e.latlng;
  const nome = `Punto ${lat.toFixed(3)}, ${lng.toFixed(3)}`;
  L.popup()
    .setLatLng(e.latlng)
    .setContent(
      el(
        "div",
        {},
        el("div", {}, `${lat.toFixed(4)}, ${lng.toFixed(4)}`),
        el("button", { class: "pulsante", onclick: () => { mappa.closePopup(); impostaLuogo({ nome, lat, lon: lng }); } }, "Meteo in questo punto"),
      ),
    )
    .openOn(mappa);
}

// --- radar RainViewer ---

function initRadar() {
  $("radar-play").addEventListener("click", () => (radar.timer ? fermaRadar() : avviaRadar()));
  $("radar-indietro").addEventListener("click", () => { fermaRadar(); mostraFotogramma(radar.indice - 1); });
  $("radar-avanti").addEventListener("click", () => { fermaRadar(); mostraFotogramma(radar.indice + 1); });
  $("radar-slider").addEventListener("input", (e) => { fermaRadar(); mostraFotogramma(Number(e.target.value)); });
  $("radar-opacita").addEventListener("input", (e) => {
    stato.spazio.layer.radar_opacita = Number(e.target.value) / 100;
    radar.livelli[radar.indice]?.setOpacity(stato.spazio.layer.radar_opacita);
    salvaSpazio();
  });
  caricaRadar();
  setInterval(caricaRadar, RADAR_INTERVALLO_MS);
}

async function caricaRadar() {
  let dati;
  try {
    dati = await api.radar();
  } catch (e) {
    $("radar-ora").textContent = "Radar non disponibile";
    $("radar-ora").title = String(e);
    return;
  }
  const inRiproduzione = Boolean(radar.timer);
  fermaRadar();
  for (const l of radar.livelli) l.remove();
  radar.host = dati.host;
  radar.zoomMax = dati.zoom_max;
  radar.fotogrammi = dati.fotogrammi;
  radar.livelli = dati.fotogrammi.map((f) =>
    L.tileLayer(`${dati.host}${f.path}/256/{z}/{x}/{y}/2/1_1.png`, {
      pane: "radar",
      opacity: 0,
      maxNativeZoom: dati.zoom_max,
      maxZoom: 19,
      attribution: '<a href="https://www.rainviewer.com/api.html">RainViewer</a>',
    }).addTo(mappa),
  );
  $("radar-slider").max = Math.max(0, radar.fotogrammi.length - 1);
  // l'ultimo fotogramma osservato (non nowcast) è il più recente reale
  const ultimoOsservato = radar.fotogrammi.findLastIndex((f) => !f.nowcast);
  mostraFotogramma(ultimoOsservato >= 0 ? ultimoOsservato : 0);
  if (inRiproduzione) avviaRadar();
}

function mostraFotogramma(i) {
  const n = radar.fotogrammi.length;
  if (!n) return;
  i = (i + n) % n;
  radar.livelli[radar.indice]?.setOpacity(0);
  radar.indice = i;
  radar.livelli[i].setOpacity(stato.spazio.layer.radar_opacita);
  const f = radar.fotogrammi[i];
  const t = new Date(f.time * 1000).toLocaleTimeString("it-IT", { hour: "2-digit", minute: "2-digit" });
  $("radar-ora").textContent = `${f.nowcast ? "Previsto" : "Radar"} ${t}`;
  $("radar-slider").value = i;
}

function avviaRadar() {
  if (radar.timer || !radar.fotogrammi.length) return;
  $("radar-play").textContent = "⏸";
  $("radar-play").setAttribute("aria-label", "Pausa");
  radar.timer = setInterval(() => mostraFotogramma(radar.indice + 1), 600);
}

function fermaRadar() {
  clearInterval(radar.timer);
  radar.timer = null;
  $("radar-play").textContent = "▶";
  $("radar-play").setAttribute("aria-label", "Riproduci");
}

// --- vento ---

async function aggiornaVento() {
  if (!stato.spazio?.layer.vento) return;
  let punti;
  try {
    punti = await api.vento();
  } catch (e) {
    console.warn("Vento non disponibile:", e);
    return;
  }
  livelloVento.clearLayers();
  for (const p of punti) {
    // la direzione è quella di provenienza: la freccia punta dove va il vento
    const ruota = (p.direzione + 180) % 360;
    const colore = p.velocita > 30 ? "#d23c3c" : p.velocita > 15 ? "#d97706" : "#24323b";
    const html = `<div class="freccia-vento"><svg width="26" height="26" viewBox="-13 -13 26 26" style="transform:rotate(${ruota}deg)"><path d="M0 -11 L6 3 L1 1 L1 11 L-1 11 L-1 1 L-6 3 Z" fill="${colore}"/></svg><span>${Math.round(p.velocita)}</span></div>`;
    L.marker([p.lat, p.lon], {
      pane: "vento",
      interactive: true,
      icon: L.divIcon({ html, className: "", iconSize: [40, 44], iconAnchor: [20, 16] }),
    })
      .bindTooltip(`Vento ${num(p.velocita, 0)} km/h da ${puntoCardinale(p.direzione)} · raffiche ${num(p.raffiche, 0)} km/h`)
      .addTo(livelloVento);
  }
}

// --- appezzamenti ---

function stileAppezzamento(f) {
  const c = COLORI_TIPO[f.properties?.tipo] ?? COLORI_TIPO.altro;
  return { color: c, weight: 2.5, fillColor: c, fillOpacity: 0.2 };
}

function anelloEsterno(geometria) {
  if (geometria.type === "Polygon") return geometria.coordinates[0];
  if (geometria.type === "MultiPolygon") return geometria.coordinates[0][0];
  return [];
}

/** Area (ettari) con proiezione equirettangolare locale: adeguata per appezzamenti agricoli. */
function ettari(geometria) {
  const R = 6_371_008.8;
  const poligoni = geometria.type === "Polygon" ? [geometria.coordinates] : geometria.coordinates;
  let area = 0;
  for (const poligono of poligoni) {
    poligono.forEach((anello, k) => {
      const lat0 = (anello.reduce((s, p) => s + p[1], 0) / anello.length) * (Math.PI / 180);
      let a = 0;
      for (let i = 0; i < anello.length - 1; i++) {
        const [x1, y1] = anello[i];
        const [x2, y2] = anello[i + 1];
        a += x1 * Math.cos(lat0) * y2 - x2 * Math.cos(lat0) * y1;
      }
      const m2 = Math.abs(a / 2) * (Math.PI / 180) ** 2 * R * R;
      area += k === 0 ? m2 : -m2;
    });
  }
  return area / 10_000;
}

function centroide(geometria) {
  const anello = anelloEsterno(geometria);
  const n = Math.max(1, anello.length - 1);
  const somma = anello.slice(0, n).reduce((s, p) => [s[0] + p[0], s[1] + p[1]], [0, 0]);
  return { lat: somma[1] / n, lon: somma[0] / n };
}

function popupAppezzamento(f, livello) {
  livello.bindPopup(() => {
    const { nome = "Appezzamento", tipo = "altro" } = f.properties ?? {};
    const c = centroide(f.geometry);
    return el(
      "div",
      {},
      el("strong", {}, nome),
      el("div", {}, `${tipo} · ${num(ettari(f.geometry), 2)} ha`),
      el("button", { class: "pulsante", onclick: () => { mappa.closePopup(); impostaLuogo({ nome, lat: c.lat, lon: c.lon }); } }, "Meteo su questo appezzamento"),
      " ",
      el("button", { class: "pulsante secondario", onclick: () => eliminaAppezzamento(f) }, "Elimina"),
    );
  });
}

function renderAppezzamenti() {
  livelloAppezzamenti.clearLayers();
  livelloAppezzamenti.addData(stato.spazio.appezzamenti);
}

function eliminaAppezzamento(f) {
  const fc = stato.spazio.appezzamenti;
  fc.features = fc.features.filter((x) => x !== f);
  mappa.closePopup();
  salvaSpazio();
  renderAppezzamenti();
}

function aggiungiAppezzamenti(features) {
  stato.spazio.appezzamenti.features.push(...features);
  salvaSpazio();
  renderAppezzamenti();
}

function initDisegno() {
  $("disegna").addEventListener("click", () => avviaDisegno());
  $("disegno-esci").addEventListener("click", terminaDisegno);
  $("disegno-annulla-punto").addEventListener("click", () => { disegno.punti.pop(); aggiornaDisegno(); });
  $("disegno-fine").addEventListener("click", () => {
    if (disegno.punti.length < 3) {
      $("disegno-info").textContent = "Servono almeno 3 vertici.";
      return;
    }
    const anello = disegno.punti.map((p) => [Number(p.lng.toFixed(6)), Number(p.lat.toFixed(6))]);
    anello.push(anello[0]);
    aggiungiAppezzamenti([{
      type: "Feature",
      properties: { nome: $("disegno-nome").value.trim() || "Appezzamento", tipo: $("disegno-tipo").value },
      geometry: { type: "Polygon", coordinates: [anello] },
    }]);
    terminaDisegno();
  });

  $("importa-geojson").addEventListener("change", async (e) => {
    const file = e.target.files[0];
    e.target.value = "";
    if (!file) return;
    try {
      const features = estraiPoligoni(JSON.parse(await file.text()), file.name.replace(/\.[^.]+$/, ""));
      if (!features.length) throw new Error("nessun poligono trovato");
      aggiungiAppezzamenti(features);
      mappa.fitBounds(L.geoJSON({ type: "FeatureCollection", features }).getBounds(), { maxZoom: 16 });
    } catch (err) {
      mostraErrore(`GeoJSON non valido: ${err.message}`);
    }
  });

  $("copia-geojson").addEventListener("click", async () => {
    const testo = JSON.stringify(stato.spazio.appezzamenti, null, 2);
    try {
      await navigator.clipboard.writeText(testo);
      $("copia-geojson").textContent = "✓ Copiato";
    } catch {
      $("copia-geojson").textContent = "✕ Non copiato";
    }
    setTimeout(() => ($("copia-geojson").textContent = "⧉ Copia GeoJSON"), 2000);
  });
}

function estraiPoligoni(gj, nomeFile) {
  const features = gj.type === "FeatureCollection" ? gj.features : gj.type === "Feature" ? [gj] : [{ type: "Feature", properties: {}, geometry: gj }];
  return features
    .filter((f) => ["Polygon", "MultiPolygon"].includes(f?.geometry?.type))
    .map((f, i) => {
      const p = f.properties ?? {};
      const nome = p.nome ?? p.name ?? p.NOME ?? p.Name ?? `${nomeFile} ${i + 1}`;
      const tipo = Object.hasOwn(COLORI_TIPO, p.tipo) ? p.tipo : "altro";
      return { type: "Feature", properties: { ...p, nome: String(nome), tipo }, geometry: f.geometry };
    });
}

function avviaDisegno() {
  disegno.attivo = true;
  disegno.punti = [];
  disegno.linea = L.polygon([], { color: "#1f8a4c", weight: 2, dashArray: "6 4", fillOpacity: 0.1 }).addTo(mappa);
  disegno.vertici = L.layerGroup().addTo(mappa);
  $("disegno").hidden = false;
  $("disegno-nome").value = "";
  $("mappa").style.cursor = "crosshair";
  aggiornaDisegno();
}

function aggiungiVertice(latlng) {
  disegno.punti.push(latlng);
  aggiornaDisegno();
}

function aggiornaDisegno() {
  disegno.linea.setLatLngs(disegno.punti);
  disegno.vertici.clearLayers();
  for (const p of disegno.punti) L.circleMarker(p, { radius: 5, color: "#fff", fillColor: "#1f8a4c", fillOpacity: 1, weight: 2 }).addTo(disegno.vertici);
  $("disegno-info").textContent = disegno.punti.length
    ? `${disegno.punti.length} vertici — continua a toccare la mappa, poi Salva`
    : "Tocca la mappa per aggiungere i vertici del perimetro";
}

function terminaDisegno() {
  disegno.attivo = false;
  disegno.linea?.remove();
  disegno.vertici?.remove();
  $("disegno").hidden = true;
  $("mappa").style.cursor = "";
}
