// Modalità telecomando: su un box TV (Android senza schermo tattile) l'app si usa con le frecce,
// OK e Indietro. Lo spostamento del fuoco tra gli elementi lo fa già il WebView (navigazione
// spaziale); qui si tolgono le trappole e si rende il fuoco ben visibile da lontano.
// Per provarla sul PC: localStorage.setItem("tv", "1") e ricaricare ("0" la forza spenta).

const CAMPI_TESTO = "input[type=search], input[type=text], input[type=url], input[type=number], input:not([type])";

export const TV = rileva();

function rileva() {
  try {
    const forzata = localStorage.getItem("tv");
    if (forzata === "1" || forzata === "0") return forzata === "1";
  } catch {
    /* preferenze non leggibili: si usa il rilevamento automatico */
  }
  return /Android/.test(navigator.userAgent) && navigator.maxTouchPoints === 0;
}

const schedaAttiva = () => document.querySelector('.scheda-nav[aria-selected="true"]');

export function initTv() {
  if (!TV) return;
  document.documentElement.classList.add("tv");

  // I campi di testo non aprono la tastiera a schermo al solo passaggio del fuoco (coprirebbe mezzo
  // schermo e catturerebbe le frecce): si apre premendo OK sul campo. Su un campo di testo il tasto OK
  // non arriva alla pagina, perciò lo segnala l'attività Android chiamando __tvOk.
  const tastiera = navigator.virtualKeyboard;
  if (tastiera) {
    document.addEventListener("focusin", (e) => {
      if (e.target.matches?.(CAMPI_TESTO)) e.target.setAttribute("virtualkeyboardpolicy", "manual");
    });
    window.__tvOk = () => {
      if (document.activeElement?.matches(CAMPI_TESTO)) tastiera.show();
    };
  }
  document.addEventListener(
    "keydown",
    (e) => {
      if (e.key !== "Enter") return;
      const t = e.target;
      // i collegamenti delle attribuzioni cartografiche porterebbero fuori dall'app
      if (t.closest?.(".leaflet-control-attribution")) return e.preventDefault();
      // elementi resi raggiungibili solo in questa modalità (celle, risultati di ricerca)
      if (t.matches?.("td[tabindex], li[tabindex]")) {
        e.preventDefault();
        t.click();
      }
    },
    true,
  );

  // Sui cursori (radar, opacità) sinistra/destra cambiano il valore; su/giù devono invece uscire dal
  // cursore, mentre il browser li userebbe anch'essi per il valore.
  document.addEventListener(
    "keydown",
    (e) => {
      if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return;
      if (!e.target.matches?.("input[type=range]")) return;
      e.preventDefault();
      e.stopImmediatePropagation();
      vicinoInVerticale(e.target, e.key === "ArrowDown")?.focus();
    },
    true,
  );

  // Quando sotto (o sopra) non trova nulla da selezionare il WebView scorre la pagina a piccoli passi.
  // Se però un elemento selezionabile è in vista (il WebView non entra da solo nelle fasce a scorrimento
  // orizzontale) il fuoco va lì; altrimenti il passo viene allungato, così per leggere un pannello
  // lungo bastano pochi tocchi.
  document.addEventListener("keydown", (e) => {
    if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return;
    if (document.querySelector("dialog[open]")) return;
    const fuoco = document.activeElement;
    const y = window.scrollY;
    requestAnimationFrame(() => {
      const passo = window.scrollY - y;
      if (document.activeElement !== fuoco || passo === 0) return;
      const inVista = fuoco === document.body ? null : vicinoInVerticale(fuoco, passo > 0, true);
      if (inVista) return inVista.focus();
      window.scrollBy(0, Math.sign(passo) * Math.round(window.innerHeight * 0.3));
    });
  });

  // le aree a scorrimento orizzontale senza elementi attivi si scorrono con le frecce quando hanno il fuoco
  for (const area of document.querySelectorAll(".scorri")) area.tabIndex = 0;

  // Tasto Indietro (chiamato dall'attività Android): chiude ciò che è aperto, altrimenti torna alle schede.
  // Risponde true se ha fatto qualcosa, false se si era già in cima sulla scheda attiva.
  window.__tvIndietro = () => {
    const dialogo = document.querySelector("dialog[open]");
    if (dialogo) {
      dialogo.close("annulla");
      return true;
    }
    const risultati = document.getElementById("risultati");
    const scheda = schedaAttiva();
    const aperti = risultati && !risultati.hidden;
    if (!aperti && window.scrollY === 0 && document.activeElement === scheda) return false;
    if (aperti) {
      risultati.hidden = true;
      risultati.replaceChildren();
    }
    window.scrollTo(0, 0);
    scheda?.focus();
    return true;
  };

  focusIniziale();
}

// L'elemento selezionabile più vicino sotto (o sopra) a quello dato; con soloInVista si considerano
// soltanto quelli già visibili sullo schermo.
function vicinoInVerticale(da, giu, soloInVista = false) {
  const r = da.getBoundingClientRect();
  const ambito = da.closest("dialog") ?? document;
  let migliore = null;
  let minimo = Infinity;
  for (const c of ambito.querySelectorAll("a[href], button, input, select, summary, [tabindex]")) {
    if (c === da || c.tabIndex < 0 || c.disabled || !c.getClientRects().length) continue;
    const q = c.getBoundingClientRect();
    if (soloInVista && (q.bottom <= 0 || q.top >= window.innerHeight || q.right <= 0 || q.left >= window.innerWidth)) continue;
    const salto = giu ? q.top - r.bottom : r.top - q.bottom;
    if (salto < -1) continue;
    const scarto = Math.max(0, q.left - r.right, r.left - q.right);
    const distanza = salto + scarto * 2;
    if (distanza < minimo) {
      minimo = distanza;
      migliore = c;
    }
  }
  return migliore;
}

// All'avvio il fuoco parte dalla scheda attiva (richiamata anche a dati caricati: il primo tentativo
// può cadere prima che la finestra abbia il fuoco).
export function focusIniziale() {
  if (!TV) return;
  const a = document.activeElement;
  if (!a || a === document.body) schedaAttiva()?.focus();
}
