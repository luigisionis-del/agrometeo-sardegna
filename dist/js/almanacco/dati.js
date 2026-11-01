// Contenuti dell'almanacco colturale per la Sardegna.
//
// Valori orientativi per il pieno campo in ambiente mediterraneo sardo, da adattare a quota,
// esposizione, suolo e varietà. Le dosi di concimazione sono riferite a rese medie: il piano
// aziendale va basato sull'analisi del suolo e sul Disciplinare di produzione integrata regionale.
//
// Schema di una scheda:
//   id, nome, latino?, famiglia?, sezione, sintesi
//   tSuolo?   { min, opt, max? }  temperatura del suolo (°C, 6 cm) per semina o trapianto in campo
//   gelo?     soglia (°C aria) sotto cui la coltura in campo subisce danni
//   calendario? { semina, semenzaio, trapianto, raccolta, utilizzo }  mesi 1–12 (Sardegna)
//   kc?       [iniziale, metà ciclo, fine ciclo] (FAO-56 o letteratura, indicativi)
//   dati      [[etichetta, valore], …]  dati rapidi
//   blocchi   [{ titolo, tipo?: "scienza" | "attenzione", punti: [...] }]
//   tabella?  { titolo, intestazioni, righe }

export const SEZIONI = [
  {
    id: "oggi",
    titolo: "Oggi in campo",
    icona: "☀️",
    intro: "Consigli generati incrociando i dati meteo e del suolo della località attiva con le soglie agronomiche delle colture. Si aggiornano a ogni nuovo report.",
  },
  {
    id: "suolo",
    titolo: "Lavorazioni del terreno",
    icona: "🚜",
    intro: "Aratura, lavorazioni conservative e preparazione del letto di semina. Il momento giusto si decide sull'umidità del suolo (lo stato di «tempera»), non sul calendario.",
    principi: [
      ["Lavorare solo in tempera", "Un suolo troppo umido si compatta e si «impasta» sotto il versoio; uno troppo secco forma zolle enormi e consuma più gasolio."],
      ["Lavorare il meno possibile", "Ogni passaggio ossida la sostanza organica e aumenta il rischio di compattamento. Meno passaggi, fatti al momento giusto."],
      ["Proteggere i versanti", "In Sardegna le piogge autunnali intense colpiscono suoli appena lavorati: a girapoggio, con residui in superficie, mai prima di un forte evento piovoso."],
    ],
  },
  {
    id: "cereali",
    titolo: "Cereali: avena e orzo",
    icona: "🌾",
    intro: "Cereali autunno-vernini da granella, da erbaio e da fieno. In Sardegna sfruttano la stagione piovosa (ottobre–maggio) e devono completare il ciclo prima della siccità estiva.",
    principi: [
      ["Seminare con le prime piogge utili", "Servono 20–25 mm cumulati per un'emergenza uniforme; una pioggia leggera seguita da giorni secchi fa germinare e poi seccare le plantule."],
      ["Azoto frazionato", "Poco o nulla alla semina, il resto tra accestimento e inizio levata, quando la coltura lo assorbe davvero: meno perdite per dilavamento."],
      ["Rispettare la levata", "Dopo l'inizio della levata l'apice della spiga sale dal suolo: pascolarlo o tagliarlo basso significa perdere il ricaccio."],
    ],
  },
  {
    id: "foraggere",
    titolo: "Foraggere: trifogli, medica, loietto",
    icona: "🍀",
    intro: "Leguminose che fissano l'azoto atmosferico e graminacee ad alta produzione: la base di pascoli, erbai e fieni per gli allevamenti sardi.",
    principi: [
      ["Leguminose: fosforo sì, azoto no", "La simbiosi con i rizobi fornisce 50–200 kg N/ha l'anno; il fosforo è invece indispensabile per la nodulazione."],
      ["Seme piccolo, letto fine", "Medica, trifogli e loietto vanno seminati a 1–1,5 cm su terreno affinato e rullato: il contatto seme-suolo garantisce l'imbibizione."],
      ["Utilizzo allo stadio giusto", "Resa e qualità vanno in direzioni opposte: più si attende, più biomassa ma meno proteina e digeribilità."],
    ],
  },
  {
    id: "ovini",
    titolo: "Erbai e foraggi per la pecora Sarda",
    icona: "🐑",
    intro: "Programmare la produzione foraggera sul ciclo della pecora di razza Sarda: il picco dei fabbisogni (fine gestazione e inizio lattazione, ottobre–gennaio) cade proprio quando il pascolo naturale produce poco.",
    principi: [
      ["Colmare il vuoto autunno-invernale", "Erbai seminati presto (settembre–ottobre) sono pascolabili 50–70 giorni dopo l'emergenza, quando il pascolo naturale è ancora scarso."],
      ["Cambi di dieta graduali", "Passare a un erbaio lussureggiante in 7–15 giorni, con fieno a disposizione: si prevengono meteorismo, acidosi ed enterotossiemia."],
      ["Conservare l'eccesso primaverile", "Ad aprile–maggio la crescita supera il consumo: fieno e fasciato trasformano l'eccedenza in scorte per l'estate e l'autunno."],
    ],
  },
  {
    id: "orticole",
    titolo: "Orticole",
    icona: "🍅",
    intro: "Le principali specie orticole di pieno campo in Sardegna, con epoche, temperature cardinali, consumi idrici (Kc FAO-56) e difesa. Per serra e tunnel le epoche si anticipano o si estendono.",
    principi: [
      ["Ruotare per famiglie", "Non tornare con la stessa famiglia sullo stesso appezzamento prima di 3–4 anni: Fusarium, Verticillium e nematodi si accumulano nel suolo."],
      ["Goccia e pacciamatura", "L'irrigazione a goccia raggiunge efficienze dell'85–95 % e lascia la chioma asciutta: meno peronospora e meno acqua sprecata."],
      ["Rispettare le temperature del suolo", "Trapiantare solanacee e cucurbitacee in un suolo freddo blocca le radici per settimane: meglio una settimana dopo che una prima."],
    ],
  },
  {
    id: "canapa",
    titolo: "Canapa (Cannabis sativa L.)",
    icona: "🌿",
    intro: "Canapa industriale da fibra e da seme con varietà certificate. Coltura da rinnovo che migliora la struttura del suolo e lascia il campo pulito per il cereale successivo.",
  },
  {
    id: "fonti",
    titolo: "Metodo e fonti",
    icona: "📚",
    intro: "Da dove vengono i numeri dell'almanacco e come sono collegati ai dati meteo dell'applicazione.",
  },
];

/** Mesi in italiano (indice 0 = gennaio). */
export const MESI = ["Gen", "Feb", "Mar", "Apr", "Mag", "Giu", "Lug", "Ago", "Set", "Ott", "Nov", "Dic"];

