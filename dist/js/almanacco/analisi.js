// Analisi del report per l'almanacco: funzioni pure che traducono i dati meteo e del suolo
// in consigli agronomici. Nessun accesso al DOM.

const somma = (a) => a.reduce((s, v) => s + (Number.isFinite(v) ? v : 0), 0);
const minimo = (a) => Math.min(...a.filter(Number.isFinite));
const massimo = (a) => Math.max(...a.filter(Number.isFinite));
const fmt = (v, d = 1) => (Number.isFinite(v) ? v.toLocaleString("it-IT", { minimumFractionDigits: d, maximumFractionDigits: d }) : "—");
const giornoBreve = (iso) => new Date(`${iso.slice(0, 10)}T12:00:00`).toLocaleDateString("it-IT", { weekday: "short", day: "numeric" });
const oraBreve = (iso) => new Date(iso).toLocaleString("it-IT", { weekday: "short", hour: "2-digit", minute: "2-digit" });
const ORDINE = { verde: 0, inattivo: 0, giallo: 1, arancio: 2, rosso: 3 };
const peggiore = (...livelli) => livelli.reduce((a, b) => (ORDINE[b] > ORDINE[a] ? b : a), "verde");

/** THI con la stessa formula del backend (NRC). */
export const thi = (t, rh) => 1.8 * t + 32 - (0.55 - 0.0055 * rh) * (1.8 * t - 26);

/** Stato idrico di uno strato, con la stessa soglia di tempera del backend (40 % dell'acqua disponibile). */
export function statoUmidita(u, suolo) {
  const { punto_appassimento: pa, capacita_campo: cc } = suolo;
  const frazione = (u - pa) / (cc - pa);
  const stato = u > cc ? "eccesso" : frazione >= 0.4 ? "tempera" : "secco";
  return { stato, frazione };
}

/**
 * Periodo di Smith (peronospora della patata e del pomodoro): due giorni consecutivi con
 * temperatura minima ≥ 10 °C e almeno 11 ore con umidità relativa ≥ 90 %.
 */
export function periodiSmith(ore) {
  const giorni = new Map();
  for (const o of ore) {
    const d = o.time.slice(0, 10);
    const g = giorni.get(d) ?? { data: d, n: 0, tmin: Infinity, umide: 0 };
    g.n++;
    g.tmin = Math.min(g.tmin, o.t);
    if (o.rh >= 90) g.umide++;
    giorni.set(d, g);
  }
  const completi = [...giorni.values()].filter((g) => g.n >= 20);
  const smith = completi.map((g) => ({ ...g, ok: g.tmin >= 10 && g.umide >= 11 }));
  // giorni favorevoli consecutivi (almeno 2) riuniti in un unico periodo
  const periodi = [];
  let inizio = null;
  smith.forEach((g, i) => {
    if (g.ok && inizio == null) inizio = i;
    const chiude = !g.ok || i === smith.length - 1;
    if (chiude && inizio != null) {
      const fine = g.ok ? i : i - 1;
      if (fine > inizio) periodi.push({ inizio: smith[inizio].data, fine: smith[fine].data });
      inizio = null;
    }
  });
  return { giorni: smith, periodi };
}

