//! Scarica i dati reali e stampa il report in JSON, senza avviare l'interfaccia.
//!
//! `cargo run --example diagnostica -- [lat] [lon] [germogli_cm]`

use agrometeo_core::model::Luogo;
use agrometeo_core::report::{self, Fonte, ParametriAgro, Payload};
use agrometeo_core::fetch;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let arg = |i: usize, d: f64| std::env::args().nth(i).and_then(|v| v.parse().ok()).unwrap_or(d);
    let luogo = Luogo { nome: "Diagnostica".into(), lat: arg(1, 39.79), lon: arg(2, 9.08) };
    let mut parametri = ParametriAgro { germogli_cm: arg(3, 0.0), ..Default::default() };
    let adesso = Utc::now();
    parametri.risolvi_date(report::ora_locale(adesso, 3600).date());

    let client = fetch::client();
    let previsione = fetch::previsione(&client, luogo.lat, luogo.lon).await.expect("previsione");
    let da = parametri.inizio_storico().expect("date risolte");
    let oggi = report::ora_locale(adesso, previsione.utc_offset_seconds).date();
    let storico = fetch::storico_giornaliero(&client, luogo.lat, luogo.lon, da, oggi).await.ok();
    let payload = Payload { scaricato_il: adesso, luogo, previsione, storico, storico_da: Some(da) };
    let r = report::costruisci(&payload, &parametri, adesso, Fonte::Rete, vec![]);
    let radar = fetch::radar(&client).await.ok();
    let vento = fetch::vento_griglia(&client).await.ok();
    println!("{}", serde_json::json!({ "report": r, "radar": radar, "vento": vento }));
}
