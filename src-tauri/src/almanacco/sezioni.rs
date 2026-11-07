//! Sezioni dell'almanacco colturale, mesi, fonti scientifiche e avvertenze.
//!
//! Separato dai dati delle singole colture per facilitare la manutenzione
//! dell'architettura informativa e della bibliografia di riferimento.

use super::SezioneAlmanacco;

/// Abbreviazioni dei 12 mesi (indice 0 = gennaio).
pub const MESI: [&str; 12] = ["Gen", "Feb", "Mar", "Apr", "Mag", "Giu", "Lug", "Ago", "Set", "Ott", "Nov", "Dic"];

/// Nomi estesi dei 12 mesi in italiano (indice 0 = gennaio).
pub const NOMI_MESI: [&str; 12] = [
    "gennaio", "febbraio", "marzo", "aprile", "maggio", "giugno",
    "luglio", "agosto", "settembre", "ottobre", "novembre", "dicembre",
];

/// Sezioni principali dell'Almanacco colturale.
pub const SEZIONI: &[SezioneAlmanacco] = &[
    SezioneAlmanacco {
        id: "oggi",
        titolo: "Oggi in campo",
        icona: "☀️",
        illustrazione: "sez-oggi",
        intro: "Consigli operativi generati dal motore Rust incrociando i dati meteo e del suolo della località attiva con le soglie fisiologiche delle colture. Si aggiornano automaticamente a ogni nuovo report.",
        principi: &[
            ("Decisioni guidate dal suolo", "Temperatura a 6 cm e frazione di acqua disponibile governano semine, trapianti e arature più della semplice data di calendario."),
            ("Finestre operative a 72 ore", "Ogni operazione (semina, concimazione azotata, sfalcio, difesa) viene incrociata con pioggia, vento, rugiada e minime termiche delle prossime 72 ore."),
            ("Adattamento al clima sardo", "I modelli tengono conto della stagionalità mediterranea: autunni piovosi, inverni miti con rischio di gelate da irraggiamento nei fondovalle e siccità estiva."),
        ],
    },
    SezioneAlmanacco {
        id: "suolo",
        titolo: "Lavorazioni del terreno",
        icona: "🚜",
        illustrazione: "sez-suolo",
        intro: "Aratura, lavorazioni conservative e preparazione del letto di semina. Il momento giusto si decide sull'umidità del suolo (lo stato di «tempera»), non sul calendario.",
        principi: &[
            ("Lavorare solo in tempera", "Un suolo troppo umido si compatta e si «impasta» sotto il versoio; uno troppo secco forma zolle enormi e consuma più gasolio."),
            ("Lavorare il meno possibile", "Ogni passaggio ossida la sostanza organica e aumenta il rischio di compattamento. Meno passaggi, fatti al momento giusto."),
            ("Proteggere i versanti", "In Sardegna le piogge autunnali intense colpiscono suoli appena lavorati: a girapoggio, con residui in superficie, mai prima di un forte evento piovoso."),
        ],
    },
    SezioneAlmanacco {
        id: "cereali",
        titolo: "Cereali: avena e orzo",
        icona: "🌾",
        illustrazione: "sez-cereali",
        intro: "Cereali autunno-vernini da granella, da erbaio e da fieno. In Sardegna sfruttano la stagione piovosa (ottobre–maggio) e devono completare il ciclo prima della siccità estiva.",
        principi: &[
            ("Seminare con le prime piogge utili", "Servono 20–25 mm cumulati per un'emergenza uniforme; una pioggia leggera seguita da giorni secchi fa germinare e poi seccare le plantule."),
            ("Azoto frazionato", "Poco o nulla alla semina, il resto tra accestimento e inizio levata, quando la coltura lo assorbe davvero: meno perdite per dilavamento."),
            ("Rispettare la levata", "Dopo l'inizio della levata l'apice della spiga sale dal suolo: pascolarlo o tagliarlo basso significa perdere il ricaccio."),
        ],
    },
    SezioneAlmanacco {
        id: "foraggere",
        titolo: "Foraggere: trifogli, medica, loietto",
        icona: "🍀",
        illustrazione: "sez-foraggere",
        intro: "Leguminose che fissano l'azoto atmosferico e graminacee ad alta produzione: la base di pascoli, erbai e fieni per gli allevamenti sardi.",
        principi: &[
            ("Leguminose: fosforo sì, azoto no", "La simbiosi con i rizobi fornisce 50–200 kg N/ha l'anno; il fosforo è invece indispensabile per la nodulazione."),
            ("Seme piccolo, letto fine", "Medica, trifogli e loietto vanno seminati a 1–1,5 cm su terreno affinato e rullato: il contatto seme-suolo garantisce l'imbibizione."),
            ("Utilizzo allo stadio giusto", "Resa e qualità vanno in direzioni opposte: più si attende, più biomassa ma meno proteina e digeribilità."),
        ],
    },
    SezioneAlmanacco {
        id: "ovini",
        titolo: "Erbai e foraggi per la pecora Sarda",
        icona: "🐑",
        illustrazione: "sez-ovini",
        intro: "Programmare la produzione foraggera sul ciclo della pecora di razza Sarda: il picco dei fabbisogni (fine gestazione e inizio lattazione, ottobre–gennaio) cade proprio quando il pascolo naturale produce poco.",
        principi: &[
            ("Colmare il vuoto autunno-invernale", "Erbai seminati presto (settembre–ottobre) sono pascolabili 50–70 giorni dopo l'emergenza, quando il pascolo naturale è ancora scarso."),
            ("Cambi di dieta graduali", "Passare a un erbaio lussureggiante in 7–15 giorni, con fieno a disposizione: si prevengono meteorismo, acidosi ed enterotossiemia."),
            ("Conservare l'eccesso primaverile", "Ad aprile–maggio la crescita supera il consumo: fieno e fasciato trasformano l'eccedenza in scorte per l'estate e l'autunno."),
        ],
    },
    SezioneAlmanacco {
        id: "orticole",
        titolo: "Orticole",
        icona: "🍅",
        illustrazione: "sez-orticole",
        intro: "Le principali specie orticole di pieno campo in Sardegna, con epoche, temperature cardinali, consumi idrici (Kc FAO-56) e difesa. Per serra e tunnel le epoche si anticipano o si estendono.",
        principi: &[
            ("Ruotare per famiglie", "Non tornare con la stessa famiglia sullo stesso appezzamento prima di 3–4 anni: Fusarium, Verticillium e nematodi si accumulano nel suolo."),
            ("Goccia e pacciamatura", "L'irrigazione a goccia raggiunge efficienze dell'85–95 % e lascia la chioma asciutta: meno peronospora e meno acqua sprecata."),
            ("Rispettare le temperature del suolo", "Trapiantare solanacee e cucurbitacee in un suolo freddo blocca le radici per settimane: meglio una settimana dopo che una prima."),
        ],
    },
    SezioneAlmanacco {
        id: "canapa",
        titolo: "Canapa (Cannabis sativa L.)",
        icona: "🌿",
        illustrazione: "sez-canapa",
        intro: "Canapa industriale da fibra e da seme con varietà certificate. Coltura da rinnovo che migliora la struttura del suolo e lascia il campo pulito per il cereale successivo.",
        principi: &[
            ("Seme certificato e tracciabilità", "Utilizzare esclusivamente varietà iscritte nel Catalogo comune europeo, conservando cartellini e fatture per almeno 12 mesi."),
            ("Suolo ben drenato e strutturato", "Il fittone profondo teme ristagni e suola di lavorazione, ma lascia un'ottima porosità per il cereale autunno-vernino successivo."),
            ("Scelta varietale sul fotoperiodo", "Alle latitudini sarde (39–41° N) le varietà dioiche tardive italiane valorizzano la fibra, quelle monoiche precoci il seme."),
        ],
    },
    SezioneAlmanacco {
        id: "fonti",
        titolo: "Metodo e fonti",
        icona: "📚",
        illustrazione: "sez-fonti",
        intro: "Da dove vengono i numeri dell'almanacco e come sono collegati ai dati meteo dell'applicazione.",
        principi: &[],
    },
];

/// Fonti scientifiche, tecniche e normative usate nell'almanacco.
pub const FONTI: &[&str] = &[
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

/// Avvertenze agronomiche generali mostrate nella sezione delle fonti.
pub const AVVERTENZE: &[&str] = &[
    "I valori sono orientativi per il pieno campo in Sardegna e vanno adattati a quota, esposizione, suolo, varietà e andamento stagionale.",
    "Le concimazioni vanno definite con un piano basato sull'analisi del suolo e sui limiti del Disciplinare di produzione integrata regionale; nelle Zone vulnerabili ai nitrati valgono limiti specifici.",
    "Per la difesa usare solo prodotti autorizzati sulla coltura, rispettando etichetta, tempi di carenza e Piano d'azione nazionale (PAN).",
    "I consigli dinamici dipendono dalla qualità delle previsioni nel punto scelto: tararli con le osservazioni di campo e con la sezione «Calibrazione stazione».",
];