/** Raccoglie in un oggetto tutto ciò che serve ai consigli. */
export function contesto(r) {
  const oggi = r.ora_corrente.slice(0, 10);
  const mese = Number(oggi.slice(5, 7));
  const i = r.indice_ora;
  const prossime72 = r.ore.slice(i, i + 72);
  const prossime48 = r.ore.slice(i, i + 48);
  const futuri = r.giorni.filter((g) => g.data > oggi);
  const passati = r.giorni.filter((g) => g.data < oggi);
  const suolo = r.parametri.suolo;

  const tMin72 = minimo(prossime72.map((o) => o.t));
  const oraMin = prossime72.find((o) => o.t === tMin72)?.time;
  const thiOre = prossime72.map((o) => thi(o.t, o.rh));
  const thiMax72 = massimo(thiOre);

  // Rugiada o brina al mattino (4–9): aria quasi satura sulla vegetazione.
  const mattineUmide = new Set(
    prossime72
      .filter((o) => {
        const h = Number(o.time.slice(11, 13));
        return h >= 4 && h <= 9 && (o.rh >= 95 || o.t - o.td <= 1.5);
      })
      .map((o) => o.time.slice(0, 10)),
  );

  // Giorni consecutivi asciutti a partire da domani (finestra di fienagione).
  let asciutti = 0;
  for (const g of futuri) {
    if (g.pioggia < 1) asciutti++;
    else break;
  }
  const finestra = futuri.slice(0, asciutti);

  const profondo = statoUmidita(r.suolo.umidita[3], suolo);
  const superficiale = statoUmidita(r.suolo.umidita[2], suolo);

  return {
    oggi,
    mese,
    t6: r.suolo.semina.t_suolo_6cm,
    statoSemina: r.suolo.semina,
    superficiale,
    profondo,
    tMin72,
    oraMin,
    tMax72: massimo(prossime72.map((o) => o.t)),
    pioggia48: somma(prossime48.map((o) => o.pioggia)),
    pioggia72: somma(prossime72.map((o) => o.pioggia)),
    pioggiaPassata7: somma(passati.map((g) => g.pioggia)),
    pioggiaFutura7: somma(futuri.map((g) => g.pioggia)),
    pioggiaFutura3: somma(futuri.slice(0, 3).map((g) => g.pioggia)),
    et0Futura7: somma(futuri.map((g) => g.et0)),
    pioggiaEfficaceFutura7: somma(futuri.map((g) => g.pioggia_efficace)),
    giorniFuturi: futuri.length,
    thiOra: r.kpi.thi,
    thiMax72,
    oreUmide72: prossime72.filter((o) => o.rh >= 90).length,
    mattineUmide: [...mattineUmide],
    giorniBui: futuri.filter((g) => g.ore_sole < 3 && g.t_max < 14),
    finestraSecca: finestra,
    giorniLavorabili: futuri.filter((g) => g.semina.stato_umidita === "tempera"),
    smith: periodiSmith(r.ore),
  };
}

// --------------------------------------------------------------------- schede colturali

const inMese = (mesi, m) => Boolean(mesi?.includes(m));

/** La coltura è presumibilmente in campo nel mese (esclusa la fase di semenzaio protetto). */
export function inCampo(scheda, mese) {
  const c = scheda.calendario;
  if (!c) return false;
  const attivi = new Set([...(c.semina ?? []), ...(c.trapianto ?? []), ...(c.raccolta ?? []), ...(c.utilizzo ?? [])]);
  if (attivi.has(mese)) return true;
  // mesi intermedi tra impianto e raccolta (es. pomodoro a giugno): da ogni mese di impianto
  // si avanza fino al primo mese di raccolta o utilizzo.
  const fine = new Set([...(c.raccolta ?? []), ...(c.utilizzo ?? [])]);
  for (const a of [...(c.semina ?? []), ...(c.trapianto ?? [])]) {
    for (let k = 0; k < 12; k++) {
      const mm = ((a - 1 + k) % 12) + 1;
      if (mm === mese) return true;
      if (fine.has(mm)) break;
    }
  }
  return false;
}

/** Confronto tra la temperatura del suolo e le soglie della coltura. */
export function verificaSuolo(scheda, t6) {
  const s = scheda.tSuolo;
  if (!s || !Number.isFinite(t6)) return null;
  if (t6 < s.min) return { ok: false, motivo: "freddo", testo: `Suolo freddo: ${fmt(t6)} °C, servono almeno ${s.min} °C` };
  if (s.max != null && t6 > s.max) return { ok: false, motivo: "caldo", testo: `Suolo troppo caldo: ${fmt(t6)} °C, oltre ${s.max} °C la germinazione è inibita` };
  return { ok: true, testo: `Suolo a ${fmt(t6)} °C, sopra la minima di ${s.min} °C` };
}

/** Indicatori sintetici («chip») per la testata di una scheda colturale. */
export function valutaColtura(scheda, ctx) {
  const c = scheda.calendario;
  if (!c || !ctx) return [];
  const m = ctx.mese;
  const chip = [];
  const semina = inMese(c.semina, m);
  const trapianto = inMese(c.trapianto, m);
  if (semina) chip.push({ livello: "verde", testo: "Epoca di semina" });
  if (trapianto) chip.push({ livello: "verde", testo: "Epoca di trapianto" });
  if (inMese(c.semenzaio, m)) chip.push({ livello: "verde", testo: "Semina in semenzaio" });
  if (inMese(c.raccolta, m)) chip.push({ livello: "inattivo", testo: "In raccolta" });
  if (inMese(c.utilizzo, m)) chip.push({ livello: "inattivo", testo: "In utilizzo" });
  if (!chip.length) chip.push({ livello: "inattivo", testo: "Fuori epoca" });

  if (semina || trapianto) {
    const v = verificaSuolo(scheda, ctx.t6);
    if (v) chip.push({ livello: v.ok ? "verde" : "giallo", testo: v.testo });
  }
  if (scheda.gelo != null && Number.isFinite(ctx.tMin72) && inCampo(scheda, m) && ctx.tMin72 <= scheda.gelo + 2) {
    chip.push({
      livello: ctx.tMin72 <= scheda.gelo ? "rosso" : "giallo",
      testo: `Minima prevista ${fmt(ctx.tMin72)} °C (danni sotto ${scheda.gelo} °C)`,
    });
  }
  return chip;
}

