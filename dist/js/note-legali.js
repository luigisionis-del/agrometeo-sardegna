// Avvertenze d'uso, privacy e licenze: mostrate al primo avvio e riapribili dal piè di pagina.
import { salvaSpazio, stato } from "./stato.js";

// Aumentare quando il testo delle avvertenze cambia in modo sostanziale: verrà mostrato di nuovo a tutti.
const VERSIONE_AVVERTENZE = 1;

const dialogo = () => document.getElementById("note-legali");

export function initNoteLegali() {
  document.getElementById("apri-note-legali").addEventListener("click", () => dialogo().showModal());
  dialogo().addEventListener("close", () => {
    if (dialogo().returnValue !== "ok" || stato.spazio.avvertenze_accettate >= VERSIONE_AVVERTENZE) return;
    stato.spazio.avvertenze_accettate = VERSIONE_AVVERTENZE;
    salvaSpazio();
  });
  // chiuso con Esc o Indietro senza conferma: verrà riproposto al prossimo avvio
  if ((stato.spazio.avvertenze_accettate ?? 0) < VERSIONE_AVVERTENZE) dialogo().showModal();
}
