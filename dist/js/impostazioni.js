// Dialogo dei parametri agronomici e dei layer cartografici.
import { salvaSpazio, stato } from "./stato.js";

const dialogo = () => document.getElementById("impostazioni");
const form = () => document.getElementById("form-impostazioni");

export function initImpostazioni({ onParametri, onLayer }) {
  document.getElementById("apri-impostazioni").addEventListener("click", apri);
  const f = form();
  f.kc_preset.addEventListener("change", () => { if (f.kc_preset.value) f.kc.value = f.kc_preset.value; });
  f.kc.addEventListener("input", () => { f.kc_preset.value = selezionaPreset(f.kc_preset, f.kc.value); });
  f.suolo_preset.addEventListener("change", () => {
    if (!f.suolo_preset.value) return;
    [f.suolo_pa.value, f.suolo_cc.value] = f.suolo_preset.value.split(",");
  });
  dialogo().addEventListener("close", () => {
    if (dialogo().returnValue !== "ok") return;
    const { parametriCambiati, layerCambiati } = applica();
    salvaSpazio();
    if (parametriCambiati) onParametri();
    if (layerCambiati) onLayer();
  });
}

function selezionaPreset(select, valore) {
  const opzione = [...select.options].find((o) => o.value && Number(o.value) === Number(valore));
  return opzione ? opzione.value : "";
}

function apri() {
  const f = form();
  const p = stato.spazio.parametri;
  const risolti = stato.report?.parametri ?? p;
  const l = stato.spazio.layer;
  f.germogli_cm.value = p.germogli_cm;
  f.inizio_vite.value = p.inizio_vite ?? risolti.inizio_vite ?? "";
  f.inizio_foraggere.value = p.inizio_foraggere ?? risolti.inizio_foraggere ?? "";
  f.kc.value = p.kc;
  f.kc_preset.value = selezionaPreset(f.kc_preset, p.kc);
  f.pioggia_soglia.value = p.pioggia.soglia_mm;
  f.pioggia_efficienza.value = p.pioggia.efficienza;
  f.suolo_pa.value = p.suolo.punto_appassimento;
  f.suolo_cc.value = p.suolo.capacita_campo;
  f.suolo_preset.value = [...f.suolo_preset.options].find((o) => o.value === `${p.suolo.punto_appassimento.toFixed(2)},${p.suolo.capacita_campo.toFixed(2)}`)?.value ?? "";
  f.base.value = l.base;
  f.vento.checked = l.vento;
  f.wms_attivo.checked = l.wms_attivo;
  f.wms_url.value = l.wms_url;
  f.wms_layer.value = l.wms_layer;
  dialogo().showModal();
}

const numero = (input, predefinito) => {
  const v = Number(input.value);
  return input.value !== "" && Number.isFinite(v) ? v : predefinito;
};

function applica() {
  const f = form();
  const p = stato.spazio.parametri;
  const risolti = stato.report?.parametri ?? {};
  const prima = JSON.stringify(p);
  const primaLayer = JSON.stringify(stato.spazio.layer);

  p.germogli_cm = Math.max(0, numero(f.germogli_cm, p.germogli_cm));
  p.kc = numero(f.kc, p.kc);
  p.pioggia = { soglia_mm: numero(f.pioggia_soglia, p.pioggia.soglia_mm), efficienza: numero(f.pioggia_efficienza, p.pioggia.efficienza) };
  const pa = numero(f.suolo_pa, p.suolo.punto_appassimento);
  const cc = numero(f.suolo_cc, p.suolo.capacita_campo);
  if (cc > pa) p.suolo = { punto_appassimento: pa, capacita_campo: cc };
  // Una data uguale al predefinito calcolato resta "automatica" e scorre con le stagioni.
  const data = (input, chiave) => (!input.value || (p[chiave] == null && input.value === risolti[chiave]) ? null : input.value);
  p.inizio_vite = data(f.inizio_vite, "inizio_vite");
  p.inizio_foraggere = data(f.inizio_foraggere, "inizio_foraggere");

  Object.assign(stato.spazio.layer, {
    base: f.base.value,
    vento: f.vento.checked,
    wms_attivo: f.wms_attivo.checked,
    wms_url: f.wms_url.value.trim(),
    wms_layer: f.wms_layer.value.trim(),
  });
  return { parametriCambiati: JSON.stringify(p) !== prima, layerCambiati: JSON.stringify(stato.spazio.layer) !== primaLayer };
}