// --------------------------------------------------------------------- consigli del giorno

const NOMI_STATO = { tempera: "in tempera", secco: "secco", eccesso: "in eccesso idrico" };

function consiglioSemina(ctx, schede) {
  const m = ctx.mese;
  const candidate = schede.filter((s) => s.tSuolo && (inMese(s.calendario?.semina, m) || inMese(s.calendario?.trapianto, m)));
  const pronte = [];
  const attesa = [];
  for (const s of candidate) {
    const v = verificaSuolo(s, ctx.t6);
    (v?.ok ? pronte : attesa).push({ id: s.id, nome: s.nome, classe: v?.ok ? "ok" : "attesa", titolo: v?.testo });
  }
  const semenzaio = schede.filter((s) => inMese(s.calendario?.semenzaio, m)).map((s) => ({ id: s.id, nome: s.nome, titolo: "Semina in semenzaio protetto" }));
  const stato = ctx.statoSemina.stato_umidita;
  const livello = stato === "tempera" ? "verde" : stato === "eccesso" ? "rosso" : "giallo";
  const testi = [];
  if (stato === "tempera") testi.push("Il letto di semina è in tempera: condizioni adatte a seminare le colture in elenco.");
  else if (stato === "secco") testi.push("Lo strato superficiale è secco: seminare in irriguo o attendere una pioggia utile (20–25 mm per le colture in asciutto), per evitare emergenze scalari o una falsa partenza.");
  else testi.push("Suolo in eccesso idrico: attendere che sgrondi prima di entrare con le macchine, per non compattarlo.");
  if (pronte.length) testi.push("Temperatura del suolo adatta per:");
  return {
    id: "semina",
    titolo: "Cosa seminare o trapiantare ora",
    livello,
    dato: `Suolo a 6 cm ${fmt(ctx.t6)} °C · strato 3–9 cm ${NOMI_STATO[stato] ?? stato}`,
    testi,
    colture: pronte,
    dopo: [
      attesa.length ? { testo: "In epoca, ma con suolo non ancora idoneo:", colture: attesa } : null,
      semenzaio.length ? { testo: "In semenzaio protetto:", colture: semenzaio } : null,
    ].filter(Boolean),
    perche: "La temperatura del suolo a 6 cm governa la germinazione: sotto la minima cardinale il seme assorbe acqua ma non cresce, e resta esposto a marciumi e insetti.",
  };
}

function consiglioLavorazioni(ctx) {
  const p = ctx.profondo;
  const pioggiaForte = ctx.pioggia72 > 25;
  let livello = p.stato === "tempera" ? "verde" : p.stato === "eccesso" ? "rosso" : "giallo";
  const testi = [];
  if (p.stato === "tempera") testi.push("Lo strato 9–27 cm, quello interessato dall'aratura, è in tempera: buone condizioni per arare o ripuntare.");
  else if (p.stato === "eccesso") testi.push("Strato profondo sopra la capacità di campo: arare ora formerebbe una suola compatta e zolle lisciate. Attendere qualche giorno asciutto.");
  else testi.push("Strato profondo secco: l'aratura darebbe zolle grossolane e alti consumi. Nei suoli compattati è invece il momento giusto per una ripuntatura, che fessura meglio il suolo asciutto.");
  if (pioggiaForte) {
    livello = peggiore(livello, "arancio");
    testi.push(`Previsti ${fmt(ctx.pioggia72, 0)} mm in 72 ore: evitare di lasciare suolo appena lavorato sui versanti, dove il rischio di erosione è massimo.`);
  }
  if (ctx.giorniLavorabili.length) testi.push(`Giorni con letto di semina in tempera: ${ctx.giorniLavorabili.map((g) => giornoBreve(g.data)).join(", ")}.`);
  return {
    id: "lavorazioni",
    titolo: "Aratura e lavorazioni",
    livello,
    dato: `Acqua disponibile 9–27 cm: ${fmt(Math.max(0, p.frazione * 100), 0)} % · 3–9 cm: ${NOMI_STATO[ctx.superficiale.stato]} · pioggia 72 h ${fmt(ctx.pioggia72)} mm`,
    testi,
    link: "aratura",
    perche: "Un suolo plastico si deforma sotto ruote e versoio: i macropori collassano e si forma la suola di lavorazione, che limita radici e infiltrazione per anni.",
  };
}

