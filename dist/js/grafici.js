// Grafici Chart.js: profilo termico del suolo, bilancio idrico, temperature, radiazione, GDD.
import { dataLocale, giorno, giornoBreve, num } from "./formato.js";

const css = getComputedStyle(document.documentElement);
const colore = (nome) => css.getPropertyValue(nome).trim();
const SERIE = ["--serie-1", "--serie-2", "--serie-3", "--serie-4"].map(colore);

Chart.defaults.color = colore("--testo-3");
Chart.defaults.borderColor = "rgba(23,34,27,0.10)";
Chart.defaults.font.family = getComputedStyle(document.body).fontFamily;
Chart.defaults.font.size = 12;
Chart.defaults.animation = false;
Chart.defaults.maintainAspectRatio = false;
Chart.defaults.plugins.legend.labels.boxWidth = 12;
Chart.defaults.plugins.legend.labels.boxHeight = 12;
Chart.defaults.plugins.legend.labels.useBorderRadius = true;
Chart.defaults.plugins.legend.labels.borderRadius = 3;
Chart.defaults.plugins.legend.labels.color = colore("--testo-2");
Chart.defaults.plugins.tooltip.backgroundColor = "rgba(255,255,255,0.97)";
Chart.defaults.plugins.tooltip.titleColor = colore("--testo");
Chart.defaults.plugins.tooltip.bodyColor = colore("--testo-2");
Chart.defaults.plugins.tooltip.footerColor = colore("--testo-3");
Chart.defaults.plugins.tooltip.padding = 10;
Chart.defaults.plugins.tooltip.borderColor = colore("--bordo-forte");
Chart.defaults.plugins.tooltip.borderWidth = 1;
Chart.defaults.elements.line.borderWidth = 2;
Chart.defaults.elements.point.radius = 0;
Chart.defaults.elements.point.hoverRadius = 5;

const grafici = new Map();

function disegna(id, config) {
  grafici.get(id)?.destroy();
  grafici.set(id, new Chart(document.getElementById(id), config));
}

/** Asse x temporale lineare con tacche a mezzanotte (o al primo del mese). */
function asseTempo({ mensile = false } = {}) {
  return {
    type: "linear",
    afterBuildTicks(asse) {
      const tacche = [];
      const d = new Date(asse.min);
      d.setHours(0, 0, 0, 0);
      if (mensile) d.setDate(1);
      while (d.getTime() <= asse.max) {
        if (d.getTime() >= asse.min) tacche.push({ value: d.getTime() });
        mensile ? d.setMonth(d.getMonth() + 1) : d.setDate(d.getDate() + 1);
      }
      // su schermi stretti una tacca ogni due giorni
      asse.ticks = !mensile && asse.chart.width < 480 ? tacche.filter((_, i) => i % 2 === 0) : tacche;
    },
    ticks: {
      autoSkip: false,
      callback: (v) => (mensile
        ? new Date(v).toLocaleDateString("it-IT", { month: "short" })
        : new Date(v).toLocaleDateString("it-IT", { weekday: "short", day: "2-digit" })),
      maxRotation: 0,
    },
    grid: { color: "rgba(23,34,27,0.07)" },
  };
}

const interazioneIndice = { mode: "index", intersect: false };

/** Linea verticale sull'istante corrente. */
const pluginAdesso = {
  id: "adesso",
  afterDatasetsDraw(chart, _args, opts) {
    if (opts.x == null) return;
    const x = chart.scales.x.getPixelForValue(opts.x);
    const { top, bottom } = chart.chartArea;
    const ctx = chart.ctx;
    ctx.save();
    ctx.strokeStyle = "rgba(23,34,27,0.45)";
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(x, top);
    ctx.lineTo(x, bottom);
    ctx.stroke();
    ctx.restore();
  },
};
Chart.register(pluginAdesso);

const tempoOra = (o) => dataLocale(o.time).getTime();
const adesso = (r) => ({ x: dataLocale(r.ora_corrente).getTime() });

