// Stato dell'applicazione e persistenza (debounced) dello spazio di lavoro.
import { api } from "./api.js";

export const stato = {
  spazio: null,
  report: null,
  ascoltatori: new Set(),
};

export function aggiornaReport(report) {
  stato.report = report;
  for (const f of stato.ascoltatori) f(report);
}

export function suReport(f) {
  stato.ascoltatori.add(f);
}

let timerSalvataggio;
export function salvaSpazio() {
  clearTimeout(timerSalvataggio);
  timerSalvataggio = setTimeout(() => {
    api.salvaSpazioLavoro(stato.spazio).catch((e) => console.error("Salvataggio spazio di lavoro:", e));
  }, 400);
}