function consiglioGelate(ctx, schede) {
  const t = ctx.tMin72;
  const livello = t <= 0 ? "rosso" : t <= 3 ? "giallo" : "verde";
  const sensibili = schede
    .filter((s) => s.gelo != null && inCampo(s, ctx.mese) && t <= s.gelo + 2)
    .map((s) => ({ id: s.id, nome: s.nome, classe: t <= s.gelo ? "attesa" : "", titolo: `danni sotto ${s.gelo} °C` }));
  const testi = [];
  if (livello === "verde") testi.push("Nessun rischio di gelo nelle prossime 72 ore.");
  else {
    testi.push(`Minima prevista ${fmt(t)} °C (${ctx.oraMin ? oraBreve(ctx.oraMin) : ""}).`);
    testi.push("Coprire le orticole sensibili con tessuto non tessuto (guadagno di 1–3 °C), raccogliere i carciofi pronti, non lavorare il suolo la sera prima: un suolo compatto e umido cede più calore di uno appena smosso.");
  }
  if (sensibili.length) testi.push("Colture in campo vicine alla soglia di danno:");
  return {
    id: "gelate",
    titolo: "Gelate e protezione",
    livello,
    dato: `Minima 72 h ${fmt(t)} °C · massima ${fmt(ctx.tMax72)} °C`,
    testi,
    colture: sensibili,
    perche: "Le gelate per irraggiamento si formano con cielo sereno e calma di vento: il suolo perde calore verso il cielo e l'aria fredda, più densa, ristagna negli avvallamenti.",
  };
}

function consiglioAcqua(ctx) {
  const f = ctx.profondo.frazione;
  const fabbisogno = Math.max(0, ctx.et0Futura7 * 1.1 - ctx.pioggiaEfficaceFutura7);
  const livello = f > 1 ? "verde" : f < 0.3 ? "arancio" : f < 0.5 ? "giallo" : "verde";
  const testi = [];
  if (f > 1) testi.push("Suolo sopra la capacità di campo: nessuna irrigazione, controllare i ristagni.");
  else if (f < 0.5) testi.push("Nello strato radicale resta meno della metà dell'acqua disponibile: per le orticole e la medica irrigua è il momento di intervenire.");
  else testi.push("Riserva idrica sufficiente nello strato radicale.");
  testi.push(`Per una coltura a pieno sviluppo (Kc 1,1) nei prossimi ${ctx.giorniFuturi} giorni servono circa ${fmt(fabbisogno, 0)} mm (${fmt(fabbisogno, 0)} L/m²) oltre alla pioggia efficace prevista.`);
  return {
    id: "acqua",
    titolo: "Acqua e irrigazione",
    livello,
    dato: `Acqua disponibile 9–27 cm: ${fmt(Math.max(0, f * 100), 0)} % · ET0 prevista ${fmt(ctx.et0Futura7, 0)} mm · pioggia ${fmt(ctx.pioggiaFutura7, 0)} mm`,
    testi,
    link: "principi-orticoltura",
    perche: "Secondo FAO-56 la maggior parte delle orticole inizia a ridurre la traspirazione quando si esaurisce il 30–50 % dell'acqua disponibile: irrigare prima di quella soglia evita lo stress.",
  };
}