function titoloOraLocale(items) {
  if (!items.length) return "";
  const d = new Date(items[0].parsed.x);
  return d.toLocaleString("it-IT", { weekday: "short", day: "2-digit", month: "short", hour: "2-digit", minute: "2-digit" });
}

function prossimeOre(r, ore) {
  return r.ore.slice(r.indice_ora, r.indice_ora + ore);
}

export function graficoSuolo(r) {
  const ore = prossimeOre(r, 168);
  const etichette = ["0 cm", "6 cm", "18 cm", "54 cm"];
  disegna("g-suolo", {
    type: "line",
    data: {
      datasets: etichette.map((label, i) => ({
        label,
        data: ore.map((o) => ({ x: tempoOra(o), y: o.suolo_t[i] })),
        borderColor: SERIE[i],
        backgroundColor: SERIE[i],
      })),
    },
    options: {
      interaction: interazioneIndice,
      scales: { x: asseTempo(), y: { title: { display: true, text: "°C" } } },
      plugins: {
        tooltip: { callbacks: { title: titoloOraLocale, label: (c) => `${c.dataset.label}: ${num(c.parsed.y)} °C` } },
        adesso: adesso(r),
      },
    },
  });
}

export function graficoBilancio(r) {
  const g = r.giorni;
  const etichette = g.map((d) => (d.previsto ? giornoBreve(d.data) : d.data === r.ora_corrente.slice(0, 10) ? "oggi" : giornoBreve(d.data)));
  const attenua = (c) => g.map((d) => (d.previsto || etichette[g.indexOf(d)] === "oggi" ? c : `${c}88`));
  document.getElementById("kc-etichetta").textContent = num(r.parametri.kc, 2);
  disegna("g-bilancio", {
    data: {
      labels: etichette,
      datasets: [
        { type: "bar", label: "Pioggia", data: g.map((d) => d.pioggia), backgroundColor: attenua(SERIE[0]), borderRadius: 4, order: 2 },
        { type: "bar", label: "ETc", data: g.map((d) => d.etc), backgroundColor: attenua(SERIE[1]), borderRadius: 4, order: 2 },
        {
          type: "line",
          label: "Bilancio cumulato",
          data: g.map((d) => d.bilancio_cumulato),
          borderColor: SERIE[2],
          backgroundColor: SERIE[2],
          pointRadius: 3,
          order: 1,
        },
      ],
    },
    options: {
      interaction: interazioneIndice,
      scales: { x: { grid: { display: false } }, y: { title: { display: true, text: "mm" } } },
      plugins: {
        tooltip: {
          callbacks: {
            title: (items) => giorno(g[items[0].dataIndex].data) + (g[items[0].dataIndex].previsto ? " (previsione)" : ""),
            label: (c) => `${c.dataset.label}: ${num(c.parsed.y)} mm`,
            afterBody: (items) => {
              const d = g[items[0].dataIndex];
              return [`Pioggia efficace: ${num(d.pioggia_efficace)} mm`, `ΔW giorno: ${num(d.delta_w)} mm`];
            },
          },
        },
      },
    },
  });
}

export function graficoTemperatura(r) {
  const ore = prossimeOre(r, 168);
  disegna("g-temperatura", {
    type: "line",
    data: {
      datasets: [
        { label: "Temperatura", data: ore.map((o) => ({ x: tempoOra(o), y: o.t })), borderColor: SERIE[1], backgroundColor: SERIE[1] },
        { label: "Punto di rugiada", data: ore.map((o) => ({ x: tempoOra(o), y: o.td })), borderColor: SERIE[0], backgroundColor: SERIE[0] },
      ],
    },
    options: {
      interaction: interazioneIndice,
      scales: { x: asseTempo(), y: { title: { display: true, text: "°C" } } },
      plugins: {
        tooltip: { callbacks: { title: titoloOraLocale, label: (c) => `${c.dataset.label}: ${num(c.parsed.y)} °C` } },
        adesso: adesso(r),
      },
    },
  });
}

