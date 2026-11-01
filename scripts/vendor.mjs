// Copia le librerie frontend in dist/vendor: l'app funziona offline senza CDN.
import { cpSync, mkdirSync } from "node:fs";

const copie = [
  ["node_modules/leaflet/dist/leaflet.js", "dist/vendor/leaflet/leaflet.js"],
  ["node_modules/leaflet/dist/leaflet.css", "dist/vendor/leaflet/leaflet.css"],
  ["node_modules/leaflet/dist/images", "dist/vendor/leaflet/images"],
  ["node_modules/chart.js/dist/chart.umd.js", "dist/vendor/chart.umd.js"],
];
mkdirSync("dist/vendor/leaflet", { recursive: true });
for (const [da, a] of copie) cpSync(da, a, { recursive: true });
console.log("Librerie copiate in dist/vendor");
