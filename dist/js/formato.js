// Formattazione in italiano e utilità comuni di rendering.
const fmtOra = new Intl.DateTimeFormat("it-IT", { hour: "2-digit", minute: "2-digit" });
const fmtGiorno = new Intl.DateTimeFormat("it-IT", { weekday: "short", day: "2-digit", month: "short" });
const fmtGiornoBreve = new Intl.DateTimeFormat("it-IT", { weekday: "short", day: "2-digit" });

/** Le ore del backend sono locali (Europe/Rome) senza fuso: "2026-11-01T14:00:00". */
export const dataLocale = (s) => new Date(s.length <= 10 ? `${s}T00:00:00` : s);

export const ora = (s) => fmtOra.format(dataLocale(s));
export const giorno = (s) => fmtGiorno.format(dataLocale(s));
export const giornoBreve = (s) => fmtGiornoBreve.format(dataLocale(s));
export const quando = (s) => `${giorno(s)} ${ora(s)}`;

export function num(v, decimali = 1) {
  return Number.isFinite(v) ? v.toLocaleString("it-IT", { minimumFractionDigits: decimali, maximumFractionDigits: decimali }) : "—";
}

export function eta(isoUtc) {
  const minuti = Math.max(0, Math.round((Date.now() - new Date(isoUtc).getTime()) / 60000));
  if (minuti < 1) return "adesso";
  if (minuti < 60) return `${minuti} min fa`;
  const ore = Math.floor(minuti / 60);
  if (ore < 48) return `${ore} h fa`;
  return `${Math.floor(ore / 24)} giorni fa`;
}

export const SIMBOLO = { verde: "", giallo: "•", arancio: "▲", rosso: "✕", inattivo: "–" };

export function el(tag, attributi = {}, ...figli) {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(attributi)) {
    if (v == null || v === false) continue;
    if (k === "class") e.className = v;
    else if (k.startsWith("on")) e.addEventListener(k.slice(2), v);
    else e.setAttribute(k, v === true ? "" : v);
  }
  for (const f of figli.flat()) if (f != null) e.append(f instanceof Node ? f : String(f));
  return e;
}

export function pallino(livello) {
  return el("i", { class: `pallino s-${livello}`, "aria-hidden": "true" }, SIMBOLO[livello]);
}

export function puntoCardinale(gradi) {
  const nomi = ["N", "NE", "E", "SE", "S", "SO", "O", "NO"];
  return Number.isFinite(gradi) ? nomi[Math.round(gradi / 45) % 8] : "";
}