function consiglioAzoto(ctx) {
  const p = ctx.pioggia72;
  let livello;
  const testi = [];
  if (ctx.statoSemina.stato_umidita === "eccesso") {
    livello = "arancio";
    testi.push("Suolo saturo: rinviare la distribuzione, l'azoto verrebbe perso per dilavamento o denitrificazione e il passaggio delle macchine compatterebbe il suolo.");
  } else if (p > 25) {
    livello = "arancio";
    testi.push(`Previsti ${fmt(p, 0)} mm: rinviare la concimazione in copertura. Piogge intense spingono i nitrati sotto le radici, soprattutto nei suoli sciolti.`);
  } else if (p >= 5) {
    livello = "verde";
    testi.push(`Finestra favorevole: una pioggia moderata (${fmt(p, 0)} mm in 72 ore) incorporerà il concime nel suolo, riducendo le perdite di ammoniaca dell'urea.`);
  } else {
    livello = "giallo";
    testi.push("Pioggia scarsa: l'urea lasciata in superficie perde ammoniaca per volatilizzazione. Interrarla, irrigare con 10–15 mm subito dopo o preferire concimi nitrici-ammoniacali.");
  }
  if ([1, 2, 3, 4].includes(ctx.mese)) testi.push("Per cereali ed erbai è la stagione delle coperture: distribuire tra fine accestimento e inizio levata.");
  return {
    id: "azoto",
    titolo: "Concimazione azotata in copertura",
    livello,
    dato: `Pioggia 48 h ${fmt(ctx.pioggia48)} mm · 72 h ${fmt(p)} mm`,
    testi,
    link: "avena",
    perche: "Il nitrato è solubile e si muove con l'acqua; l'urea in superficie si trasforma in ammoniaca che si disperde nell'aria. Una pioggia di 5–20 mm porta l'azoto alle radici senza dilavarlo.",
  };
}

function consiglioPeronospora(ctx, schede) {
  const futuri = ctx.smith.periodi.filter((p) => p.fine >= ctx.oggi);
  const recenti = ctx.smith.periodi.filter((p) => p.fine < ctx.oggi);
  let livello = "verde";
  const testi = [];
  if (futuri.length) {
    livello = "arancio";
    testi.push(`Periodo di Smith in corso o previsto (${futuri.map((p) => `${giornoBreve(p.inizio)}–${giornoBreve(p.fine)}`).join(", ")}): condizioni favorevoli alle infezioni di peronospora su pomodoro e patata. Proteggere la vegetazione prima dell'evento.`);
  } else if (recenti.length) {
    livello = "giallo";
    testi.push(`Periodo di Smith registrato negli ultimi giorni (${recenti.map((p) => giornoBreve(p.fine)).join(", ")}): controllare le colture per le prime macchie nei prossimi 5–7 giorni.`);
  } else if (ctx.oreUmide72 >= 24) {
    livello = "giallo";
    testi.push(`${ctx.oreUmide72} ore con umidità ≥ 90 % nelle prossime 72: rischio moderato di malattie fogliari. Evitare l'irrigazione a pioggia.`);
  } else testi.push("Nessun periodo di Smith previsto: rischio di peronospora basso.");
  const colture = schede.filter((s) => ["pomodoro", "patata"].includes(s.id) && inCampo(s, ctx.mese)).map((s) => ({ id: s.id, nome: s.nome }));
  if (!colture.length) testi.push("Pomodoro e patata non sono di norma in campo in questo mese: indicazione utile per serre e cicli fuori stagione.");
  return {
    id: "peronospora",
    titolo: "Peronospora di pomodoro e patata",
    livello,
    dato: `Ore con UR ≥ 90 % nelle prossime 72: ${ctx.oreUmide72}`,
    testi,
    colture,
    perche: "Phytophthora infestans produce spore e infetta quando l'aria resta satura per molte ore con temperature miti: il periodo di Smith (2 giorni con minima ≥ 10 °C e almeno 11 ore con UR ≥ 90 %) ne è l'indicatore classico.",
  };
}

function consiglioOvini(ctx) {
  const t = ctx.thiMax72;
  let livello = t >= 78 ? "rosso" : t >= 72 ? "arancio" : t >= 68 ? "giallo" : "verde";
  const testi = [];
  if (t >= 72) testi.push(`THI massimo previsto ${fmt(t)}: pascolo nelle ore fresche (alba, sera o notte), ombra e acqua fresca a volontà.`);
  else testi.push(`THI massimo previsto ${fmt(t)}: nessuno stress da caldo.`);
  if (ctx.mattineUmide.length) {
    livello = peggiore(livello, "giallo");
    testi.push(`Rugiada o brina al mattino (${ctx.mattineUmide.map(giornoBreve).join(", ")}): portare il gregge su erbai ricchi di leguminose solo dopo che l'erba si è asciugata, e con fieno già nel rumine, per prevenire il meteorismo.`);
  }
  if (ctx.giorniBui.length >= 2) {
    livello = peggiore(livello, "giallo");
    testi.push(`${ctx.giorniBui.length} giorni freddi e poco soleggiati in arrivo: sugli erbai di graminacee concimati con azoto possono accumularsi nitrati. Pascolare nel pomeriggio e introdurre gradualmente.`);
  }
  return {
    id: "ovini",
    titolo: "Pecore al pascolo",
    livello,
    dato: `THI ora ${fmt(ctx.thiOra)} · max 72 h ${fmt(t)}`,
    testi,
    link: "rischi-sanitari",
    perche: "Il THI combina temperatura e umidità: oltre circa 72 la pecora riduce ingestione e latte per limitare il calore prodotto dalla digestione.",
  };
}

