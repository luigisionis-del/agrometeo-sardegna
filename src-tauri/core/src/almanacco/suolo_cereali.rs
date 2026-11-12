//! Schede agronomiche per le sezioni «Lavorazioni del terreno» e «Cereali».

use super::{BloccoScheda, CalendarioColturale, SchedaColturale, SoglieSuolo};

pub const SCHEDE_SUOLO_CEREALI: &[SchedaColturale] = &[
    // ============================================================ LAVORAZIONI
    SchedaColturale {
        id: "aratura",
        nome: "Aratura",
        latino: None,
        famiglia: None,
        sezione: "suolo",
        illustrazione: "scheda-aratura",
        infografica: Some("info-tempera"),
        legale: false,
        sintesi: "Quando, quanto in profondità e come arare per preparare il terreno senza danneggiarne la struttura.",
        t_suolo: None,
        gelo: None,
        calendario: None,
        kc: None,
        dati: &[
            ("Profondità seminativi", "25–35 cm"),
            ("Profondità erbai e cereali", "20–30 cm (spesso sufficiente)"),
            ("Scasso per impianti arborei", "50–80 cm, una tantum"),
            ("Epoca per semine autunnali", "fine estate – autunno, in tempera"),
            ("Velocità di lavoro", "5–7 km/h"),
            ("Pressione pneumatici", "0,8–1,2 bar in campo"),
        ],
        tabella: None,
        blocchi: &[
            BloccoScheda {
                titolo: "Quando arare: lo stato di tempera",
                tipo: None,
                punti: &[
                    "Il suolo è in tempera quando l'umidità è sotto la capacità di campo ma sopra la soglia in cui diventa duro e zolloso; nell'app corrisponde al semaforo verde «Lavorabilità» (strato 3–9 cm) e alla valutazione dello strato 9–27 cm in «Oggi in campo».",
                    "Prova della manciata: prendere terra a 10–15 cm e stringerla. In tempera si modella senza appiccicarsi e si sbriciola sotto il pollice. Se forma un cilindretto lucido è troppo umida; se non si aggrega affatto è troppo secca.",
                    "Il versoio pulito e la fetta che si rovescia sgretolandosi sono il segnale in campo; una fetta lucida e compatta indica eccesso d'acqua.",
                    "Suoli sabbiosi: ampia finestra di tempera, si lavorano anche poco dopo la pioggia. Suoli argillosi (es. vertisuoli del Campidano): finestra di pochi giorni, da cogliere.",
                ],
            },
            BloccoScheda {
                titolo: "Epoca per tipo di suolo",
                tipo: None,
                punti: &[
                    "Argillosi: arare a fine estate o subito dopo le prime piogge, quando la tempera c'è: i cicli bagnato/asciutto dell'autunno e del freddo invernale affinano naturalmente le zolle.",
                    "Franchi: lavorare in autunno 2–4 settimane prima della semina, così da poter fare una falsa semina.",
                    "Sabbiosi: lavorazioni leggere e vicine alla semina, per non esporre a lungo il suolo all'erosione eolica e alla perdita di umidità.",
                    "Per le colture primaverili (orticole, canapa) lasciare il terreno coperto in inverno e lavorare a fine inverno, in tempera.",
                ],
            },
            BloccoScheda {
                titolo: "Perché non arare un suolo bagnato",
                tipo: Some("scienza"),
                punti: &[
                    "Ruote e versoio deformano un suolo plastico: i macropori (le «vie» di aria e acqua) collassano. Sotto il 10 % circa di porosità ad aria le radici soffrono di asfissia.",
                    "Il fondo del solco, lisciato dal versoio, forma la «suola di lavorazione»: uno strato compatto che le radici superano a fatica. Una resistenza alla penetrazione superiore a circa 2 MPa limita già sensibilmente la crescita radicale.",
                    "Il compattamento profondo può durare anni: è molto più facile prevenirlo che correggerlo.",
                ],
            },
            BloccoScheda {
                titolo: "Profondità e sostanza organica",
                tipo: Some("scienza"),
                punti: &[
                    "L'aratura profonda porta in superficie suolo meno fertile e accelera la mineralizzazione della sostanza organica: si perde carbonio, struttura e capacità di trattenere l'acqua.",
                    "Per cereali ed erbai 20–30 cm bastano: le radici esplorano gli strati sottostanti se il suolo non è compattato.",
                    "Variare la profondità tra un anno e l'altro evita la suola; se presente, una ripuntatura a 40–50 cm ogni 3–4 anni (in suolo asciutto, perché si fessuri) la rompe senza rovesciare gli strati.",
                ],
            },
            BloccoScheda {
                titolo: "Versanti, erosione e regole PAC",
                tipo: Some("attenzione"),
                punti: &[
                    "Su pendii lavorare a girapoggio (lungo le curve di livello), non a rittochino: i solchi trasversali rallentano il ruscellamento.",
                    "Non arare se nelle 48–72 ore sono previsti più di 20–30 mm: il suolo appena lavorato e nudo è il più esposto all'erosione.",
                    "La formazione del suolo è dell'ordine di 1 t/ha l'anno: un singolo evento erosivo può asportarne molte di più.",
                    "Condizionalità PAC: BCAA 5 (lavorazioni che tengano conto della pendenza) e BCAA 6 (copertura minima del suolo nei periodi sensibili). Verificare le regole regionali vigenti.",
                ],
            },
        ],
    },
    SchedaColturale {
        id: "lavorazioni-conservative",
        nome: "Lavorazioni conservative e semina su sodo",
        latino: None,
        famiglia: None,
        sezione: "suolo",
        illustrazione: "scheda-conservative",
        infografica: None,
        legale: false,
        sintesi: "Minima lavorazione, ripuntatura, strip-till e semina diretta: meno passaggi, più sostanza organica e semine più tempestive.",
        t_suolo: None,
        gelo: None,
        calendario: None,
        kc: None,
        dati: &[
            ("Minima lavorazione", "10–15 cm con dischi o coltivatore"),
            ("Ripuntatura", "40–50 cm, suolo asciutto, ogni 3–4 anni"),
            ("Semina su sodo", "seminatrice da sodo, nessuna lavorazione"),
            ("Risparmio di gasolio", "fino al 50–70 % rispetto all'aratura"),
        ],
        tabella: None,
        blocchi: &[
            BloccoScheda {
                titolo: "Vantaggi nel contesto sardo",
                tipo: None,
                punti: &[
                    "Tra le prime piogge autunnali e la semina il tempo è poco: con la semina su sodo si semina appena il suolo è umido, senza attendere la tempera per arare.",
                    "I residui in superficie riducono l'evaporazione e proteggono dall'impatto della pioggia, limitando crosta ed erosione.",
                    "Nel medio periodo (3–5 anni) aumentano sostanza organica superficiale, attività dei lombrichi e stabilità degli aggregati.",
                    "Trasemina dei pascoli: con seminatrici da sodo si introducono trifoglio sotterraneo e loietto rigido in pascoli degradati senza rompere il cotico.",
                ],
            },
            BloccoScheda {
                titolo: "Condizioni per riuscire",
                tipo: Some("attenzione"),
                punti: &[
                    "Controllo delle infestanti prima della semina (falsa semina, pascolamento intenso o diserbo autorizzato): senza aratura la banca semi resta in superficie.",
                    "Residui ben distribuiti dalla mietitrebbia, altrimenti la seminatrice non deposita correttamente il seme.",
                    "Nei primi anni possono servire più azoto e una gestione attenta delle limacce, favorite dai residui.",
                    "Su suoli già compattati conviene una ripuntatura iniziale prima di passare al sodo.",
                ],
            },
            BloccoScheda {
                titolo: "Perché funziona",
                tipo: Some("scienza"),
                punti: &[
                    "Senza inversione degli strati la sostanza organica si mineralizza più lentamente e si accumula in superficie, dove migliora infiltrazione e struttura.",
                    "I canali lasciati dalle radici morte e dai lombrichi restano continui: l'acqua si infiltra meglio di quanto accada in un suolo arato e poi compattato.",
                ],
            },
        ],
    },
    SchedaColturale {
        id: "letto-semina",
        nome: "Letto di semina e falsa semina",
        latino: None,
        famiglia: None,
        sezione: "suolo",
        illustrazione: "scheda-letto-semina",
        infografica: Some("info-profondita-seme"),
        legale: false,
        sintesi: "Affinare il terreno in proporzione alla dimensione del seme, interrare fosforo e potassio e ridurre le infestanti prima di seminare.",
        t_suolo: None,
        gelo: None,
        calendario: None,
        kc: None,
        dati: &[
            ("Regola della profondità", "2–3 volte il diametro del seme"),
            ("Semi piccoli", "medica, trifogli, loietto, carota: 0,5–1,5 cm"),
            ("Semi grossi", "fava, pisello, fagiolo: 3–6 cm"),
            ("Falsa semina", "preparare 2–3 settimane prima"),
        ],
        tabella: None,
        blocchi: &[
            BloccoScheda {
                titolo: "Affinamento giusto",
                tipo: None,
                punti: &[
                    "Semi piccoli: letto fine e assestato, poi rullatura dopo la semina per garantire il contatto con il suolo e la risalita capillare dell'acqua.",
                    "Semi grossi: tollerano zolle di qualche centimetro, che anzi riducono la formazione di crosta.",
                    "Evitare di polverizzare i suoli limosi: con la prima pioggia battente formano una crosta che impedisce l'emergenza.",
                ],
            },
            BloccoScheda {
                titolo: "Falsa semina contro le infestanti",
                tipo: None,
                punti: &[
                    "Preparare il letto 2–3 settimane prima della semina, lasciar emergere le infestanti dopo una pioggia o un'irrigazione, poi eliminarle con una lavorazione superficiale (meno di 5 cm) o con il pirodiserbo.",
                    "Lavorare più in profondità riporterebbe in superficie nuovi semi: l'obiettivo è esaurire lo strato superficiale della banca semi.",
                    "Particolarmente utile per orticole a semina diretta (carota, spinacio) e per gli erbai in agricoltura biologica.",
                ],
            },
            BloccoScheda {
                titolo: "Concimazione di fondo",
                tipo: Some("scienza"),
                punti: &[
                    "Fosforo e potassio si muovono poco nel suolo: vanno distribuiti e interrati alla preparazione, vicino a dove cresceranno le radici.",
                    "L'azoto invece è mobile (nitrato) e va dato quando la coltura lo assorbe: alla semina basta una piccola quota, il resto in copertura.",
                    "Nei suoli calcarei sardi il fosforo si lega al calcio e diventa poco disponibile: la localizzazione alla semina ne aumenta l'efficienza.",
                ],
            },
        ],
    },

    // ============================================================ CEREALI
    SchedaColturale {
        id: "avena",
        nome: "Avena",
        latino: Some("Avena sativa L."),
        famiglia: Some("Poaceae"),
        sezione: "cereali",
        illustrazione: "scheda-avena",
        infografica: Some("info-levata"),
        legale: false,
        sintesi: "Rustica e adattabile a suoli poveri e sub-acidi, ottima da erbaio, da fieno e in consociazione con la veccia. Meno resistente al freddo dell'orzo.",
        t_suolo: Some(SoglieSuolo { min: 3.0, opt: "15–20", max: None }),
        gelo: Some(-8.0),
        calendario: Some(CalendarioColturale {
            semina: &[10, 11, 12],
            semenzaio: &[],
            trapianto: &[],
            raccolta: &[6, 7],
            utilizzo: &[12, 1, 2, 3, 4, 5],
        }),
        kc: Some([0.3, 1.15, 0.25]),
        dati: &[
            ("Semina", "ottobre – dicembre"),
            ("Densità", "350–450 semi germinabili/m²"),
            ("Dose di seme", "120–160 kg/ha granella; 140–180 kg/ha erbaio"),
            ("Profondità", "3–4 cm"),
            ("Azoto totale", "60–90 kg N/ha"),
            ("P₂O₅ in presemina", "50–70 kg/ha"),
            ("pH ottimale", "5,5–7 (tollera l'acidità)"),
        ],
        tabella: None,
        blocchi: &[
            BloccoScheda {
                titolo: "Suolo e semina",
                tipo: None,
                punti: &[
                    "Si adatta a suoli poveri, sciolti o sub-acidi meglio degli altri cereali; meno indicata su suoli calcarei o salini, dove l'orzo rende di più.",
                    "Nelle zone interne e montane preferire varietà resistenti al freddo e semine di ottobre, perché le piante accestite resistano meglio alle gelate.",
                    "Semina a file distanti 15–18 cm, seguita da rullatura su suoli soffici.",
                ],
            },
            BloccoScheda {
                titolo: "Nutrizione",
                tipo: None,
                punti: &[
                    "Fabbisogno indicativo: 25–30 kg di azoto per tonnellata di granella prodotta (paglia compresa).",
                    "Sui suoli poveri dare il 20–30 % dell'azoto alla semina, il resto tra fine accestimento e inizio levata.",
                    "L'avena è molto sensibile all'allettamento: un eccesso di azoto riduce la resa e, negli erbai, aumenta il rischio di nitrati nel foraggio.",
                ],
            },
            BloccoScheda {
                titolo: "Utilizzo come erbaio",
                tipo: None,
                punti: &[
                    "Pascolamento da accestimento (20–25 cm di altezza), lasciando 5–7 cm per il ricaccio.",
                    "Per il doppio uso (pascolo + granella) sospendere il pascolo entro l'inizio della levata.",
                    "Fieno: tra inizio spigatura e fioritura, il miglior compromesso tra resa e digeribilità.",
                    "Insilato o fasciato: maturazione lattea–cerosa, con il 30–35 % di sostanza secca.",
                    "In consociazione con la veccia (80–100 + 60–80 kg/ha) il foraggio raggiunge un tenore proteico nettamente superiore all'avena in purezza.",
                ],
            },
            BloccoScheda {
                titolo: "Avversità",
                tipo: Some("attenzione"),
                punti: &[
                    "Ruggine coronata (Puccinia coronata): favorita da autunni e inverni miti e umidi. Scegliere varietà resistenti.",
                    "Virus del nanismo giallo (BYDV), trasmesso dagli afidi: il rischio cresce con semine precoci e autunni caldi.",
                    "Mal del piede e carbone: rotazione e seme conciato.",
                ],
            },
            BloccoScheda {
                titolo: "Perché la levata è decisiva",
                tipo: Some("scienza"),
                punti: &[
                    "Fino alla fine dell'accestimento l'apice vegetativo resta vicino al suolo, protetto dal morso degli animali e dalla barra falciante.",
                    "Con la levata l'apice (la futura spiga) sale: superato circa 1 cm dal suolo, un pascolamento basso lo asporta e il culmo non produce più spiga né ricaccio.",
                ],
            },
        ],
    },
    SchedaColturale {
        id: "orzo",
        nome: "Orzo",
        latino: Some("Hordeum vulgare L."),
        famiglia: Some("Poaceae"),
        sezione: "cereali",
        illustrazione: "scheda-orzo",
        infografica: None,
        legale: false,
        sintesi: "Il cereale più precoce e tollerante a siccità e salinità: completa il ciclo prima del caldo estivo. Teme acidità e ristagni.",
        t_suolo: Some(SoglieSuolo { min: 3.0, opt: "15–20", max: None }),
        gelo: Some(-10.0),
        calendario: Some(CalendarioColturale {
            semina: &[10, 11, 12],
            semenzaio: &[],
            trapianto: &[],
            raccolta: &[5, 6],
            utilizzo: &[12, 1, 2, 3, 4],
        }),
        kc: Some([0.3, 1.15, 0.25]),
        dati: &[
            ("Semina", "novembre – dicembre (erbaio da ottobre)"),
            ("Densità", "300–400 semi germinabili/m²"),
            ("Dose di seme", "150–180 kg/ha"),
            ("Profondità", "3–4 cm"),
            ("Azoto totale", "60–100 kg N/ha (malto: moderato)"),
            ("Tolleranza salinità", "soglia ECe 8 dS/m"),
            ("Raccolta granella", "maggio – giugno, umidità ≤ 13 %"),
        ],
        tabella: None,
        blocchi: &[
            BloccoScheda {
                titolo: "Suolo e semina",
                tipo: None,
                punti: &[
                    "Ideale su suoli neutri o alcalini, anche calcarei e moderatamente salini; soffre su suoli acidi (pH < 6) e con ristagno idrico durante l'accestimento.",
                    "Gli orzi polistici (a sei file) accestiscono meno dei distici: per loro usare il valore alto della densità.",
                    "Grazie al ciclo breve «sfugge» alla siccità di fine primavera: è il cereale più sicuro nelle aree con meno di 400–450 mm di pioggia.",
                ],
            },
            BloccoScheda {
                titolo: "Nutrizione",
                tipo: None,
                punti: &[
                    "Frazionare l'azoto: circa 1/3 a fine accestimento e 2/3 a inizio levata.",
                    "Per l'orzo da malto evitare eccessi: la proteina della granella deve restare tra il 9,5 e l'11,5 %.",
                    "Il fosforo va interrato in presemina (50–70 kg P₂O₅/ha sui suoli poveri).",
                ],
            },
            BloccoScheda {
                titolo: "Utilizzo come erbaio",
                tipo: None,
                punti: &[
                    "Il più precoce tra gli erbai di graminacee: prima utilizzazione 50–70 giorni dall'emergenza, buon ricaccio.",
                    "Insilato a maturazione cerosa; in miscuglio con pisello proteico (100–120 + 80–100 kg/ha) migliora il valore proteico.",
                ],
            },
            BloccoScheda {
                titolo: "Avversità",
                tipo: Some("attenzione"),
                punti: &[
                    "Elmintosporiosi (Pyrenophora teres) e rincosporiosi (Rhynchosporium): favorite da piogge frequenti e temperature miti in primavera.",
                    "Oidio con primavere miti; BYDV con semine precoci.",
                    "Gelate tardive (≤ −2 °C) in spigatura causano sterilità delle spighette.",
                ],
            },
            BloccoScheda {
                titolo: "Tolleranza alla salinità",
                tipo: Some("scienza"),
                punti: &[
                    "Secondo FAO (Ayers e Westcot) l'orzo non perde resa fino a una conducibilità dell'estratto saturo (ECe) di 8 dS/m e perde circa il 5 % per ogni dS/m oltre la soglia: è il cereale più adatto ai suoli costieri salinizzati.",
                ],
            },
        ],
    },
];
