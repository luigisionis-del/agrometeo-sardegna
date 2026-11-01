// Navigazione tra le sezioni principali (schede), con ancora nell'URL e ultima scelta ricordata.
// L'ancora è «#/sezione»: non coincide con l'id dell'elemento, così il browser non scorre la pagina.
const SEZIONI = ["cruscotto", "previsioni", "almanacco", "stazioni"];
const CHIAVE = "sezione";
const daAncora = () => location.hash.replace(/^#\/?/, "");

function preferita() {
  const daUrl = daAncora();
  if (SEZIONI.includes(daUrl)) return daUrl;
  try {
    const v = localStorage.getItem(CHIAVE);
    return SEZIONI.includes(v) ? v : SEZIONI[0];
  } catch {
    return SEZIONI[0];
  }
}

export function mostraSezione(id) {
  if (!SEZIONI.includes(id)) return;
  const cambiata = document.getElementById(id).hidden;
  for (const s of SEZIONI) {
    document.getElementById(s).hidden = s !== id;
    const tab = document.getElementById(`tab-${s}`);
    tab.setAttribute("aria-selected", String(s === id));
    tab.tabIndex = s === id ? 0 : -1;
  }
  if (daAncora() !== id || !location.hash.startsWith("#/")) history.replaceState(null, "", `#/${id}`);
  if (cambiata) window.scrollTo(0, 0);
  try {
    localStorage.setItem(CHIAVE, id);
  } catch {
    /* preferenza non salvabile: nessun effetto sul funzionamento */
  }
}

export function initNavigazione() {
  const schede = [...document.querySelectorAll(".scheda-nav")];
  for (const tab of schede) tab.addEventListener("click", () => mostraSezione(tab.dataset.sezione));
  // frecce sinistra/destra tra le schede (pattern ARIA tablist)
  document.querySelector(".navigazione").addEventListener("keydown", (e) => {
    if (e.key !== "ArrowRight" && e.key !== "ArrowLeft") return;
    const i = schede.indexOf(document.activeElement);
    if (i < 0) return;
    const prossima = schede[(i + (e.key === "ArrowRight" ? 1 : schede.length - 1)) % schede.length];
    prossima.focus();
    mostraSezione(prossima.dataset.sezione);
  });
  window.addEventListener("hashchange", () => mostraSezione(daAncora()));
  mostraSezione(preferita());
}