function consiglioErbai(ctx) {
  if (![8, 9, 10, 11, 12].includes(ctx.mese)) return null;
  const pioggia = ctx.pioggiaPassata7 + ctx.pioggiaFutura3;
  const livello = pioggia >= 20 ? "verde" : pioggia >= 10 ? "giallo" : "arancio";
  const testi = [];
  if (pioggia >= 20) testi.push(`Pioggia sufficiente (${fmt(pioggia, 0)} mm tra ultimi 7 giorni e prossimi 3): buona finestra per seminare gli erbai autunno-vernini.`);
  else {
    testi.push(`Solo ${fmt(pioggia, 0)} mm tra ultimi 7 giorni e prossimi 3: la semina in asciutto rischia una falsa partenza.`);
    testi.push(ctx.pioggiaFutura7 >= 20
      ? `La previsione a 7 giorni indica ${fmt(ctx.pioggiaFutura7, 0)} mm: si può seminare in asciutto poco prima dell'evento.`
      : "Nessuna pioggia significativa nei prossimi 7 giorni: seminare solo con irrigazione di soccorso.");
  }
  if (ctx.mese >= 11) testi.push("Gli erbai seminati ora arrivano all'inverno poco sviluppati: preferire specie precoci (orzo, loietto italico) e dosi di seme più alte.");
  return {
    id: "erbai",
    titolo: "Semina degli erbai",
    livello,
    dato: `Pioggia ultimi 7 giorni ${fmt(ctx.pioggiaPassata7, 0)} mm · prossimi 3 giorni ${fmt(ctx.pioggiaFutura3, 0)} mm`,
    testi,
    link: "miscugli-erbai",
    perche: "Per un'emergenza uniforme il seme deve restare in un suolo umido per diversi giorni: 20–25 mm cumulati bagnano lo strato di semina abbastanza a lungo.",
  };
}

function consiglioFienagione(ctx) {
  const n = ctx.finestraSecca.length;
  const stagione = ctx.mese >= 4 && ctx.mese <= 9;
  if (!stagione && n < 3) return null;
  const livello = n >= 4 ? "verde" : n === 3 ? "giallo" : "rosso";
  const et0 = n ? ctx.finestraSecca.reduce((s, g) => s + g.et0, 0) / n : 0;
  const testi = [];
  if (n >= 4) testi.push(`${n} giorni asciutti consecutivi da domani (ET0 media ${fmt(et0)} mm/giorno): finestra adatta per il fieno.`);
  else if (n === 3) testi.push("3 giorni asciutti: sufficienti per il fasciato, al limite per il fieno di leguminose.");
  else testi.push("Pioggia entro 2 giorni: non falciare per il fieno.");
  if (!stagione) testi.push("Fuori dalla stagione principale di fienagione (aprile–settembre).");
  return {
    id: "fieno",
    titolo: "Finestra di fienagione",
    livello,
    dato: n ? `Asciutto da ${giornoBreve(ctx.finestraSecca[0].data)} a ${giornoBreve(ctx.finestraSecca[n - 1].data)}` : "Pioggia prevista a breve",
    testi,
    link: "fieno-insilato",
    perche: "Il foraggio deve scendere dal 75–80 % al 18–20 % di umidità: con buona evaporazione (ET0 alta) servono 3–4 giorni senza pioggia, che lava zuccheri e fa cadere le foglie.",
  };
}

/** Elenco ordinato dei consigli del giorno. */
export function consigliOggi(ctx, schede) {
  return [
    consiglioSemina(ctx, schede),
    consiglioLavorazioni(ctx),
    consiglioGelate(ctx, schede),
    consiglioAcqua(ctx),
    consiglioAzoto(ctx),
    consiglioErbai(ctx),
    consiglioOvini(ctx),
    consiglioPeronospora(ctx, schede),
    consiglioFienagione(ctx),
  ].filter(Boolean);
}