export const SCHEDE = [
  // ============================================================ LAVORAZIONI
  {
    id: "aratura",
    nome: "Aratura",
    sezione: "suolo",
    sintesi: "Quando, quanto in profondità e come arare per preparare il terreno senza danneggiarne la struttura.",
    dati: [
      ["Profondità seminativi", "25–35 cm"],
      ["Profondità erbai e cereali", "20–30 cm (spesso sufficiente)"],
      ["Scasso per impianti arborei", "50–80 cm, una tantum"],
      ["Epoca per semine autunnali", "fine estate – autunno, in tempera"],
      ["Velocità di lavoro", "5–7 km/h"],
      ["Pressione pneumatici", "0,8–1,2 bar in campo"],
    ],
    blocchi: [
      {
        titolo: "Quando arare: lo stato di tempera",
        punti: [
          "Il suolo è in tempera quando l'umidità è sotto la capacità di campo ma sopra la soglia in cui diventa duro e zolloso; nell'app corrisponde al semaforo verde «Lavorabilità» (strato 3–9 cm) e alla valutazione dello strato 9–27 cm in «Oggi in campo».",
          "Prova della manciata: prendere terra a 10–15 cm e stringerla. In tempera si modella senza appiccicarsi e si sbriciola sotto il pollice. Se forma un cilindretto lucido è troppo umida; se non si aggrega affatto è troppo secca.",
          "Il versoio pulito e la fetta che si rovescia sgretolandosi sono il segnale in campo; una fetta lucida e compatta indica eccesso d'acqua.",
          "Suoli sabbiosi: ampia finestra di tempera, si lavorano anche poco dopo la pioggia. Suoli argillosi (es. vertisuoli del Campidano): finestra di pochi giorni, da cogliere.",
        ],
      },
      {
        titolo: "Epoca per tipo di suolo",
        punti: [
          "Argillosi: arare a fine estate o subito dopo le prime piogge, quando la tempera c'è: i cicli bagnato/asciutto dell'autunno e del freddo invernale affinano naturalmente le zolle.",
          "Franchi: lavorare in autunno 2–4 settimane prima della semina, così da poter fare una falsa semina.",
          "Sabbiosi: lavorazioni leggere e vicine alla semina, per non esporre a lungo il suolo all'erosione eolica e alla perdita di umidità.",
          "Per le colture primaverili (orticole, canapa) lasciare il terreno coperto in inverno e lavorare a fine inverno, in tempera.",
        ],
      },
      {
        titolo: "Perché non arare un suolo bagnato",
        tipo: "scienza",
        punti: [
          "Ruote e versoio deformano un suolo plastico: i macropori (le «vie» di aria e acqua) collassano. Sotto il 10 % circa di porosità ad aria le radici soffrono di asfissia.",
          "Il fondo del solco, lisciato dal versoio, forma la «suola di lavorazione»: uno strato compatto che le radici superano a fatica. Una resistenza alla penetrazione superiore a circa 2 MPa limita già sensibilmente la crescita radicale.",
          "Il compattamento profondo può durare anni: è molto più facile prevenirlo che correggerlo.",
        ],
      },
      {
        titolo: "Profondità e sostanza organica",
        tipo: "scienza",
        punti: [
          "L'aratura profonda porta in superficie suolo meno fertile e accelera la mineralizzazione della sostanza organica: si perde carbonio, struttura e capacità di trattenere l'acqua.",
          "Per cereali ed erbai 20–30 cm bastano: le radici esplorano gli strati sottostanti se il suolo non è compattato.",
          "Variare la profondità tra un anno e l'altro evita la suola; se presente, una ripuntatura a 40–50 cm ogni 3–4 anni (in suolo asciutto, perché si fessuri) la rompe senza rovesciare gli strati.",
        ],
      },
      {
        titolo: "Versanti, erosione e regole PAC",
        tipo: "attenzione",
        punti: [
          "Su pendii lavorare a girapoggio (lungo le curve di livello), non a rittochino: i solchi trasversali rallentano il ruscellamento.",
          "Non arare se nelle 48–72 ore sono previsti più di 20–30 mm: il suolo appena lavorato e nudo è il più esposto all'erosione.",
          "La formazione del suolo è dell'ordine di 1 t/ha l'anno: un singolo evento erosivo può asportarne molte di più.",
          "Condizionalità PAC: BCAA 5 (lavorazioni che tengano conto della pendenza) e BCAA 6 (copertura minima del suolo nei periodi sensibili). Verificare le regole regionali vigenti.",
        ],
      },
    ],
  },
  {
    id: "lavorazioni-conservative",
    nome: "Lavorazioni conservative e semina su sodo",
    sezione: "suolo",
    sintesi: "Minima lavorazione, ripuntatura, strip-till e semina diretta: meno passaggi, più sostanza organica e semine più tempestive.",
    dati: [
      ["Minima lavorazione", "10–15 cm con dischi o coltivatore"],
      ["Ripuntatura", "40–50 cm, suolo asciutto, ogni 3–4 anni"],
      ["Semina su sodo", "seminatrice da sodo, nessuna lavorazione"],
      ["Risparmio di gasolio", "fino al 50–70 % rispetto all'aratura"],
    ],
    blocchi: [
      {
        titolo: "Vantaggi nel contesto sardo",
        punti: [
          "Tra le prime piogge autunnali e la semina il tempo è poco: con la semina su sodo si semina appena il suolo è umido, senza attendere la tempera per arare.",
          "I residui in superficie riducono l'evaporazione e proteggono dall'impatto della pioggia, limitando crosta ed erosione.",
          "Nel medio periodo (3–5 anni) aumentano sostanza organica superficiale, attività dei lombrichi e stabilità degli aggregati.",
          "Trasemina dei pascoli: con seminatrici da sodo si introducono trifoglio sotterraneo e loietto rigido in pascoli degradati senza rompere il cotico.",
        ],
      },
      {
        titolo: "Condizioni per riuscire",
        tipo: "attenzione",
        punti: [
          "Controllo delle infestanti prima della semina (falsa semina, pascolamento intenso o diserbo autorizzato): senza aratura la banca semi resta in superficie.",
          "Residui ben distribuiti dalla mietitrebbia, altrimenti la seminatrice non deposita correttamente il seme.",
          "Nei primi anni possono servire più azoto e una gestione attenta delle limacce, favorite dai residui.",
          "Su suoli già compattati conviene una ripuntatura iniziale prima di passare al sodo.",
        ],
      },
      {
        titolo: "Perché funziona",
        tipo: "scienza",
        punti: [
          "Senza inversione degli strati la sostanza organica si mineralizza più lentamente e si accumula in superficie, dove migliora infiltrazione e struttura.",
          "I canali lasciati dalle radici morte e dai lombrichi restano continui: l'acqua si infiltra meglio di quanto accada in un suolo arato e poi compattato.",
        ],
      },
    ],
  },
  {
    id: "letto-semina",
    nome: "Letto di semina e falsa semina",
    sezione: "suolo",
    sintesi: "Affinare il terreno in proporzione alla dimensione del seme, interrare fosforo e potassio e ridurre le infestanti prima di seminare.",
    dati: [
      ["Regola della profondità", "2–3 volte il diametro del seme"],
      ["Semi piccoli", "medica, trifogli, loietto, carota: 0,5–1,5 cm"],
      ["Semi grossi", "fava, pisello, fagiolo: 3–6 cm"],
      ["Falsa semina", "preparare 2–3 settimane prima"],
    ],
    blocchi: [
      {
        titolo: "Affinamento giusto",
        punti: [
          "Semi piccoli: letto fine e assestato, poi rullatura dopo la semina per garantire il contatto con il suolo e la risalita capillare dell'acqua.",
          "Semi grossi: tollerano zolle di qualche centimetro, che anzi riducono la formazione di crosta.",
          "Evitare di polverizzare i suoli limosi: con la prima pioggia battente formano una crosta che impedisce l'emergenza.",
        ],
      },
      {
        titolo: "Falsa semina contro le infestanti",
        punti: [
          "Preparare il letto 2–3 settimane prima della semina, lasciar emergere le infestanti dopo una pioggia o un'irrigazione, poi eliminarle con una lavorazione superficiale (meno di 5 cm) o con il pirodiserbo.",
          "Lavorare più in profondità riporterebbe in superficie nuovi semi: l'obiettivo è esaurire lo strato superficiale della banca semi.",
          "Particolarmente utile per orticole a semina diretta (carota, spinacio) e per gli erbai in agricoltura biologica.",
        ],
      },
      {
        titolo: "Concimazione di fondo",
        tipo: "scienza",
        punti: [
          "Fosforo e potassio si muovono poco nel suolo: vanno distribuiti e interrati alla preparazione, vicino a dove cresceranno le radici.",
          "L'azoto invece è mobile (nitrato) e va dato quando la coltura lo assorbe: alla semina basta una piccola quota, il resto in copertura.",
          "Nei suoli calcarei sardi il fosforo si lega al calcio e diventa poco disponibile: la localizzazione alla semina ne aumenta l'efficienza.",
        ],
      },
    ],
  },

  // ============================================================ CEREALI
  {
    id: "avena",
    nome: "Avena",
    latino: "Avena sativa L.",
    famiglia: "Poaceae",
    sezione: "cereali",
    sintesi: "Rustica e adattabile a suoli poveri e sub-acidi, ottima da erbaio, da fieno e in consociazione con la veccia. Meno resistente al freddo dell'orzo.",
    tSuolo: { min: 3, opt: "15–20" },
    gelo: -8,
    calendario: { semina: [10, 11, 12], utilizzo: [12, 1, 2, 3, 4, 5], raccolta: [6, 7] },
    kc: [0.3, 1.15, 0.25],
    dati: [
      ["Semina", "ottobre – dicembre"],
      ["Densità", "350–450 semi germinabili/m²"],
      ["Dose di seme", "120–160 kg/ha granella; 140–180 kg/ha erbaio"],
      ["Profondità", "3–4 cm"],
      ["Azoto totale", "60–90 kg N/ha"],
      ["P₂O₅ in presemina", "50–70 kg/ha"],
      ["pH ottimale", "5,5–7 (tollera l'acidità)"],
    ],
    blocchi: [
      {
        titolo: "Suolo e semina",
        punti: [
          "Si adatta a suoli poveri, sciolti o sub-acidi meglio degli altri cereali; meno indicata su suoli calcarei o salini, dove l'orzo rende di più.",
          "Nelle zone interne e montane preferire varietà resistenti al freddo e semine di ottobre, perché le piante accestite resistano meglio alle gelate.",
          "Semina a file distanti 15–18 cm, seguita da rullatura su suoli soffici.",
        ],
      },
      {
        titolo: "Nutrizione",
        punti: [
          "Fabbisogno indicativo: 25–30 kg di azoto per tonnellata di granella prodotta (paglia compresa).",
          "Sui suoli poveri dare il 20–30 % dell'azoto alla semina, il resto tra fine accestimento e inizio levata.",
          "L'avena è molto sensibile all'allettamento: un eccesso di azoto riduce la resa e, negli erbai, aumenta il rischio di nitrati nel foraggio.",
        ],
      },
      {
        titolo: "Utilizzo come erbaio",
        punti: [
          "Pascolamento da accestimento (20–25 cm di altezza), lasciando 5–7 cm per il ricaccio.",
          "Per il doppio uso (pascolo + granella) sospendere il pascolo entro l'inizio della levata.",
          "Fieno: tra inizio spigatura e fioritura, il miglior compromesso tra resa e digeribilità.",
          "Insilato o fasciato: maturazione lattea–cerosa, con il 30–35 % di sostanza secca.",
          "In consociazione con la veccia (80–100 + 60–80 kg/ha) il foraggio raggiunge un tenore proteico nettamente superiore all'avena in purezza.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Ruggine coronata (Puccinia coronata): favorita da autunni e inverni miti e umidi. Scegliere varietà resistenti.",
          "Virus del nanismo giallo (BYDV), trasmesso dagli afidi: il rischio cresce con semine precoci e autunni caldi.",
          "Mal del piede e carbone: rotazione e seme conciato.",
        ],
      },
      {
        titolo: "Perché la levata è decisiva",
        tipo: "scienza",
        punti: [
          "Fino alla fine dell'accestimento l'apice vegetativo resta vicino al suolo, protetto dal morso degli animali e dalla barra falciante.",
          "Con la levata l'apice (la futura spiga) sale: superato circa 1 cm dal suolo, un pascolamento basso lo asporta e il culmo non produce più spiga né ricaccio.",
        ],
      },
    ],
  },
  {
    id: "orzo",
    nome: "Orzo",
    latino: "Hordeum vulgare L.",
    famiglia: "Poaceae",
    sezione: "cereali",
    sintesi: "Il cereale più precoce e tollerante a siccità e salinità: completa il ciclo prima del caldo estivo. Teme acidità e ristagni.",
    tSuolo: { min: 3, opt: "15–20" },
    gelo: -10,
    calendario: { semina: [10, 11, 12], utilizzo: [12, 1, 2, 3, 4], raccolta: [5, 6] },
    kc: [0.3, 1.15, 0.25],
    dati: [
      ["Semina", "novembre – dicembre (erbaio da ottobre)"],
      ["Densità", "300–400 semi germinabili/m²"],
      ["Dose di seme", "150–180 kg/ha"],
      ["Profondità", "3–4 cm"],
      ["Azoto totale", "60–100 kg N/ha (malto: moderato)"],
      ["Tolleranza salinità", "soglia ECe 8 dS/m"],
      ["Raccolta granella", "maggio – giugno, umidità ≤ 13 %"],
    ],
    blocchi: [
      {
        titolo: "Suolo e semina",
        punti: [
          "Ideale su suoli neutri o alcalini, anche calcarei e moderatamente salini; soffre su suoli acidi (pH < 6) e con ristagno idrico durante l'accestimento.",
          "Gli orzi polistici (a sei file) accestiscono meno dei distici: per loro usare il valore alto della densità.",
          "Grazie al ciclo breve «sfugge» alla siccità di fine primavera: è il cereale più sicuro nelle aree con meno di 400–450 mm di pioggia.",
        ],
      },
      {
        titolo: "Nutrizione",
        punti: [
          "Frazionare l'azoto: circa 1/3 a fine accestimento e 2/3 a inizio levata.",
          "Per l'orzo da malto evitare eccessi: la proteina della granella deve restare tra il 9,5 e l'11,5 %.",
          "Il fosforo va interrato in presemina (50–70 kg P₂O₅/ha sui suoli poveri).",
        ],
      },
      {
        titolo: "Utilizzo come erbaio",
        punti: [
          "Il più precoce tra gli erbai di graminacee: prima utilizzazione 50–70 giorni dall'emergenza, buon ricaccio.",
          "Insilato a maturazione cerosa; in miscuglio con pisello proteico (100–120 + 80–100 kg/ha) migliora il valore proteico.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Elmintosporiosi (Pyrenophora teres) e rincosporiosi (Rhynchosporium): favorite da piogge frequenti e temperature miti in primavera.",
          "Oidio con primavere miti; BYDV con semine precoci.",
          "Gelate tardive (≤ −2 °C) in spigatura causano sterilità delle spighette.",
        ],
      },
      {
        titolo: "Tolleranza alla salinità",
        tipo: "scienza",
        punti: [
          "Secondo FAO (Ayers e Westcot) l'orzo non perde resa fino a una conducibilità dell'estratto saturo (ECe) di 8 dS/m e perde circa il 5 % per ogni dS/m oltre la soglia: è il cereale più adatto ai suoli costieri salinizzati.",
        ],
      },
    ],
  },

  // ============================================================ FORAGGERE
  {
    id: "trifoglio-sotterraneo",
    nome: "Trifoglio sotterraneo",
    latino: "Trifolium subterraneum L.",
    famiglia: "Fabaceae",
    sezione: "foraggere",
    sintesi: "Leguminosa annuale autoriseminante: interra i propri semi e ricostituisce il pascolo ogni autunno. È la base dei pascoli migliorati della Sardegna.",
    tSuolo: { min: 4, opt: "15–20" },
    gelo: -6,
    calendario: { semina: [9, 10, 11], utilizzo: [11, 12, 1, 2, 3, 4, 5] },
    kc: [0.4, 0.9, 0.85],
    dati: [
      ["Semina", "settembre – novembre, con le prime piogge"],
      ["Dose di seme", "20–30 kg/ha in purezza; 10–15 in miscuglio"],
      ["Profondità", "1–2 cm"],
      ["P₂O₅ all'impianto", "60–100 kg/ha, poi 30–50 ogni 1–2 anni"],
      ["Azoto", "nessuno (fissazione simbiotica)"],
      ["Azoto fissato", "50–150 kg N/ha l'anno"],
    ],
    blocchi: [
      {
        titolo: "Scelta delle sottospecie e delle varietà",
        punti: [
          "subsp. subterraneum: suoli sub-acidi o neutri (pH 5,5–7), i più diffusi nelle aree granitiche e scistose.",
          "subsp. brachycalycinum: suoli neutri, alcalini o argillosi, dove interra meglio i glomeruli.",
          "subsp. yanninicum: suoli soggetti a ristagno temporaneo.",
          "Precocità calibrata sulla durata della stagione piovosa: varietà precoci dove piove meno di 450 mm, medio-tardive oltre 600 mm.",
          "Preferire cultivar a basso contenuto di formononetina (fitoestrogeno).",
        ],
      },
      {
        titolo: "Impianto",
        punti: [
          "Dove il trifoglio non è mai stato coltivato, usare seme inoculato con Rhizobium leguminosarum bv. trifolii (seme confettato).",
          "Semina superficiale e rullatura; su pascoli esistenti trasemina con seminatrice da sodo dopo un pascolamento intenso che riduca la competizione.",
          "In consociazione con loietto rigido (10–15 kg/ha) si ottiene un pascolo più equilibrato e meno meteorizzante.",
        ],
      },
      {
        titolo: "Gestione del pascolo",
        punti: [
          "Primo anno: alleggerire il carico tra fioritura e interramento dei glomeruli (primavera) per costituire una buona banca semi.",
          "In estate pascolare il residuo secco con carichi moderati: riduce la paglia che ostacola la germinazione autunnale, ma un carico eccessivo asporta anche i glomeruli.",
          "Una concimazione fosfatica regolare mantiene la componente leguminosa; senza fosforo il pascolo si impoverisce in pochi anni.",
        ],
      },
      {
        titolo: "Fitoestrogeni e fertilità",
        tipo: "attenzione",
        punti: [
          "Le vecchie cultivar ricche di formononetina possono ridurre la fertilità delle pecore («malattia del trifoglio»).",
          "Usare cultivar moderne a basso contenuto ed evitare il pascolo su trifoglio in purezza durante la monta.",
        ],
      },
      {
        titolo: "Perché è adatto al clima mediterraneo",
        tipo: "scienza",
        punti: [
          "Geocarpia: dopo la fecondazione i peduncoli si piegano e interrano i glomeruli, proteggendo il seme dal pascolo e dal caldo.",
          "Una parte dei semi è «dura» (tegumento impermeabile) e germina negli anni successivi: una riserva contro le annate siccitose.",
          "Il ciclo autunno–primavera evita l'estate: la pianta sopravvive alla siccità come seme.",
        ],
      },
    ],
  },
  {
    id: "trifoglio-alessandrino",
    nome: "Trifoglio alessandrino e incarnato",
    latino: "Trifolium alexandrinum L. · T. incarnatum L.",
    famiglia: "Fabaceae",
    sezione: "foraggere",
    sintesi: "Trifogli annuali da erbaio. L'alessandrino dà più tagli ed è poco meteorizzante; l'incarnato fa un solo taglio ma resiste meglio al freddo e ai suoli sub-acidi.",
    tSuolo: { min: 5, opt: "15–22" },
    gelo: -5,
    calendario: { semina: [9, 10, 2, 3], utilizzo: [12, 1, 2, 3, 4, 5] },
    kc: [0.4, 0.9, 0.85],
    dati: [
      ["Semina", "settembre – ottobre; irriguo anche febbraio – marzo"],
      ["Dose alessandrino", "25–35 kg/ha in purezza"],
      ["Dose incarnato", "20–30 kg/ha in purezza"],
      ["Profondità", "1–1,5 cm"],
      ["Tagli alessandrino", "3–5 in irriguo, 2 in asciutto"],
      ["Altezza di taglio", "7–10 cm (alessandrino)"],
    ],
    blocchi: [
      {
        titolo: "Trifoglio alessandrino (bersim)",
        punti: [
          "Non autoriseminante; ricaccia dalle gemme basali dello stelo: tagli troppo bassi riducono la ripresa, per questo si taglia a 7–10 cm.",
          "Tollera suoli alcalini e una salinità moderata; soffre sotto −4/−6 °C: preferirlo nelle aree costiere e di pianura.",
          "Classico in miscuglio con loietto italico (15–20 + 15–20 kg/ha) per pascolo e più sfalci.",
          "Rispetto agli altri trifogli è considerato poco meteorizzante, ma l'introduzione al pascolo deve restare graduale.",
        ],
      },
      {
        titolo: "Trifoglio incarnato",
        punti: [
          "Un solo taglio ad aprile, a inizio fioritura; più resistente al freddo e adatto ai suoli sub-acidi delle zone interne.",
          "Ottimo con avena o loietto, e come sovescio prima di una coltura primaverile.",
        ],
      },
      {
        titolo: "Perché seminarli in miscuglio",
        tipo: "scienza",
        punti: [
          "La graminacea sfrutta l'azoto rilasciato dalla leguminosa e ne sostiene i fusti; la leguminosa alza il tenore proteico del foraggio.",
          "Una quota di graminacee ≥ 50 % riduce il rischio di meteorismo e migliora l'essiccazione in campo durante la fienagione.",
        ],
      },
    ],
  },
  {
    id: "erba-medica",
    nome: "Erba medica",
    latino: "Medicago sativa L.",
    famiglia: "Fabaceae",
    sezione: "foraggere",
    sintesi: "La «regina delle foraggere»: poliennale, proteica, con radice fittonante profonda. Vuole suoli profondi, drenati e non acidi.",
    tSuolo: { min: 8, opt: "18–25" },
    gelo: -10,
    calendario: { semina: [9, 10, 2, 3], utilizzo: [4, 5, 6, 7, 8, 9, 10] },
    kc: [0.4, 0.95, 0.9],
    dati: [
      ["Semina", "settembre – inizio ottobre o febbraio – marzo"],
      ["Dose di seme", "25–35 kg/ha"],
      ["Profondità", "0,5–1,5 cm, letto fine e rullato"],
      ["pH ottimale", "6,5–8"],
      ["Concimazione d'impianto", "P₂O₅ 100–150, K₂O 100–200 kg/ha"],
      ["Tagli", "2–4 in asciutto, 6–8 in irriguo"],
      ["Durata", "3–5 anni"],
    ],
    blocchi: [
      {
        titolo: "Suolo e impianto",
        punti: [
          "Suoli profondi e drenati; il ristagno favorisce i marciumi radicali (Phytophthora medicaginis) e l'acidità inibisce il suo simbionte Sinorhizobium meliloti.",
          "La semina di fine estate deve avvenire almeno 6–8 settimane prima delle gelate, perché le piantine arrivino all'inverno con 3–4 foglie trifogliate.",
          "Inoculare il seme dove la medica non è mai stata coltivata; azoto non necessario (al massimo 20–30 kg/ha come starter).",
          "Preferire varietà a basso riposo invernale (classi 7–9) selezionate per gli ambienti mediterranei.",
        ],
      },
      {
        titolo: "Gestione dei tagli",
        punti: [
          "Tagliare a inizio fioritura (10 % dei fusti fioriti): miglior compromesso tra resa, proteina e persistenza.",
          "Almeno una volta l'anno, a fine estate, lasciar arrivare la coltura a piena fioritura per ricostituire le riserve della radice.",
          "Altezza di taglio 5–7 cm; in estate non scendere sotto i 28–35 giorni tra un taglio e l'altro.",
          "Pascolamento ovino solo a rotazione, mai continuo: il calpestio e il morso ripetuto danneggiano il colletto.",
        ],
      },
      {
        titolo: "Fienagione senza perdere le foglie",
        punti: [
          "Le foglie contengono gran parte della proteina e si staccano facilmente quando sono secche.",
          "Rivoltare e ranghinare con umidità del foraggio superiore al 40 %, meglio al mattino con un po' di rugiada.",
          "Imballare al 18–20 % di umidità (le balle grandi ancora più secche): oltre il 25 % si rischiano muffe e autocombustione.",
          "Servono 3–4 giorni senza pioggia: la sezione «Oggi in campo» indica le finestre di fienagione previste.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Fitonomo (Hypera postica): in caso di forti attacchi primaverili anticipare il primo taglio.",
          "Cuscuta (pianta parassita): usare solo seme certificato.",
          "Autotossicità: non riseminare medica subito dopo medica; attendere almeno 1–2 anni con un'altra coltura.",
          "Meteorismo negli animali al pascolo su medica giovane o bagnata.",
        ],
      },
      {
        titolo: "Resa contro qualità",
        tipo: "scienza",
        punti: [
          "Dal bottone fiorale alla piena fioritura la resa per taglio aumenta, ma la proteina grezza scende indicativamente da circa il 22 % al 16 % della sostanza secca e la fibra lignificata cresce.",
          "La radice fittonante accumula carboidrati e proteine di riserva che alimentano il ricaccio: tagli troppo frequenti la esauriscono e riducono la durata del medicaio.",
        ],
      },
    ],
  },
  {
    id: "loietto",
    nome: "Loietto (italico e rigido)",
    latino: "Lolium multiflorum Lam. · L. rigidum Gaudin",
    famiglia: "Poaceae",
    sezione: "foraggere",
    sintesi: "Graminacea da erbaio e da pascolo: l'italico per la massima produzione con acqua e azoto, il rigido (autoctono) per i pascoli asciutti e autoriseminanti.",
    tSuolo: { min: 4, opt: "15–20" },
    gelo: -10,
    calendario: { semina: [9, 10, 11], utilizzo: [11, 12, 1, 2, 3, 4, 5] },
    kc: [0.95, 1.05, 1.0],
    dati: [
      ["Semina", "settembre – novembre"],
      ["Dose italico", "30–40 kg/ha in purezza; 15–20 in miscuglio"],
      ["Dose rigido", "15–25 kg/ha in purezza; 10–15 in miscuglio"],
      ["Profondità", "1–2 cm"],
      ["Azoto", "40–60 kg N/ha per utilizzazione"],
      ["Produzione italico", "8–12 t SS/ha in irriguo"],
    ],
    blocchi: [
      {
        titolo: "Quale loietto",
        punti: [
          "Italico: annuale o biennale, molto produttivo, esigente in acqua e azoto. Le varietà tetraploidi sono più appetibili e zuccherine, le diploidi più resistenti al calpestio.",
          "Rigido: specie spontanea mediterranea, resistente alla siccità e autoriseminante, adatta ai pascoli migliorati in consociazione con il trifoglio sotterraneo.",
        ],
      },
      {
        titolo: "Pascolamento: la regola delle tre foglie",
        punti: [
          "Entrare quando i culmi hanno 2,5–3 foglie nuove (15–20 cm): la pianta ha ricostituito le riserve e la crescita è massima.",
          "Oltre le 3 foglie la più vecchia muore: si spreca foraggio e la qualità scende.",
          "Uscire lasciando 5–7 cm di residuo, dove si trovano le riserve per il ricaccio.",
          "Turni di riposo indicativi: 25–40 giorni in autunno–inverno, 15–25 in primavera.",
        ],
      },
      {
        titolo: "Concimazione e nitrati",
        tipo: "attenzione",
        punti: [
          "Distribuire l'azoto dopo ogni utilizzazione (40–60 kg N/ha), non in un'unica dose.",
          "Con molto azoto disponibile e giornate fredde e nuvolose la pianta accumula nitrati: possono diventare tossici per i ruminanti.",
          "Il loietto rigido è anche un'infestante dei cereali con diffusi casi di resistenza agli erbicidi: evitare che vada a seme nei campi destinati al frumento.",
        ],
      },
      {
        titolo: "Conservazione",
        punti: [
          "Fieno o fasciato a inizio spigatura.",
          "L'elevato contenuto di zuccheri solubili lo rende ottimo per il fasciato: fermenta bene e scende rapidamente di pH.",
        ],
      },
      {
        titolo: "Fisiologia del ricaccio",
        tipo: "scienza",
        punti: [
          "Dopo la defogliazione la pianta ricresce a spese dei fruttani accumulati alla base dei culmi; le riserve si ricostituiscono quando si forma la terza foglia (Fulkerson e Donaghy, 2001).",
          "Utilizzare prima esaurisce le riserve e indebolisce il cotico; utilizzare dopo comporta senescenza e perdita di qualità.",
        ],
      },
    ],
  },

  // ============================================================ OVINI
  {
    id: "calendario-foraggero",
    nome: "Calendario foraggero della pecora Sarda",
    sezione: "ovini",
    sintesi: "Come far coincidere la produzione di erba con i fabbisogni del gregge lungo l'anno.",
    dati: [
      ["Peso pecora adulta", "38–45 kg"],
      ["Latte per lattazione", "≈ 180–250 kg (anche oltre 300 nelle linee selezionate)"],
      ["Grasso / proteine del latte", "≈ 6,5 % / 5,5 %"],
      ["Parti principali", "ottobre – dicembre (adulte)"],
      ["Picco dei fabbisogni", "fine gestazione e primi 2 mesi di lattazione"],
    ],
    blocchi: [
      {
        titolo: "Le quattro stagioni del foraggio",
        punti: [
          "Autunno (ott–dic): pascolo naturale scarso proprio quando i fabbisogni salgono. Erbai precoci di loietto, orzo o avena seminati a settembre–ottobre, più fieno e concentrati.",
          "Inverno (gen–feb): crescita lenta (indicativamente 10–20 kg SS/ha al giorno). Pascolamento razionato degli erbai e dei pascoli migliorati.",
          "Primavera (mar–mag): crescita esplosiva che supera il consumo. Pascolare le superfici migliori e destinare le eccedenze a fieno e fasciato.",
          "Estate (giu–set): pascolo secco e stoppie, medica irrigua dove disponibile, fieno, concentrati. Tosatura a maggio–giugno.",
        ],
      },
      {
        titolo: "Pianificare le superfici",
        punti: [
          "Stimare le superfici da destinare a erbaio dal deficit autunnale: (capi × ingestione giornaliera × giorni da coprire) ÷ (produzione utile dell'erbaio per ettaro).",
          "Esempio: 300 pecore × 1,0 kg SS/giorno da erbaio × 60 giorni = 18 t SS; con un erbaio che rende 3 t SS/ha utilizzabili servono circa 6 ha.",
          "Scaglionare le semine (inizio settembre in irriguo, poi con le prime piogge) per avere erbai pronti in tempi diversi.",
        ],
      },
      {
        titolo: "Perché gli erbai precoci",
        tipo: "scienza",
        punti: [
          "La crescita delle graminacee rallenta fortemente sotto i 5–6 °C di temperatura media: un erbaio che arriva all'inverno già accestito produce molto di più di uno seminato tardi.",
          "La semina «in asciutto» (prima delle piogge) anticipa l'emergenza, ma con piogge scarse e intermittenti rischia una falsa partenza: conviene se la previsione indica almeno 20–25 mm.",
        ],
      },
    ],
  },
  {
    id: "miscugli-erbai",
    nome: "Erbai autunno-vernini: miscugli consigliati",
    sezione: "ovini",
    sintesi: "Graminacea + leguminosa: più proteina, meno azoto da concimazione, meno meteorismo rispetto alla leguminosa pura.",
    tabella: {
      titolo: "Miscugli per la Sardegna (dosi indicative di seme, kg/ha)",
      intestazioni: ["Miscuglio", "Dose", "Utilizzo", "Ambiente"],
      righe: [
        ["Loietto italico + trifoglio alessandrino", "15–20 + 15–20", "pascolo e 2–4 sfalci", "irriguo o > 500 mm, pianura e collina"],
        ["Avena + veccia comune", "80–100 + 60–80", "fieno o fasciato (1 taglio), pascolo precoce", "asciutto, suoli medi e sub-acidi"],
        ["Orzo + pisello proteico", "100–120 + 80–100", "insilato o fieno, molto precoce", "asciutto, suoli calcarei e salini"],
        ["Triticale + veccia villosa", "100–120 + 30–40", "fieno o fasciato", "zone interne fredde, suoli acidi"],
        ["Loietto rigido + trifoglio sotterraneo", "10–15 + 15–20", "pascolo permanente autoriseminante", "asciutto, < 600 mm, colline e altopiani"],
      ],
    },
    blocchi: [
      {
        titolo: "Semina",
        punti: [
          "Seminare le due specie in passaggi separati se i semi hanno dimensioni molto diverse, oppure regolare la seminatrice sul seme più piccolo e a profondità intermedia (2–3 cm).",
          "Concimazione: P₂O₅ 60–80 kg/ha in presemina; azoto ridotto (30–50 kg/ha) perché una quota arriva dalla leguminosa.",
          "Con la consociazione la leguminosa dovrebbe rappresentare il 30–50 % della biomassa: un eccesso di azoto favorisce la graminacea e la soffoca.",
        ],
      },
      {
        titolo: "Valore nutritivo indicativo",
        tipo: "scienza",
        punti: [
          "Loietto pascolato in fase vegetativa: circa 0,85–0,95 UFL/kg SS, proteina grezza 15–20 %.",
          "Erbaio con buona componente leguminosa: proteina grezza 18–24 %.",
          "Fieno di avena–veccia tagliato a fioritura: circa 0,6–0,7 UFL/kg SS, proteina 10–14 %; il ritardo nel taglio abbassa entrambi i valori.",
        ],
      },
    ],
  },
  {
    id: "pascolamento",
    nome: "Pascolamento razionato degli erbai",
    sezione: "ovini",
    sintesi: "Recinti mobili, accessi a tempo e altezze di ingresso e uscita: utilizzare tutto l'erbaio senza sprechi e senza indebolirlo.",
    dati: [
      ["Ingresso", "15–20 cm (graminacee a 3 foglie)"],
      ["Uscita", "5–7 cm di residuo"],
      ["Riposo autunno–inverno", "25–40 giorni"],
      ["Riposo primavera", "15–25 giorni"],
      ["Pascolo a tempo", "2–6 ore/giorno sull'erbaio"],
    ],
    blocchi: [
      {
        titolo: "Organizzazione",
        punti: [
          "Dividere l'erbaio con filo elettrico in settori da consumare in 1–3 giorni: gli animali scelgono meno, calpestano meno e il ricaccio è uniforme.",
          "Il pascolo a tempo limitato (poche ore al giorno sull'erbaio, il resto su pascolo naturale o con fieno) mantiene alta la produzione di latte e riduce sprechi e meteorismo.",
          "Fare entrare il gregge con il rumine già parzialmente pieno (fieno al mattino) e mai su erba bagnata di rugiada o brina se c'è una leguminosa abbondante.",
        ],
      },
      {
        titolo: "Calcolo del carico",
        punti: [
          "Capi × giorni = (biomassa disponibile × efficienza di utilizzazione) ÷ ingestione giornaliera.",
          "Esempio: erbaio con 1,5 t SS/ha, utilizzazione 60 % → 900 kg SS; con 2,2 kg SS/capo al giorno si ottengono circa 400 capi × giorno per ettaro (es. 200 pecore per 2 giorni).",
          "Stima rapida della biomassa: in un erbaio fitto ogni centimetro di altezza sopra i 5 cm vale indicativamente 100–200 kg SS/ha; tarare la stima con pesate di prova.",
        ],
      },
      {
        titolo: "Proteggere il ricaccio",
        tipo: "scienza",
        punti: [
          "Prima della levata l'apice vegetativo delle graminacee è basso: il pascolamento non lo raggiunge e la pianta ricaccia.",
          "Dopo l'inizio della levata un pascolamento raso elimina gli apici: se si vuole un'altra utilizzazione, uscire prima o alzare l'altezza di uscita.",
        ],
      },
    ],
  },
  {
    id: "fieno-insilato",
    nome: "Fieno, fasciato e insilato",
    sezione: "ovini",
    sintesi: "Conservare l'eccesso primaverile con il minimo di perdite: momento di taglio, umidità alla raccolta, tenuta dell'aria.",
    tabella: {
      titolo: "Umidità e stadio di raccolta",
      intestazioni: ["Prodotto", "Stadio di taglio", "Umidità alla raccolta", "Note"],
      righe: [
        ["Fieno", "graminacee inizio spigatura; leguminose inizio fioritura", "≤ 18–20 % (balle grandi ≤ 15–18 %)", "3–4 giorni asciutti; oltre il 25 % rischio muffe e autocombustione"],
        ["Fasciato (fieno-silo)", "come il fieno, anche leggermente prima", "45–60 % (40–55 % di SS)", "fasciare entro 2 ore dalla pressatura, almeno 6 strati di film"],
        ["Insilato di cereali", "maturazione lattea–cerosa", "65–70 % (30–35 % di SS)", "trinciatura 2–3 cm, compattazione e chiusura rapide"],
      ],
    },
    blocchi: [
      {
        titolo: "Buone pratiche",
        punti: [
          "Falciare dopo che la rugiada è evaporata e usare un condizionatore: lo schiacciamento degli steli accelera l'essiccazione delle leguminose.",
          "Rivoltare e ranghinare quando il foraggio ha ancora più del 40 % di umidità, per non perdere le foglie.",
          "Le balle fasciate vanno stoccate su superfici pulite, lontano da roditori e uccelli che bucano il film: ogni foro fa entrare aria e muffe.",
        ],
      },
      {
        titolo: "Perché l'aria è il nemico",
        tipo: "scienza",
        punti: [
          "Nell'insilato i batteri lattici trasformano gli zuccheri in acido lattico e abbassano il pH (circa 4–4,5 negli insilati, un po' più alto nei fasciati), conservando il foraggio.",
          "L'ossigeno residuo consente invece a lieviti e muffe di consumare zuccheri e proteine, scaldando la massa e producendo micotossine.",
          "Le leguminose hanno pochi zuccheri e un alto potere tampone: fermentano peggio delle graminacee, perciò si appassiscono di più o si consociano.",
        ],
      },
    ],
  },
  {
    id: "rischi-sanitari",
    nome: "Rischi sanitari legati al foraggio",
    sezione: "ovini",
    sintesi: "Meteorismo, nitrati, enterotossiemia, fitoestrogeni e stress da caldo: riconoscerli e prevenirli con la gestione del pascolo.",
    blocchi: [
      {
        titolo: "Meteorismo spumoso",
        tipo: "attenzione",
        punti: [
          "Causa: leguminose giovani (medica, trifogli) ricche di proteine solubili e saponine formano una schiuma stabile nel rumine che impedisce l'eruttazione.",
          "Prevenzione: fieno prima del pascolo, introduzione graduale, almeno il 50 % di graminacee nel miscuglio, niente erba bagnata di rugiada o brina, prodotti antischiumogeni nei periodi a rischio.",
          "Il trifoglio alessandrino e le leguminose ricche di tannini condensati (sulla, ginestrino) sono meno meteorizzanti.",
        ],
      },
      {
        titolo: "Nitrati",
        tipo: "attenzione",
        punti: [
          "Si accumulano in avena, orzo, loietto e altre graminacee concimate con molto azoto, dopo giorni freddi e nuvolosi o quando la pioggia arriva dopo una siccità.",
          "Contenuti superiori a circa l'1 % di nitrato nella sostanza secca sono pericolosi; tra 0,5 e 1 % serve cautela, soprattutto con animali affamati.",
          "Prevenzione: frazionare l'azoto, far pascolare nel pomeriggio delle giornate soleggiate (con la luce la pianta riduce i nitrati), introdurre gradualmente l'erbaio.",
        ],
      },
      {
        titolo: "Enterotossiemia e acidosi",
        tipo: "attenzione",
        punti: [
          "Il passaggio brusco a erbai lussureggianti o a molto concentrato favorisce la proliferazione di Clostridium perfringens e l'acidosi ruminale.",
          "Vaccinazione secondo il piano del veterinario e adattamento graduale di 10–15 giorni.",
        ],
      },
      {
        titolo: "Stress da caldo",
        punti: [
          "Con THI superiore a circa 72 calano ingestione e latte; oltre 78 lo stress è severo (vedi indice THI nel cruscotto).",
          "In estate: pascolo nelle ore fresche (alba e sera, anche notturno), ombra disponibile, acqua pulita e fresca a volontà.",
          "Fabbisogno d'acqua indicativo: 3–6 L/capo al giorno in asciutta, 6–10 in lattazione, di più con il caldo.",
        ],
      },
      {
        titolo: "Fitoestrogeni",
        punti: [
          "Formononetina nelle vecchie varietà di trifoglio sotterraneo; cumestrolo nella medica attaccata da funghi fogliari.",
          "Evitare questi foraggi in purezza nel periodo della monta.",
        ],
      },
    ],
  },
  {
    id: "fabbisogni",
    nome: "Fabbisogni nutrizionali indicativi",
    sezione: "ovini",
    sintesi: "Ordini di grandezza per una pecora Sarda di 45 kg, utili per bilanciare razione e superfici foraggere.",
    tabella: {
      titolo: "Fabbisogni giornalieri per capo (valori orientativi)",
      intestazioni: ["Fase", "Energia (UFL/giorno)", "Sostanza secca ingerita", "Note"],
      righe: [
        ["Asciutta, mantenimento", "≈ 0,6–0,7", "1,2–1,4 kg", "pascolo naturale e fieno sufficienti"],
        ["Ultime 6 settimane di gestazione", "≈ 0,9–1,1", "1,3–1,6 kg", "rischio di tossiemia gravidica se la razione è carente"],
        ["Lattazione, 1 kg latte/giorno", "≈ 1,3–1,4", "1,8–2,2 kg", "≈ 0,7 UFL per kg di latte in più del mantenimento"],
        ["Lattazione, 2 kg latte/giorno", "≈ 2,0–2,1", "2,3–2,8 kg", "serve foraggio di alta qualità più concentrati"],
      ],
    },
    blocchi: [
      {
        titolo: "Come usare la tabella",
        tipo: "scienza",
        punti: [
          "Energia per la lattazione ≈ mantenimento + 0,7 UFL per ogni kg di latte di pecora (riferito a un latte con circa 6,5 % di grasso).",
          "La capacità di ingestione (circa 2–3,5 % del peso vivo in sostanza secca) limita quanto foraggio può essere consumato: con foraggi poveri l'energia non basta e serve concentrato.",
          "Valori derivati dai sistemi INRA per gli ovini da latte: per razioni precise rivolgersi a un tecnico alimentarista.",
        ],
      },
    ],
  },

  // ============================================================ ORTICOLE
  {
    id: "principi-orticoltura",
    nome: "Principi comuni per l'orto",
    sezione: "orticole",
    sintesi: "Rotazioni, irrigazione, pacciamatura, solarizzazione e qualità dell'acqua: le regole che valgono per tutte le orticole.",
    blocchi: [
      {
        titolo: "Rotazione per famiglie",
        punti: [
          "Solanacee (pomodoro, peperone, melanzana, patata) · Cucurbitacee (zucchina, melone, anguria) · Brassicacee (cavoli) · Amarillidacee (cipolla, aglio) · Fabacee (fava, pisello, fagiolo) · Apiacee (carota, finocchio) · Asteracee (lattuga, carciofo) · Amarantacee (spinacio).",
          "Non ripetere la stessa famiglia sullo stesso appezzamento prima di 3–4 anni.",
          "Far precedere le colture esigenti (solanacee, cavoli) da una leguminosa: lascia indicativamente 30–60 kg N/ha.",
        ],
      },
      {
        titolo: "Irrigazione",
        punti: [
          "Il fabbisogno giornaliero è circa ETc = ET0 × Kc meno la pioggia efficace: impostare il Kc della coltura nei Parametri e leggere il bilancio in «Previsioni e bilancio».",
          "Volume per intervento ≈ (capacità di campo − soglia di intervento) × profondità delle radici. Esempio: suolo franco, 0,32 − 0,22 = 0,10 m³/m³ × 0,3 m = 30 mm (30 L/m²).",
          "Irrigare al mattino presto: la chioma si asciuga rapidamente e si perde meno acqua per evaporazione.",
        ],
      },
      {
        titolo: "Pacciamatura e solarizzazione",
        punti: [
          "Il film nero riscalda il suolo di circa 2–4 °C e controlla le infestanti; i film biodegradabili si interrano a fine ciclo.",
          "Solarizzazione: a luglio–agosto suolo irrigato coperto con polietilene trasparente sottile (30–50 µm) per 4–6 settimane. In Sardegna la temperatura a 10 cm supera i 45 °C: si riducono funghi del suolo, nematodi e semi di infestanti.",
        ],
      },
      {
        titolo: "Qualità dell'acqua: salinità",
        tipo: "scienza",
        punti: [
          "Nei pozzi costieri l'intrusione marina alza la salinità. Soglie di salinità del suolo (ECe, dS/m) oltre le quali la resa cala, secondo FAO (Ayers e Westcot):",
          "Fagiolo 1,0 · carota 1,0 · cipolla 1,2 · lattuga 1,3 · peperone 1,5 · patata 1,7 · spinacio 2,0 · melone 2,2 · pomodoro 2,5 · broccolo 2,8 · zucchina 4,7.",
          "Con acque saline: irrigazioni più frequenti, volumi di lisciviazione periodici e preferenza per le specie più tolleranti.",
        ],
      },
    ],
  },
  {
    id: "pomodoro",
    nome: "Pomodoro",
    latino: "Solanum lycopersicum L.",
    famiglia: "Solanaceae",
    sezione: "orticole",
    sintesi: "Da mensa e da industria. Vuole caldo costante ma soffre oltre i 32–35 °C in fioritura; l'irrigazione regolare previene il marciume apicale.",
    tSuolo: { min: 12, opt: "20–25" },
    gelo: 0,
    calendario: { semenzaio: [2, 3], trapianto: [4, 5], raccolta: [7, 8, 9, 10] },
    kc: [0.6, 1.15, 0.8],
    dati: [
      ["Trapianto", "aprile – maggio (serra: ago–set e gen–feb)"],
      ["Condizioni al trapianto", "suolo ≥ 12–13 °C, notti > 10 °C"],
      ["Germinazione", "minimo 10 °C, ottimo 25–30 °C"],
      ["Sesto da mensa", "1,0–1,2 × 0,4–0,5 m, tutorato"],
      ["Sesto da industria", "file binate 1,6 × 0,25–0,35 m"],
      ["Fabbisogno idrico", "400–600 mm a ciclo"],
      ["N – P₂O₅ – K₂O", "120–150 · 80–100 · 150–250 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Temperature critiche",
        punti: [
          "Sotto i 10–12 °C notturni l'allegagione è scarsa; oltre i 32–35 °C il polline diventa sterile e i fiori cascano.",
          "Il gelo (0 °C) uccide la pianta: nelle zone interne non trapiantare prima che il rischio di gelate tardive sia passato.",
          "In estate le reti ombreggianti (20–30 %) riducono le scottature dei frutti.",
        ],
      },
      {
        titolo: "Irrigazione e nutrizione",
        punti: [
          "Goccia con fertirrigazione: frazionare azoto e potassio secondo le fasi, con la massima richiesta di potassio dalla maturazione dei primi palchi.",
          "Un leggero deficit idrico in maturazione alza il grado zuccherino (°Brix) ma riduce la resa: nel pomodoro da industria si sospende l'irrigazione 2–3 settimane prima della raccolta.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Peronospora (Phytophthora infestans): con temperature di 10–25 °C e foglie bagnate per molte ore. L'indicatore «periodo di Smith» in «Oggi in campo» segnala le condizioni favorevoli.",
          "Tuta absoluta: monitoraggio con trappole a feromoni, confusione sessuale, rimozione dei residui.",
          "Orobanche (Phelipanche ramosa) nei suoli infestati: rotazioni lunghe, evitare canapa e tabacco come precessione.",
          "Nematodi galligeni nei suoli sabbiosi: solarizzazione, innesto su portinnesti resistenti.",
          "In serra: virus TYLCV trasmesso dalla mosca bianca (Bemisia tabaci), da contrastare con reti antinsetto.",
        ],
      },
      {
        titolo: "Perché il marciume apicale",
        tipo: "scienza",
        punti: [
          "Il calcio si muove nella pianta con il flusso traspirativo dello xilema: le foglie, che traspirano molto, lo attirano più dei frutti.",
          "Con irrigazioni irregolari o con il caldo il frutto in rapida crescita ne riceve troppo poco e i tessuti dell'apice collassano. Più che il calcio nel suolo conta la regolarità dell'acqua.",
        ],
      },
    ],
  },
  {
    id: "peperone",
    nome: "Peperone",
    latino: "Capsicum annuum L.",
    famiglia: "Solanaceae",
    sezione: "orticole",
    sintesi: "Più esigente in calore del pomodoro: si trapianta a suolo ben caldo. In piena estate teme scottature e cascola dei fiori.",
    tSuolo: { min: 15, opt: "22–28" },
    gelo: 0,
    calendario: { semenzaio: [2, 3], trapianto: [5], raccolta: [7, 8, 9, 10] },
    kc: [0.6, 1.05, 0.9],
    dati: [
      ["Trapianto", "maggio"],
      ["Condizioni al trapianto", "suolo ≥ 15 °C, notti > 12–15 °C"],
      ["Germinazione", "minimo 15 °C, ottimo 25–30 °C"],
      ["Sesto", "0,8–1,0 × 0,3–0,4 m (3–4 piante/m²)"],
      ["N – P₂O₅ – K₂O", "150–200 · 80–100 · 200–250 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Sotto i 15 °C la crescita si ferma, oltre i 32 °C cascano fiori e frutticini: in luglio–agosto conviene un ombreggio del 20–30 %.",
          "Tutoraggio leggero per evitare che le piante cariche si spezzino con il vento (maestrale).",
          "Acqua regolare per prevenire il marciume apicale, frequente come nel pomodoro.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Phytophthora capsici (marciume del colletto) in suoli con ristagno: aiuole rialzate e irrigazione a goccia.",
          "Afidi vettori di virus (CMV, PVY) e tripidi vettori di TSWV: reti, eliminazione delle infestanti ospiti, varietà resistenti.",
          "Ragnetto rosso con caldo secco.",
        ],
      },
      {
        titolo: "Perché si trapianta tardi",
        tipo: "scienza",
        punti: [
          "Con il suolo sotto i 15 °C le radici assorbono poco acqua e fosforo: la pianta resta ferma e diventa più sensibile ai marciumi. Un trapianto più tardivo in suolo caldo recupera rapidamente il ritardo.",
        ],
      },
    ],
  },
  {
    id: "melanzana",
    nome: "Melanzana",
    latino: "Solanum melongena L.",
    famiglia: "Solanaceae",
    sezione: "orticole",
    sintesi: "Specie amante del caldo, produttiva da giugno a ottobre. Sensibile al ragnetto rosso e alla verticilliosi.",
    tSuolo: { min: 15, opt: "22–30" },
    gelo: 0,
    calendario: { semenzaio: [2, 3], trapianto: [5], raccolta: [6, 7, 8, 9, 10] },
    kc: [0.6, 1.05, 0.9],
    dati: [
      ["Trapianto", "maggio"],
      ["Germinazione", "minimo 15 °C, ottimo 25–30 °C"],
      ["Sesto", "1,0–1,2 × 0,5–0,6 m"],
      ["Temperatura ottimale", "22–30 °C"],
      ["N – P₂O₅ – K₂O", "150–180 · 80–100 · 200–250 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Raccogliere i frutti ancora lucidi, prima che i semi imbruniscano: la raccolta frequente stimola nuove fioriture.",
          "Fertirrigazione regolare, con aumento del potassio in piena produzione.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Ragnetto rosso (Tetranychus urticae) con estati calde e secche: favorire gli acari predatori, evitare piretroidi che li eliminano.",
          "Dorifora nelle prime fasi.",
          "Verticillium dahliae: rotazione di almeno 4 anni con colture non ospiti, innesto su portinnesti resistenti.",
        ],
      },
    ],
  },
  {
    id: "zucchina",
    nome: "Zucchina",
    latino: "Cucurbita pepo L.",
    famiglia: "Cucurbitaceae",
    sezione: "orticole",
    sintesi: "Ciclo rapido, semine scalari da aprile ad agosto. L'oidio e i virus trasmessi dagli afidi sono i problemi principali.",
    tSuolo: { min: 13, opt: "22–30" },
    gelo: 1,
    calendario: { semina: [4, 5, 6, 7, 8], raccolta: [5, 6, 7, 8, 9, 10] },
    kc: [0.5, 0.95, 0.75],
    dati: [
      ["Semina o trapianto", "aprile – agosto, scalare"],
      ["Germinazione", "minimo 12–15 °C, ottimo 25–30 °C"],
      ["Sesto", "1,0–1,2 × 0,6–0,8 m"],
      ["Prima raccolta", "45–60 giorni dalla semina"],
      ["N – P₂O₅ – K₂O", "100–150 · 60–80 · 150–200 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Raccogliere ogni 2–3 giorni, quando il frutto è lungo 15–20 cm: frutti lasciati crescere rallentano la produzione.",
          "Le api sono indispensabili per l'impollinazione: trattare solo la sera e con prodotti selettivi.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Oidio (Podosphaera xanthii): favorito da giornate calde con notti umide, anche senza pioggia. Varietà tolleranti, zolfo nei momenti a rischio.",
          "Virus (ZYMV, CMV, WMV) trasmessi dagli afidi: pacciamatura riflettente nelle prime fasi, reti, eliminazione delle piante infette.",
        ],
      },
    ],
  },
  {
    id: "melone",
    nome: "Melone",
    latino: "Cucumis melo L.",
    famiglia: "Cucurbitaceae",
    sezione: "orticole",
    sintesi: "Vuole suolo caldo e molto sole. Il contenimento dell'acqua in maturazione concentra gli zuccheri.",
    tSuolo: { min: 15, opt: "25–32" },
    gelo: 1,
    calendario: { semenzaio: [3], trapianto: [4, 5], raccolta: [7, 8, 9] },
    kc: [0.5, 0.85, 0.6],
    dati: [
      ["Trapianto", "aprile – maggio, su pacciamatura nera"],
      ["Germinazione", "minimo 15–16 °C, ottimo 30 °C"],
      ["Sesto", "2,0 × 0,5–0,8 m"],
      ["N – P₂O₅ – K₂O", "100–120 · 80 · 150–200 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Pacciamatura nera e tunnel nelle prime settimane anticipano la produzione.",
          "Ridurre l'irrigazione a inizio maturazione: aumentano gli zuccheri (°Brix) e si evitano spaccature; un eccesso d'acqua dà frutti insipidi.",
          "Maturazione: nei cantalupo il peduncolo si stacca facilmente (distacco o «slip»).",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Fusariosi (Fusarium oxysporum f. sp. melonis): rotazioni lunghe, innesto su portinnesti resistenti.",
          "Oidio e afidi come nella zucchina.",
        ],
      },
    ],
  },
  {
    id: "anguria",
    nome: "Anguria",
    latino: "Citrullus lanatus (Thunb.) Matsum. & Nakai",
    famiglia: "Cucurbitaceae",
    sezione: "orticole",
    sintesi: "Coltura estiva per suoli sciolti e caldi. Nelle varietà senza semi servono piante impollinatrici.",
    tSuolo: { min: 16, opt: "25–32" },
    gelo: 1,
    calendario: { semenzaio: [3], trapianto: [4, 5], raccolta: [7, 8] },
    kc: [0.4, 1.0, 0.75],
    dati: [
      ["Trapianto", "aprile – maggio"],
      ["Germinazione", "minimo 16 °C, ottimo 30 °C"],
      ["Sesto", "2,0–2,5 × 0,8–1,0 m"],
      ["Impollinatrici", "1 ogni 3 piante nelle varietà senza semi"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Suoli sciolti, profondi e caldi; teme il ristagno.",
          "Maturazione: cirro opposto al frutto secco, macchia di appoggio da bianca a giallo crema, suono sordo alla percussione.",
          "Ridurre l'acqua negli ultimi 10–15 giorni per non diluire gli zuccheri.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Fusariosi e Verticillium: innesto su zucca o su ibridi resistenti, ormai la norma nelle aree a coltivazione intensiva.",
          "Scottature dei frutti con fogliame scarso: mantenere una buona copertura vegetale.",
        ],
      },
    ],
  },
  {
    id: "carciofo",
    nome: "Carciofo",
    latino: "Cynara cardunculus L. var. scolymus (L.) Fiori",
    famiglia: "Asteraceae",
    sezione: "orticole",
    sintesi: "Coltura simbolo della Sardegna (Carciofo Spinoso di Sardegna DOP): risveglio estivo con l'irrigazione e raccolta da ottobre ad aprile.",
    tSuolo: { min: 12, opt: "15–22" },
    gelo: -1,
    calendario: { trapianto: [7, 8], raccolta: [10, 11, 12, 1, 2, 3, 4] },
    kc: [0.5, 1.0, 0.95],
    dati: [
      ["Impianto", "luglio – agosto con ovoli o carducci"],
      ["Sesto", "1,2–1,4 × 0,8–1,0 m"],
      ["Durata carciofaia", "3–4 anni"],
      ["Raccolta", "ottobre – aprile"],
      ["N – P₂O₅ – K₂O", "150–250 (frazionato) · 80–100 · 150–250 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Ciclo e forzatura",
        punti: [
          "La pianta va in riposo estivo; l'irrigazione di luglio–agosto la «risveglia» e anticipa la produzione autunnale. Senza irrigazione la produzione inizia in inverno.",
          "Scarducciatura: lasciare 1–3 carducci per pianta per ottenere capolini di buona pezzatura.",
          "Rinnovare la carciofaia ogni 3–4 anni: con l'età calano resa e qualità e aumentano virosi e marciumi.",
        ],
      },
      {
        titolo: "Freddo e difesa",
        tipo: "attenzione",
        punti: [
          "Le piante sopportano qualche grado sotto zero, ma i capolini si danneggiano già intorno a −1/−2 °C (brattee che si spaccano e imbruniscono): raccogliere quelli pronti prima di una gelata prevista.",
          "Afidi, nottue e limacce nella fase autunnale; peronospora (Bremia lactucae) con umidità elevata.",
          "Usare materiale di propagazione sano: virus e verticillium si trasmettono con ovoli e carducci.",
        ],
      },
    ],
  },
  {
    id: "fava",
    nome: "Fava",
    latino: "Vicia faba L.",
    famiglia: "Fabaceae",
    sezione: "orticole",
    sintesi: "Leguminosa autunno-vernina per consumo fresco e da granella, ottima precessione. Il nemico principale in Sardegna è l'orobanche.",
    tSuolo: { min: 4, opt: "15–20" },
    gelo: -3,
    calendario: { semina: [10, 11, 12], raccolta: [3, 4, 5, 6] },
    kc: [0.5, 1.15, 1.1],
    dati: [
      ["Semina", "ottobre – dicembre"],
      ["Germinazione", "minimo 3–4 °C, ottimo 15–20 °C"],
      ["Profondità", "5–6 cm"],
      ["Densità", "15–25 piante/m² (orto: 0,6 × 0,2 m)"],
      ["Azoto", "nessuno (fissazione simbiotica)"],
      ["P₂O₅", "60–80 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Sopporta alcuni gradi sotto zero in fase vegetativa; i fiori si danneggiano già intorno a −2/−3 °C.",
          "Cimatura degli apici a inizio allegagione contro l'afide nero (Aphis fabae), che si concentra sui germogli teneri.",
          "Raccolta da fresco quando i semi sono verdi e teneri; da granella con baccelli neri e secchi.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Orobanche crenata: semine ritardate (dicembre) riducono gli attacchi; rotazioni lunghe; varietà tolleranti.",
          "Botrite, ruggine e antracnosi (Ascochyta) con inverni miti e piovosi.",
        ],
      },
      {
        titolo: "Perché migliora il suolo",
        tipo: "scienza",
        punti: [
          "La simbiosi con Rhizobium leguminosarum fissa l'azoto atmosferico; i residui ne lasciano una parte alla coltura successiva.",
          "La radice fittonante migliora la struttura del suolo: un frumento dopo fava rende di più e richiede meno azoto.",
        ],
      },
    ],
  },
  {
    id: "pisello",
    nome: "Pisello",
    latino: "Pisum sativum L.",
    famiglia: "Fabaceae",
    sezione: "orticole",
    sintesi: "Leguminosa da clima fresco: si semina da ottobre a febbraio e deve fiorire prima dei caldi primaverili.",
    tSuolo: { min: 4, opt: "16–20" },
    gelo: -2,
    calendario: { semina: [10, 11, 12, 1, 2], raccolta: [3, 4, 5, 6] },
    kc: [0.5, 1.15, 1.1],
    dati: [
      ["Semina", "ottobre – febbraio"],
      ["Germinazione", "minimo 4 °C, ottimo 16–20 °C"],
      ["Profondità", "3–5 cm"],
      ["Densità", "80–120 piante/m²"],
      ["P₂O₅", "60–80 kg/ha, azoto non necessario"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Temperature sopra 25–28 °C in fioritura causano aborto dei fiori: evitare semine troppo tardive.",
          "Le varietà rampicanti richiedono sostegni; quelle nane si prestano alla raccolta meccanica.",
          "Raccolta da fresco con semi ben formati ma ancora dolci: il ritardo trasforma rapidamente gli zuccheri in amido.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Oidio (Erysiphe pisi) in primavera; antracnosi (Ascochyta) con piogge frequenti.",
          "Tonchio (Bruchus pisorum) sui semi destinati alla granella.",
          "Orobanche nei suoli infestati, come per la fava.",
        ],
      },
    ],
  },
  {
    id: "fagiolo",
    nome: "Fagiolo e fagiolino",
    latino: "Phaseolus vulgaris L.",
    famiglia: "Fabaceae",
    sezione: "orticole",
    sintesi: "Coltura primaverile-estiva, molto sensibile a freddo, salinità e caldo secco in fioritura.",
    tSuolo: { min: 12, opt: "20–28" },
    gelo: 1,
    calendario: { semina: [4, 5, 6, 7], raccolta: [6, 7, 8, 9, 10] },
    kc: [0.5, 1.05, 0.9],
    dati: [
      ["Semina", "aprile – luglio"],
      ["Germinazione", "minimo 10–12 °C, ottimo 25 °C"],
      ["Profondità", "3–4 cm"],
      ["Sesto nano", "0,5 × 0,05–0,07 m (25–35 piante/m²)"],
      ["N – P₂O₅ – K₂O", "40–60 · 60 · 100 kg/ha"],
      ["Salinità", "la specie più sensibile (soglia ECe 1,0 dS/m)"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Semi in suolo freddo e umido marciscono prima di emergere: attendere che il suolo superi i 12 °C.",
          "Con oltre 30 °C e aria secca i fiori cascano: le semine di luglio vanno irrigate con regolarità.",
          "Fissa poco azoto rispetto ad altre leguminose: una piccola dose alla semina è utile.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Ragnetto rosso e afidi in estate; antracnosi e batteriosi con irrigazione a pioggia: preferire la goccia.",
          "Evitare acque saline: anche livelli modesti riducono molto la resa.",
        ],
      },
    ],
  },
  {
    id: "patata",
    nome: "Patata",
    latino: "Solanum tuberosum L.",
    famiglia: "Solanaceae",
    sezione: "orticole",
    sintesi: "In Sardegna tre cicli: primaticcia (semina dicembre–febbraio), comune (primavera) e bisestile (fine estate). Esigente in potassio, sensibile alla peronospora.",
    tSuolo: { min: 8, opt: "15–20" },
    gelo: -1,
    calendario: { semina: [12, 1, 2, 3, 8, 9], raccolta: [4, 5, 6, 7, 11, 12] },
    kc: [0.5, 1.15, 0.75],
    dati: [
      ["Primaticcia", "semina dic–feb, raccolta apr–giu"],
      ["Bisestile", "semina ago–set, raccolta nov–dic"],
      ["Suolo alla semina", "≥ 7–8 °C a 10 cm"],
      ["Profondità", "8–12 cm"],
      ["Sesto", "0,75 × 0,25–0,30 m (2–2,5 t/ha di tuberi seme)"],
      ["N – P₂O₅ – K₂O", "120–180 · 80–120 · 200–300 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Suoli sciolti, ben drenati, leggermente acidi o neutri; evitare i suoli pesanti che deformano i tuberi.",
          "Rincalzare per proteggere i tuberi dalla luce (inverdimento e solanina) e dalla tignola.",
          "Irrigazione regolare dalla tuberificazione: stress seguiti da irrigazioni abbondanti causano tuberi deformi e cuore cavo.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Peronospora (Phytophthora infestans): con un «periodo di Smith» (2 giorni con minime ≥ 10 °C e almeno 11 ore con umidità ≥ 90 %) il rischio di infezione è alto. Indicatore calcolato in «Oggi in campo».",
          "Tignola (Phthorimaea operculella) in estate e in magazzino; elateridi nei terreni dopo prato; dorifora.",
          "Usare solo tubero-seme certificato.",
        ],
      },
      {
        titolo: "Perché il caldo blocca la tuberificazione",
        tipo: "scienza",
        punti: [
          "La formazione dei tuberi è favorita da notti fresche (10–14 °C) e si arresta con il suolo sopra i 25–27 °C, perché la pianta destina gli zuccheri alla crescita dei fusti.",
          "Per questo in Sardegna i cicli produttivi evitano l'estate.",
        ],
      },
    ],
  },
  {
    id: "cipolla",
    nome: "Cipolla",
    latino: "Allium cepa L.",
    famiglia: "Amaryllidaceae",
    sezione: "orticole",
    sintesi: "La bulbificazione dipende dalla lunghezza del giorno: scegliere varietà adatte al periodo di coltivazione.",
    tSuolo: { min: 3, opt: "18–25" },
    gelo: -6,
    calendario: { semina: [9, 10, 1, 2], trapianto: [11, 12, 1, 2], raccolta: [5, 6, 7, 8] },
    kc: [0.7, 1.05, 0.75],
    dati: [
      ["Semina", "settembre – ottobre o gennaio – febbraio"],
      ["Trapianto", "novembre – febbraio"],
      ["Germinazione", "minimo 2–4 °C, ottimo 20–25 °C"],
      ["Densità", "50–80 piante/m²"],
      ["Profondità di semina", "1–2 cm"],
      ["N – P₂O₅ – K₂O", "100–130 · 60–80 · 120–150 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Sospendere l'irrigazione 2–3 settimane prima della raccolta per favorire la conservabilità.",
          "Raccogliere i bulbi da serbo quando il 50–80 % dei colletti è ripiegato; farli asciugare in campo o all'ombra.",
          "Sensibile alla salinità (soglia ECe 1,2 dS/m).",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Peronospora (Peronospora destructor) con primavere umide e fresche.",
          "Mosca della cipolla (Delia antiqua) e tripidi.",
        ],
      },
      {
        titolo: "Fotoperiodo",
        tipo: "scienza",
        punti: [
          "La pianta forma il bulbo solo quando il giorno supera una certa durata: varietà a giorno corto (11–12 ore) per i cicli autunno-invernali del Sud, a giorno lungo (14–16 ore) per quelli primaverili.",
          "Una varietà sbagliata non bulbifica o bulbifica troppo presto, con bulbi piccoli.",
        ],
      },
    ],
  },
  {
    id: "aglio",
    nome: "Aglio",
    latino: "Allium sativum L.",
    famiglia: "Amaryllidaceae",
    sezione: "orticole",
    sintesi: "Si pianta in autunno: il freddo invernale è necessario per una buona formazione del bulbo.",
    tSuolo: { min: 3, opt: "12–18" },
    gelo: -8,
    calendario: { semina: [10, 11, 12], raccolta: [6, 7] },
    kc: [0.7, 1.0, 0.7],
    dati: [
      ["Impianto bulbilli", "ottobre – dicembre"],
      ["Profondità", "3–5 cm, punta verso l'alto"],
      ["Sesto", "0,3–0,4 × 0,08–0,12 m"],
      ["Bulbilli", "800–1200 kg/ha"],
      ["N – P₂O₅ – K₂O", "80–120 · 60–80 · 100–150 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Usare bulbilli grossi e sani di materiale certificato: la pezzatura del bulbillo influenza quella del bulbo.",
          "Raccolta quando 2/3 delle foglie sono secche; asciugatura all'ombra e in luogo ventilato.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Marciume bianco (Sclerotium cepivorum): sopravvive molti anni nel suolo, rotazioni di 6–8 anni.",
          "Ruggine (Puccinia allii) in primavera; nematode dei bulbi (Ditylenchus dipsaci), che si diffonde con bulbilli infetti.",
        ],
      },
      {
        titolo: "Perché serve il freddo",
        tipo: "scienza",
        punti: [
          "L'esposizione dei bulbilli o delle giovani piante a temperature intorno a 0–10 °C per alcune settimane (vernalizzazione) induce la differenziazione dei bulbilli: con inverni troppo miti si formano bulbi poco divisi.",
        ],
      },
    ],
  },
  {
    id: "lattuga",
    nome: "Lattuga",
    latino: "Lactuca sativa L.",
    famiglia: "Asteraceae",
    sezione: "orticole",
    sintesi: "Da fine estate a primavera in pieno campo. Il seme non germina con il suolo troppo caldo; le foglie accumulano nitrati con poca luce.",
    tSuolo: { min: 3, opt: "15–20", max: 25 },
    gelo: -3,
    calendario: { semina: [8, 9, 10, 11, 1, 2, 3], trapianto: [9, 10, 11, 2, 3, 4], raccolta: [10, 11, 12, 1, 2, 3, 4, 5] },
    kc: [0.7, 1.0, 0.95],
    dati: [
      ["Trapianto", "settembre – novembre e febbraio – aprile"],
      ["Germinazione", "minimo 2–4 °C, ottimo 15–20 °C"],
      ["Inibizione", "oltre 25–28 °C (termodormienza)"],
      ["Sesto", "0,3 × 0,3 m (8–12 piante/m²)"],
      ["Ciclo dal trapianto", "45–70 giorni"],
      ["N – P₂O₅ – K₂O", "80–120 · 50–60 · 120–150 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "In estate seminare in contenitori al fresco o la sera, o usare semi pre-trattati: con il suolo caldo il seme entra in dormienza.",
          "Giorni lunghi e caldo provocano la prefioritura (salita a seme) e il gusto amaro: in estate coltivare solo nelle zone interne fresche.",
          "Irrigazioni leggere e frequenti; sbalzi idrici e caldo causano il tip-burn (necrosi dei margini fogliari, legata al calcio).",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Peronospora (Bremia lactucae) con umidità e temperature fresche: varietà resistenti, irrigazione a goccia.",
          "Sclerotinia e botrite al colletto; afidi.",
        ],
      },
      {
        titolo: "Nitrati nelle foglie",
        tipo: "scienza",
        punti: [
          "L'enzima che trasforma i nitrati (nitrato reduttasi) lavora con la luce: con giornate corte e nuvolose i nitrati si accumulano.",
          "Raccogliere nel pomeriggio di giornate luminose e non eccedere con l'azoto, soprattutto in inverno.",
        ],
      },
    ],
  },
  {
    id: "cavoli",
    nome: "Cavolfiore e broccolo",
    latino: "Brassica oleracea L. var. botrytis e var. italica",
    famiglia: "Brassicaceae",
    sezione: "orticole",
    sintesi: "Trapianti estivi per raccolte autunno-invernali. Esigenti in azoto, boro e molibdeno.",
    tSuolo: { min: 7, opt: "18–25" },
    gelo: -3,
    calendario: { semenzaio: [6, 7, 8], trapianto: [7, 8, 9], raccolta: [10, 11, 12, 1, 2, 3] },
    kc: [0.7, 1.05, 0.95],
    dati: [
      ["Semenzaio", "giugno – agosto"],
      ["Trapianto", "luglio – settembre"],
      ["Germinazione", "minimo 5 °C, ottimo 20–25 °C"],
      ["Sesto", "0,8 × 0,5–0,6 m"],
      ["N – P₂O₅ – K₂O", "150–200 · 80–100 · 200–250 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Trapiantare piantine giovani e vigorose, irrigando subito: lo stress dopo il trapianto estivo causa una formazione prematura di piccoli corimbi.",
          "Boro contro il «cuore cavo» del fusto; molibdeno nei suoli acidi.",
          "Le piante sopportano −5 °C, ma il corimbo del cavolfiore si danneggia intorno a −2/−3 °C.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Cavolaia (Pieris brassicae) e nottue: Bacillus thuringiensis sulle larve giovani.",
          "Altica e mosca del cavolo nelle prime fasi; afide ceroso in autunno.",
          "Ernia del cavolo (Plasmodiophora brassicae) nei suoli acidi: calcitazione e rotazioni lunghe.",
        ],
      },
    ],
  },
  {
    id: "finocchio",
    nome: "Finocchio",
    latino: "Foeniculum vulgare Mill. var. azoricum",
    famiglia: "Apiaceae",
    sezione: "orticole",
    sintesi: "Coltura autunno-invernale tipica del Sud. Le semine primaverili rischiano la prefioritura.",
    tSuolo: { min: 8, opt: "18–25" },
    gelo: -3,
    calendario: { semina: [7, 8, 9], trapianto: [8, 9], raccolta: [10, 11, 12, 1, 2, 3, 4] },
    kc: [0.7, 1.05, 1.0],
    dati: [
      ["Semina o trapianto", "luglio – settembre"],
      ["Germinazione", "minimo 8–10 °C, ottimo 20–25 °C"],
      ["Sesto", "0,6–0,7 × 0,2–0,25 m"],
      ["N – P₂O₅ – K₂O", "100–150 · 60–80 · 150–200 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Rincalzare a metà ciclo per imbianchire e rendere più tenero il grumolo.",
          "Irrigazione costante: alternanze secco/umido causano la spaccatura del grumolo.",
          "Con giorni lunghi la pianta va a fiore: nelle semine di fine inverno usare varietà resistenti alla prefioritura.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Sclerotinia e marciumi del colletto con ristagno; afidi.",
          "Coefficiente colturale indicativo, non riportato da FAO-56: assimilato alle altre Apiacee.",
        ],
      },
    ],
  },
  {
    id: "carota",
    nome: "Carota",
    latino: "Daucus carota L.",
    famiglia: "Apiaceae",
    sezione: "orticole",
    sintesi: "Semina diretta su suolo sciolto e profondo. L'emergenza è lenta e richiede umidità costante in superficie.",
    tSuolo: { min: 5, opt: "18–25" },
    gelo: -5,
    calendario: { semina: [1, 2, 3, 8, 9, 10], raccolta: [4, 5, 6, 11, 12, 1, 2] },
    kc: [0.7, 1.05, 0.95],
    dati: [
      ["Semina", "agosto – ottobre e gennaio – marzo"],
      ["Germinazione", "minimo 4–5 °C, ottimo 20–25 °C (10–20 giorni)"],
      ["Profondità", "1–1,5 cm"],
      ["Densità", "60–100 piante/m²"],
      ["N – P₂O₅ – K₂O", "80–120 · 60–80 · 150–200 kg/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Suolo sciolto e senza sassi: gli ostacoli e il letame fresco causano radici biforcute.",
          "Tenere umido lo strato superficiale fino all'emergenza; coprire la fila con sabbia o vermiculite nei suoli che fanno crosta.",
          "Ideale una falsa semina: l'emergenza lenta rende la carota poco competitiva con le infestanti.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Mosca della carota (Psila rosae): reti antinsetto, semine che evitino i voli principali.",
          "Alternaria fogliare con umidità elevata; nematodi nei suoli sabbiosi.",
        ],
      },
    ],
  },
  {
    id: "spinacio",
    nome: "Spinacio",
    latino: "Spinacia oleracea L.",
    famiglia: "Amaranthaceae",
    sezione: "orticole",
    sintesi: "Coltura invernale rapida e resistente al freddo. Con giorni lunghi e caldo va a seme.",
    tSuolo: { min: 2, opt: "15–20", max: 26 },
    gelo: -6,
    calendario: { semina: [9, 10, 11, 12, 1, 2], raccolta: [11, 12, 1, 2, 3, 4] },
    kc: [0.7, 1.0, 0.95],
    dati: [
      ["Semina", "settembre – febbraio"],
      ["Germinazione", "minimo 2 °C, ottimo 15–20 °C, inibita oltre 25–30 °C"],
      ["Dose di seme", "25–35 kg/ha, file a 25–30 cm"],
      ["Ciclo", "40–60 giorni"],
      ["Azoto", "80–120 kg N/ha"],
    ],
    blocchi: [
      {
        titolo: "Gestione",
        punti: [
          "Pianta longidiurna: con l'allungarsi del giorno in primavera va a seme, quindi le semine tardive vanno limitate.",
          "Come la lattuga accumula nitrati con poca luce: raccogliere nelle ore luminose e frazionare l'azoto.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: ["Peronospora (Peronospora effusa): varietà resistenti alle razze più diffuse.", "Afidi e minatori fogliari."],
      },
    ],
  },

  // ============================================================ CANAPA
  {
    id: "canapa-normativa",
    nome: "Quadro normativo",
    sezione: "canapa",
    legale: true,
    sintesi: "Cosa è consentito coltivare e commercializzare. Verificare sempre la normativa vigente prima di seminare.",
    blocchi: [
      {
        titolo: "Cosa dice la legge (situazione nota a metà 2026)",
        punti: [
          "Legge 242/2016: la coltivazione della canapa industriale è consentita senza autorizzazione, purché si usino varietà iscritte nel Catalogo comune europeo delle varietà agricole e seme certificato (non autoprodotto).",
          "Conservare i cartellini della semente per almeno 12 mesi e le fatture di acquisto. È buona prassi comunicare la semina al più vicino comando delle forze dell'ordine.",
          "Contenuto di THC: per gli aiuti PAC la soglia è dello 0,3 %; la legge 242/2016 esclude la responsabilità dell'agricoltore che abbia rispettato le norme se il THC totale resta entro lo 0,6 %.",
          "Il decreto-legge 48/2025 (convertito dalla legge 80/2025, art. 18) vieta la cessione, il commercio e la lavorazione delle infiorescenze, anche essiccate o trinciate, e dei prodotti che le contengono.",
          "Orientare quindi la coltura a fibra, canapulo e seme. La coltivazione di varietà non certificate o a scopo stupefacente resta reato (DPR 309/1990).",
          "La materia è oggetto di contenziosi e possibili modifiche: prima della semina verificare la normativa con l'associazione di categoria o un consulente.",
        ],
      },
    ],
  },
  {
    id: "canapa",
    nome: "Canapa industriale",
    latino: "Cannabis sativa L.",
    famiglia: "Cannabaceae",
    sezione: "canapa",
    sintesi: "Coltura primaverile-estiva da fibra e da seme. Cresce rapidamente, soffoca le infestanti e lascia il suolo strutturato per il cereale successivo.",
    tSuolo: { min: 10, opt: "12–20" },
    gelo: -2,
    calendario: { semina: [4, 5], raccolta: [7, 8, 9] },
    kc: [0.4, 1.15, 0.7],
    dati: [
      ["Semina", "aprile – maggio, suolo ≥ 10 °C"],
      ["Profondità", "2–3 cm"],
      ["Fibra", "40–60 kg/ha seme, file a 15–20 cm"],
      ["Seme", "20–30 kg/ha, file a 30–45 cm"],
      ["N – P₂O₅ – K₂O", "80–120 · 60–80 · 80–150 kg/ha"],
      ["pH", "6–7,5 (tollera 5,5–8)"],
      ["Kc", "valori indicativi da letteratura"],
    ],
    blocchi: [
      {
        titolo: "Varietà e fotoperiodo",
        punti: [
          "La canapa è brevidiurna: fiorisce quando il giorno si accorcia. Alle latitudini sarde (39–41° N) le varietà nordiche fioriscono presto con piante basse, quindi sono adatte al seme.",
          "Per la fibra preferire varietà tardive (es. le italiane dioiche Carmagnola, Fibranova, Eletta Campana); per seme e doppio uso varietà monoiche più precoci (es. Futura 75, Felina 32, Fedora 17).",
          "Verificare sempre che la varietà sia iscritta nel Catalogo europeo nell'anno di semina.",
        ],
      },
      {
        titolo: "Suolo e semina",
        punti: [
          "Suoli profondi, di medio impasto e ben drenati: la radice fittonante teme il ristagno e il compattamento (attenzione alla suola di lavorazione).",
          "Letto di semina fine come per il frumento; si usa la stessa seminatrice a righe.",
          "Germina già a 1–2 °C, ma lentamente: con il suolo sopra i 10 °C l'emergenza è rapida e uniforme e la coltura chiude la fila in 4–6 settimane.",
          "Falsa semina consigliata: dopo la chiusura della fila la canapa controlla da sola le infestanti, mentre gli erbicidi autorizzati sono pochissimi.",
        ],
      },
      {
        titolo: "Acqua e nutrizione",
        punti: [
          "Nelle semine primaverili sarde le piogge non bastano: un'irrigazione di soccorso in giugno–luglio è decisiva, soprattutto per il seme (fioritura e riempimento).",
          "Un eccesso di azoto causa allettamento e una fibra di minore qualità; il potassio sostiene la produzione di fibra.",
        ],
      },
      {
        titolo: "Raccolta",
        punti: [
          "Fibra: a piena fioritura maschile, quando la qualità della fibra è massima; poi macerazione in campo (2–5 settimane, ruotando gli steli).",
          "Seme: con il 60–70 % dei semi maturi (brunastri); essiccare subito sotto il 10 % di umidità per evitare riscaldamento e irrancidimento.",
          "Gli uccelli possono causare forti perdite di seme a maturazione.",
        ],
      },
      {
        titolo: "Avversità",
        tipo: "attenzione",
        punti: [
          "Muffa grigia (Botrytis cinerea) e sclerotinia con umidità elevata a fine ciclo.",
          "Piralide (Ostrinia nubilalis) sugli steli.",
          "Orobanche (Phelipanche ramosa): la canapa ne è ospite, come pomodoro e tabacco. Evitare la successione con queste colture nei suoli infestati.",
        ],
      },
      {
        titolo: "Perché è una buona coltura da rinnovo",
        tipo: "scienza",
        punti: [
          "La crescita rapida e la chioma fitta riducono la luce al suolo e sopprimono le infestanti.",
          "La radice fittonante esplora gli strati profondi e ne migliora la struttura: il cereale che segue ne trae vantaggio in resa.",
        ],
      },
    ],
  },
];

