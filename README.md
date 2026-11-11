# Agrometeo Sardegna

Decision Support System agrometeorologico per vigneto, cerealicolo/foraggero, orticoltura e zootecnia in Sardegna,
con almanacco colturale collegato ai dati meteo e del suolo.
Rust + Tauri 2, solo Open Data, **nessuna API key**.

## Avvio

```sh
npm install            # CLI Tauri + Leaflet/Chart.js
npm run vendor         # copia Leaflet e Chart.js in dist/vendor (già fatto, serve solo dopo un aggiornamento)
npm run dev            # app in sviluppo
npm run build          # pacchetti AppImage + deb in src-tauri/target/release/bundle
npm test               # test unitari Rust
cd src-tauri && cargo test -- --ignored                       # test sulle API reali
cd src-tauri && cargo run --example diagnostica -- 39.79 9.08 15   # report JSON senza interfaccia
```

### APK Android

```sh
export JAVA_HOME=~/.cache/agrometeo-android/jdk17          # JDK 17 (Gradle non supporta Java 27)
export ANDROID_HOME=~/.cache/fmis-android/sdk NDK_HOME=$ANDROID_HOME/ndk/29.0.14206865
export GRADLE_USER_HOME=~/.cache/agrometeo-android/gradle
npm run android:apk    # → src-tauri/gen/android/app/build/outputs/apk/universal/release/
```

Per i box TV a 32 bit (es. TIM Box): `npm run android:apk:tv` (target `armv7`). Su un dispositivo Android
senza schermo tattile l'app entra da sola in modalità telecomando (`dist/js/tv.js`): frecce per spostare
il fuoco, OK per attivare o aprire la tastiera su un campo, Indietro per chiudere e tornare alle schede.
Se il box non ha un launcher, l'app si propone come schermata Home (`MainActivity.kt`) e smette di farlo
quando ne viene installato uno.

Firma: `~/.config/agrometeo/keystore.properties` (o `AGROMETEO_KEYSTORE_PROPERTIES`), mai nel repository.
Senza quel file l'APK esce non firmato. Su Android il client HTTP verifica i certificati con le radici
Mozilla incluse (`webpki-root-certs`): il verificatore di piattaforma richiederebbe un'inizializzazione JNI.

## Architettura

```
dist/                      Frontend statico leggero: solo rendering DOM/SVG/Chart.js/Leaflet, tema chiaro
  js/api.js                wrapper dei comandi Tauri
  js/illustrazioni.js      rendering sicuro delle icone, illustrazioni e infografiche SVG dal catalogo Rust
  js/navigazione.js        schede principali: Cruscotto, Previsioni e bilancio, Almanacco, Calibrazione
  js/almanacco.js          rendering dell'Almanacco colturale, infografiche, termometro suolo e calendari
  js/testata.js            località, ricerca comuni/coordinate, KPI con icone SVG, età del dato
  js/suolo.js              sezione stratigrafica SVG degli apparati radicali, termometri e barre idriche
  js/mappa.js              radar RainViewer, WMS opzionale, vento, appezzamenti GeoJSON
  js/matrice.js            matrice a semaforo 72 h con icone e regole agronomiche, sintesi allerte
  js/grafici.js            Chart.js: suolo, bilancio idrico, temperature, radiazione, GDD
  js/impostazioni.js       parametri agronomici e layer
  js/stazioni.js           confronto con stazione (CSV o dataset dimostrativo ARPAS 30 gg)
src-tauri/src/             Tutta la logica di dominio, i calcoli, il catalogo e le illustrazioni in Rust
  fetch.rs                 Data ingestion: Open-Meteo (previsione, storico, geocoding, vento), RainViewer
  model.rs                 struct dei payload e normalizzazione (lacune interpolate)
  agro/                    Motore agrometeorologico: funzioni pure con test
    wetness.rs             3.1 bagnatura fogliare
    disease.rs             3.2 peronospora (3 dieci, Goidanich semplificato), oidio
    spray.rs               3.3 finestra trattamenti
    gdd.rs                 3.4 gradi giorno
    water.rs               3.5 bilancio idrico, classi VPD
    thi.rs                 3.6 THI ovini/caprini
    frost_soil.rs          3.7 gelate, lavorabilità e semina
  almanacco/               Catalogo colturale e motore di analisi dinamica in Rust
    mod.rs                 tipi condivisi, assemblaggio catalogo e test d'integrità
    sezioni.rs             8 sezioni tematiche, mesi, bibliografia scientifica e avvertenze
    suolo_cereali.rs       lavorazioni del terreno (aratura, conservative, letto di semina), avena, orzo
    foraggere_ovini.rs     trifogli, medica, loietto e gestione foraggera per la pecora Sarda
    orticole_canapa.rs     principi orticoli, 18 orticole da pieno campo e canapa industriale
    analisi.rs             motore puro: tempera 3–9/9–27 cm, gelate, THI, periodo di Smith, 9 consigli live
  illustrazioni/           Catalogo vettoriale (SVG) descrittivo ed esplicativo validato dai test Rust
    mod.rs                 mappa illustrazioni e test di copertura di tutte le schede, sezioni e KPI
    icone_kpi_matrice.rs   icone KPI, righe matrice 72 h, illustrazioni sezioni e consigli del giorno
    colture.rs             36 illustrazioni botaniche e tecniche per le schede colturali
    infografiche.rs        10 infografiche scientifiche (tempera, levata, geocarpia, 3 foglie, pecora Sarda,
                           THI, rotazione orto, profondità seme, bilancio FAO-56, calibrazione stazione)
  report.rs                assemblaggio deterministico del report (inclusa l'analisi dell'Almanacco)
  stations.rs              importatore CSV stazioni, generatore esempio ARPAS e statistiche
  storage.rs               cache offline e spazio di lavoro (JSON su disco, scrittura atomica)
  lib.rs                   comandi Tauri (`report`, `catalogo_almanacco`, `csv_esempio_stazione`, ecc.)
```

