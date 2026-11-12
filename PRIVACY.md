# Informativa sulla privacy — Agrometeo Sardegna

Ultimo aggiornamento: 12 novembre 2026.

Agrometeo Sardegna è un'applicazione che funziona sul dispositivo dell'utente. Non richiede
registrazione e non raccoglie dati personali sui server dello sviluppatore, che non ne gestisce alcuno.

## Dati conservati sul dispositivo

Località preimpostate, parametri agronomici, appezzamenti disegnati, preferenze della mappa e una copia
in cache degli ultimi dati meteo scaricati. Restano nella cartella dati dell'app
(su Linux `~/.local/share/it.agrometeo.sardegna/`) e si cancellano disinstallando l'app o eliminando
quella cartella. Nessuno di questi dati viene inviato allo sviluppatore.

## Dati inviati a servizi esterni

Per funzionare l'app contatta direttamente questi servizi, che ricevono l'indirizzo IP del dispositivo
come qualsiasi sito web:

| Servizio | Cosa riceve | Perché |
|---|---|---|
| Open-Meteo (api.open-meteo.com e sottodomini) | coordinate della località, testo delle ricerche di comuni | previsioni, storico, ricerca località |
| RainViewer (api.rainviewer.com, tilecache.rainviewer.com) | area della mappa visualizzata | radar delle precipitazioni |
| Server cartografici (Esri, OpenStreetMap, OpenTopoMap, EOX) | area della mappa visualizzata | base cartografica scelta |
| Server WMS configurato dall'utente (disattivato di default) | area della mappa visualizzata | layer radar opzionale |

Il trattamento da parte di questi servizi è regolato dalle rispettive informative.

## Cosa l'app non fa

Nessun account, nessuna pubblicità, nessuna statistica d'uso, nessun tracciamento, nessuna cessione
di dati a terzi. Il file CSV di una stazione viene elaborato solo sul dispositivo.

## Contatti

Per domande sulla privacy: apri una segnalazione nel repository del progetto su GitHub.
