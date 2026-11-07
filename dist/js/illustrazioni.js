// Rendering delle illustrazioni e infografiche SVG fornite dal catalogo Rust.
import { el } from "./formato.js";
import { stato } from "./stato.js";

const parser = new DOMParser();

/**
 * Crea un contenitore DOM con il nodo <svg> corrispondente all'ID nel catalogo Rust.
 * @param {string} id - Identificativo dell'illustrazione (es. "kpi-temp", "scheda-avena", "info-tempera")
 * @param {string} [classe="illustrazione-svg"] - Classe CSS del contenitore
 * @returns {HTMLElement}
 */
export function svg(id, classe = "illustrazione-svg") {
  const box = el("span", { class: classe, "aria-hidden": "true" });
  const markup = stato.catalogo?.illustrazioni?.[id];
  if (!markup) return box;
  const doc = parser.parseFromString(markup, "image/svg+xml");
  const nodo = doc.documentElement;
  if (nodo && nodo.nodeName.toLowerCase() === "svg") {
    box.appendChild(document.importNode(nodo, true));
  }
  return box;
}

/**
 * Popola tutti i segnaposto [data-svg-id] presenti nel documento con le infografiche del catalogo Rust.
 */
export function popolaSegnapostoSvg() {
  for (const nodo of document.querySelectorAll("[data-svg-id]")) {
    const id = nodo.dataset.svgId;
    const markup = stato.catalogo?.illustrazioni?.[id];
    if (!markup) continue;
    const doc = parser.parseFromString(markup, "image/svg+xml");
    const svgEl = doc.documentElement;
    if (svgEl && svgEl.nodeName.toLowerCase() === "svg") {
      nodo.replaceChildren(document.importNode(svgEl, true));
    }
  }
}