## Almanacco colturale

Sezioni: Oggi in campo · Lavorazioni del terreno · Cereali · Foraggere · Erbai e foraggi per la pecora Sarda ·
Orticole · Canapa · Metodo e fonti. «Oggi in campo» incrocia il report con le soglie delle schede
(T suolo 6 cm vs minima di semina/trapianto, minima a 72 h vs soglia di danno da gelo delle colture in campo
nel mese, pioggia passata/prevista, THI, periodo di Smith). Ogni scheda mostra indicatori in tempo reale
(epoca, suolo idoneo, rischio gelo). I valori sono orientativi: le avvertenze sono nella sezione «Metodo e fonti».

Dati salvati in `~/.local/share/it.agrometeo.sardegna/`: `cache/previsione_<lat>_<lon>.json`
(ultimo payload valido per località) e `spazio_lavoro.json` (preset, parametri, appezzamenti, layer).
Senza rete il report viene ricalcolato dalla cache e la testata mostra il badge «Offline · dati di …».
Con cache più recente di 15 minuti la rete non viene interrogata.

## Fonti

| Livello | Fonte | Uso |
|---|---|---|
| 1 | `api.open-meteo.com/v1/forecast` | 25 variabili orarie + 6 giornaliere, 7 giorni passati + 7 di previsione |
| 1 | `historical-forecast-api.open-meteo.com` | Tmax/Tmin per GDD stagionali, pioggia per il confronto con le stazioni |
| 1 | `geocoding-api.open-meteo.com` | ricerca comuni (filtrata sulla Sardegna) |
| 2 | RainViewer `weather-maps.json` | radar ultime 2 h, step 10 min |
| 2 | WMS configurabile | predisposto per mosaico DPC VMI/SRI o ARPAS (disattivato di default) |
| 3 | CSV stazione | pioggia osservata vs modello (bias, MAE, concordanza giorni piovosi) |
| Base | Esri Light Gray / Dark Gray, OSM, OpenTopoMap, Sentinel-2 cloudless (EOX) | cartografia |

## Scelte e scostamenti dalla specifica

- **RainViewer**: l'API pubblica oggi non fornisce più il nowcast (lista vuota) e serve tile solo fino
  allo zoom 7, con un solo schema colori. Il player mostra le ultime 2 h; se il nowcast tornasse viene
  usato automaticamente. Oltre lo zoom 7 Leaflet ingrandisce i tile del 7.
- **WMS Protezione Civile**: l'endpoint non era raggiungibile durante lo sviluppo; URL e layer sono
  modificabili in «Parametri».
- **Base cartografica**: CARTO ora richiede una chiave; la base predefinita è Esri Light Gray (tema chiaro).
  Gli spazi di lavoro salvati con la vecchia base scura passano una volta sola alla chiara
  (`layer.versione_tema`); la base scura resta selezionabile nei Parametri.
- **Livelli intermedi non definiti dalla specifica** (documentati nel codice):
  - peronospora primaria gialla quando, con germogli > 10 cm, è soddisfatta 1 sola delle 2 condizioni meteo;
  - peronospora secondaria gialla con LWD ≥ 6 h e T 10–28 °C, oppure LWD ≥ 3 h e T 18–24 °C;
  - oidio giallo con T 15–30 °C e RH > 45 %; «piogge dilavanti» = pioggia nelle 24 h precedenti ≥ 2 mm;
    «forte insolazione» = radiazione ≥ 600 W/m²;
  - VPD 1.2–1.6 kPa = «Elevato» (arancio);
  - gelate: rosso anche con T ≤ 0 °C, giallo con T ≤ 3 °C senza le condizioni di irraggiamento.
- **Germogli**: la regola dei 3 dieci richiede la lunghezza dei germogli, impostata dall'utente.
  Con 0 cm (vite a riposo) i modelli di vite sono sospesi (grigio «n.a.»).
- **Pioggia efficace**: 0 sotto 2 mm, poi 80 % (parametri modificabili).
- **Tempera del suolo**: tra il 40 % dell'acqua disponibile e la capacità di campo; preset per tessitura.
- **Grafici**: niente doppi assi; radiazione e temperatura sono grafici separati, la PAR è nel tooltip.
- **Sentinel-2 NDVI/NDWI**: è disponibile la base Sentinel-2 cloudless; per NDVI servono servizi
  con registrazione, quindi gli appezzamenti GeoJSON (disegno, import, copia) sono già pronti ad agganciarli.