export function graficoRadiazione(r) {
  const ore = prossimeOre(r, 168);
  disegna("g-radiazione", {
    type: "line",
    data: {
      datasets: [
        {
          label: "Globale (GHI)",
          data: ore.map((o) => ({ x: tempoOra(o), y: o.radiazione })),
          borderColor: SERIE[3],
          backgroundColor: `${SERIE[3]}40`,
          fill: "origin",
        },
        { label: "Diretta normale (DNI)", data: ore.map((o) => ({ x: tempoOra(o), y: o.dni })), borderColor: SERIE[0], backgroundColor: SERIE[0] },
      ],
    },
    options: {
      interaction: interazioneIndice,
      scales: { x: asseTempo(), y: { min: 0, title: { display: true, text: "W/m²" } } },
      plugins: {
        tooltip: {
          callbacks: {
            title: titoloOraLocale,
            label: (c) => `${c.dataset.label}: ${num(c.parsed.y, 0)} W/m²`,
            afterBody: (items) => `PAR stimata: ${num(ore[items[0].dataIndex].par, 0)} µmol/m²/s`,
          },
        },
        adesso: adesso(r),
      },
    },
  });
}

export function graficoGdd(r) {
  const box = document.getElementById("gdd-totali");
  box.replaceChildren(
    ...r.gdd.map((s) => {
      const d = document.createElement("div");
      d.innerHTML = `${s.coltura} (base ${s.t_base} °C, dal ${new Date(s.inizio).toLocaleDateString("it-IT")})<br><b>${num(s.totale_oggi, 0)}</b> GDD · fra 7 giorni ${num(s.totale_previsto, 0)}${s.parziale ? " · <span class=\"errore\">parziale</span>" : ""}`;
      return d;
    }),
  );
  disegna("g-gdd", {
    type: "line",
    data: {
      datasets: r.gdd.map((s, i) => ({
        label: s.coltura,
        data: s.serie.map((g) => ({ x: dataLocale(g.data).getTime(), y: g.cumulato, previsto: g.previsto })),
        borderColor: SERIE[i],
        backgroundColor: SERIE[i],
        segment: { borderDash: (ctx) => (ctx.p1.raw?.previsto ? [6, 4] : undefined) },
      })),
    },
    options: {
      interaction: { mode: "nearest", intersect: false, axis: "x" },
      scales: { x: asseTempo({ mensile: true }), y: { min: 0, title: { display: true, text: "GDD" } } },
      plugins: {
        tooltip: {
          callbacks: {
            title: (items) => (items.length ? new Date(items[0].parsed.x).toLocaleDateString("it-IT") : ""),
            label: (c) => `${c.dataset.label}: ${num(c.parsed.y, 0)} GDD${c.raw.previsto ? " (previsto)" : ""}`,
          },
        },
      },
    },
  });
}

export function tabellaGiorni(r) {
  const intestazioni = ["Giorno", "T max", "T min", "Pioggia", "P eff.", "ET0", "ETc", "ΔW", "Bilancio", "Sole h", "UV", "FV kWh/kWp", "GDD vite", "GDD forag.", "Semina"];
  const oggi = r.ora_corrente.slice(0, 10);
  const t = document.getElementById("tabella-giorni");
  t.className = "dati";
  t.innerHTML = "";
  const thead = t.createTHead().insertRow();
  for (const h of intestazioni) thead.append(Object.assign(document.createElement("th"), { textContent: h }));
  const tbody = t.createTBody();
  for (const d of r.giorni) {
    const riga = tbody.insertRow();
    riga.className = d.data === oggi ? "oggi" : d.previsto ? "previsto" : "";
    const celle = [
      giorno(d.data) + (d.previsto ? " ⓟ" : ""),
      num(d.t_max), num(d.t_min), num(d.pioggia), num(d.pioggia_efficace), num(d.et0), num(d.etc),
      num(d.delta_w), num(d.bilancio_cumulato), num(d.ore_sole), num(d.uv_max), num(d.fv_kwh_kwp),
      num(d.gdd_vite), num(d.gdd_foraggere), d.semina.valutazione.etichetta,
    ];
    for (const c of celle) riga.insertCell().textContent = c;
  }
}