/** Fonti e criteri usati nell'almanacco. */
export const FONTI = [
  "Allen R. G., Pereira L. S., Raes D., Smith M. (1998). Crop evapotranspiration. FAO Irrigation and Drainage Paper 56 — coefficienti colturali Kc ed ET0 (Penman-Monteith).",
  "Ayers R. S., Westcot D. W. (1985). Water quality for agriculture. FAO Irrigation and Drainage Paper 29 — soglie di tolleranza alla salinità.",
  "INRA (2018). Alimentation des ruminants — sistemi UFL/PDI per i fabbisogni degli ovini da latte.",
  "Fulkerson W. J., Donaghy D. J. (2001). Plant-soluble carbohydrate reserves and senescence: key criteria for developing an effective grazing management system for ryegrass-based pastures. Australian Journal of Experimental Agriculture 41.",
  "Periodo di Smith per la peronospora della patata (Met Office, Regno Unito): 2 giorni consecutivi con minima ≥ 10 °C e almeno 11 ore con umidità relativa ≥ 90 %.",
  "Indice THI (Temperature Humidity Index) secondo la formula NRC usata nel cruscotto, con soglie di benessere per piccoli ruminanti.",
  "Agenzie regionali AGRIS Sardegna e LAORE Sardegna; Disciplinari di produzione integrata della Regione Sardegna (dosi massime di concimazione e difesa).",
  "Legge 2 dicembre 2016, n. 242; decreto-legge 11 aprile 2025, n. 48, convertito dalla legge 9 giugno 2025, n. 80 (art. 18); DPR 309/1990.",
  "Regolamento (UE) 2021/2115 e norme di condizionalità (BCAA 5 e 6) della PAC 2023–2027.",
];

/** Avvertenze generali mostrate nella sezione delle fonti. */
export const AVVERTENZE = [
  "I valori sono orientativi per il pieno campo in Sardegna e vanno adattati a quota, esposizione, suolo, varietà e andamento stagionale.",
  "Le concimazioni vanno definite con un piano basato sull'analisi del suolo e sui limiti del Disciplinare di produzione integrata regionale; nelle Zone vulnerabili ai nitrati valgono limiti specifici.",
  "Per la difesa usare solo prodotti autorizzati sulla coltura, rispettando etichetta, tempi di carenza e Piano d'azione nazionale (PAN).",
  "I consigli dinamici dipendono dalla qualità delle previsioni nel punto scelto: tararli con le osservazioni di campo e con la sezione «Calibrazione stazione».",
];
