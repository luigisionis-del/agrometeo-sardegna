//! Icone vettoriali per i KPI del cruscotto, la matrice decisionale a 72 ore,
//! i banner delle sezioni dell'Almanacco e le schede dei consigli del giorno.

/// Icone per le schede KPI (48×48) e per le righe della matrice (28×28).
pub const ICONE_KPI_E_MATRICE: &[(&str, &str)] = &[
    (
        "kpi-temp",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#fdeee6"/>
          <path d="M24 10a4 4 0 0 0-4 4v14.5a7 7 0 1 0 8 0V14a4 4 0 0 0-4-4Z" fill="#fff" stroke="#d4581f" stroke-width="2.4"/>
          <circle cx="24" cy="33" r="4" fill="#d4581f"/>
          <path d="M24 18v12" stroke="#d4581f" stroke-width="2.6" stroke-linecap="round"/>
          <path d="M31 15h3M31 20h2M31 25h3" stroke="#ea7a45" stroke-width="2" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "kpi-rh",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#e7f0fc"/>
          <path d="M24 10c-6 8-11 13.5-11 19a11 11 0 1 0 22 0c0-5.5-5-11-11-19Z" fill="#9cc2f2" fill-opacity="0.45" stroke="#2a6fd0" stroke-width="2.4"/>
          <path d="M18 31a6 6 0 0 0 6 5" stroke="#2a6fd0" stroke-width="2.2" stroke-linecap="round"/>
          <circle cx="33" cy="14" r="2.5" fill="#2a6fd0" opacity="0.6"/>
        </svg>"##,
    ),
    (
        "kpi-vpd",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#e6f5ec"/>
          <path d="M13 33c0-11 9-19 22-19 0 12-8 21-20 21-1 0-2-.5-2-2Z" fill="#7ed39a" fill-opacity="0.4" stroke="#1f8a4c" stroke-width="2.4"/>
          <path d="M14 34c5-5 11-9 18-12" stroke="#1f8a4c" stroke-width="2" stroke-linecap="round"/>
          <path d="M20 14v-4m0 0-2 2m2-2 2 2M28 12V8m0 0-2 2m2-2 2 2" stroke="#2a6fd0" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "kpi-thi",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#fdf4dc"/>
          <path d="M15 28a5 5 0 0 1 2-9 6 6 0 0 1 11-1 4.5 4.5 0 0 1 4 7.5v2.5H15Z" fill="#fff" stroke="#8a5a2b" stroke-width="2.2" stroke-linejoin="round"/>
          <ellipse cx="33" cy="23" rx="4.5" ry="3.2" fill="#3b2a1a"/>
          <circle cx="34.2" cy="22.3" r="0.8" fill="#fff"/>
          <path d="M18 28v7M22 28v7M27 28v7M30 28v7" stroke="#3b2a1a" stroke-width="2.2" stroke-linecap="round"/>
          <circle cx="14" cy="14" r="3.5" fill="#f2b01e"/>
        </svg>"##,
    ),
    (
        "kpi-pioggia",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#e7f0fc"/>
          <path d="M15 26a5.5 5.5 0 0 1 1-10.9A7.5 7.5 0 0 1 30.5 14a5 5 0 0 1 2.5 9.3H15Z" fill="#fff" stroke="#2a6fd0" stroke-width="2.2"/>
          <path d="M18 30l-2 6M24 30l-2 6M30 30l-2 6" stroke="#2a6fd0" stroke-width="2.4" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "kpi-vento",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#f0f4f1"/>
          <path d="M10 19h21a3.5 3.5 0 1 0-3.5-3.5" stroke="#3f4c44" stroke-width="2.4" stroke-linecap="round"/>
          <path d="M10 25h26a3.5 3.5 0 1 1-3.5 3.5" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
          <path d="M12 31h14a3 3 0 1 1-3 3" stroke="#66736b" stroke-width="2.2" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "kpi-et0",
        r##"<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="48" height="48" rx="12" fill="#fdf4dc"/>
          <circle cx="24" cy="17" r="5.5" fill="#f2b01e" stroke="#b07800" stroke-width="2"/>
          <path d="M11 35c4-2 8-2 13 0s9 2 13 0" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
          <path d="M17 30v-6m0 0-2 2m2-2 2 2M24 29v-5m0 0-2 2m2-2 2 2M31 30v-6m0 0-2 2m2-2 2 2" stroke="#2a6fd0" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    // Icone righe matrice (28x28)
    (
        "mat-trattamenti",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="4" y="10" width="11" height="12" rx="2.5" fill="#e3f3e8" stroke="#1f8a4c" stroke-width="1.8"/>
          <path d="M15 15h4l4-4M19 15l4 4M21 15h3" stroke="#2a6fd0" stroke-width="1.8" stroke-linecap="round"/>
          <path d="M7 10V7h5v3" stroke="#1f8a4c" stroke-width="1.8"/>
        </svg>"##,
    ),
    (
        "mat-bagnatura",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M6 21C6 12 13 6 22 6c0 9-6 15-16 15Z" fill="#e3f3e8" stroke="#1f8a4c" stroke-width="1.8"/>
          <path d="M18 14c-1.8 2.5-3 4.2-3 5.8a3 3 0 1 0 6 0c0-1.6-1.2-3.3-3-5.8Z" fill="#2a6fd0"/>
        </svg>"##,
    ),
    (
        "mat-peronospora",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M6 20C6 11 13 6 22 6c0 9-6 15-16 15Z" fill="#fdf4dc" stroke="#8a5a2b" stroke-width="1.8"/>
          <circle cx="13" cy="14" r="2.2" fill="#d4581f"/>
          <circle cx="17" cy="11" r="1.5" fill="#f2b01e"/>
          <text x="15" y="25" font-size="7.5" font-weight="800" fill="#8a5a2b">3×10</text>
        </svg>"##,
    ),
    (
        "mat-secondaria",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="10" cy="11" r="3" fill="#fdeee6" stroke="#d23c3c" stroke-width="1.7"/>
          <circle cx="18" cy="10" r="2.5" fill="#fdeee6" stroke="#d23c3c" stroke-width="1.7"/>
          <circle cx="15" cy="18" r="3.2" fill="#fdeee6" stroke="#d23c3c" stroke-width="1.7"/>
          <path d="M12.5 12.5 14 15.5M16.5 12 15.5 15" stroke="#d23c3c" stroke-width="1.5"/>
        </svg>"##,
    ),
    (
        "mat-oidio",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="11" cy="10" r="3" fill="#e9dff5" stroke="#6c3fa8" stroke-width="1.6"/>
          <circle cx="17" cy="10" r="3" fill="#e9dff5" stroke="#6c3fa8" stroke-width="1.6"/>
          <circle cx="14" cy="15" r="3" fill="#e9dff5" stroke="#6c3fa8" stroke-width="1.6"/>
          <circle cx="14" cy="20" r="2.6" fill="#e9dff5" stroke="#6c3fa8" stroke-width="1.6"/>
          <path d="M14 4v3" stroke="#1f8a4c" stroke-width="1.8" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "mat-thi",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M6 18a4 4 0 0 1 2-7.5A5 5 0 0 1 17 10a3.5 3.5 0 0 1 3 6v2H6Z" fill="#fff" stroke="#8a5a2b" stroke-width="1.7"/>
          <circle cx="21" cy="14" r="2.8" fill="#3b2a1a"/>
          <path d="M9 18v5M13 18v5M17 18v5" stroke="#3b2a1a" stroke-width="1.7" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "mat-gelata",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M14 4v20M5.3 9l17.4 10M22.7 9 5.3 19" stroke="#2a6fd0" stroke-width="2" stroke-linecap="round"/>
          <circle cx="14" cy="14" r="2.5" fill="#e7f0fc" stroke="#2a6fd0" stroke-width="1.6"/>
        </svg>"##,
    ),
    (
        "mat-vpd",
        r##"<svg viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <ellipse cx="14" cy="14" rx="9" ry="6" fill="#e3f3e8" stroke="#1f8a4c" stroke-width="1.8"/>
          <ellipse cx="14" cy="14" rx="4.5" ry="2.2" fill="#fff" stroke="#17703c" stroke-width="1.6"/>
        </svg>"##,
    ),
];

/// Illustrazioni panoramiche per le 8 sezioni dell'Almanacco (180×110).
pub const ILLUSTRAZIONI_SEZIONI: &[(&str, &str)] = &[
    (
        "sez-oggi",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#e6f4ea"/>
          <circle cx="142" cy="30" r="16" fill="#f5b83d"/>
          <circle cx="142" cy="30" r="22" fill="#f5b83d" fill-opacity="0.22"/>
          <path d="M35 32a10 10 0 0 1 9-6 13 13 0 0 1 24 2 8 8 0 0 1 2 15.8H36A8 8 0 0 1 35 32Z" fill="#fff" fill-opacity="0.9"/>
          <path d="M44 48l-3 7M54 48l-3 7M64 48l-3 7" stroke="#2a6fd0" stroke-width="2" stroke-linecap="round"/>
          <path d="M0 82C38 64 78 68 118 78s44 8 62-2v34H0V82Z" fill="#68b678"/>
          <path d="M0 92c48-10 95-4 180-12v30H0V92Z" fill="#2e7d47"/>
          <path d="M0 100h180v10H0z" fill="#8a5a2b"/>
          <g transform="translate(22,60)">
            <ellipse cx="14" cy="14" rx="10" ry="6.5" fill="#fff" stroke="#3b2a1a" stroke-width="1.6"/>
            <circle cx="24" cy="11" r="3.5" fill="#3b2a1a"/>
            <path d="M8 20v6M13 20v6M18 20v6M21 20v6" stroke="#3b2a1a" stroke-width="1.8" stroke-linecap="round"/>
          </g>
          <g transform="translate(95,52)">
            <path d="M12 28V10M12 16c-5-1-8-5-8-9 5 0 8 3 8 9ZM12 13c5-1 8-5 8-9-5 0-8 3-8 9Z" stroke="#17703c" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/>
          </g>
        </svg>"##,
    ),
    (
        "sez-suolo",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#f6eee4"/>
          <rect x="0" y="42" width="180" height="20" fill="#d9be9b"/>
          <rect x="0" y="62" width="180" height="24" fill="#b88e61"/>
          <rect x="0" y="86" width="180" height="24" rx="0" fill="#8a5a2b"/>
          <path d="M12 42c10-6 20 0 30-4s20 2 30-3 20 3 30-2 25 2 40-3 22 4 38 0" stroke="#5d3a1a" stroke-width="2.5" fill="none"/>
          <rect x="26" y="20" width="34" height="16" rx="4" fill="#1f8a4c"/>
          <rect x="44" y="10" width="14" height="12" rx="2" fill="#e3f3e8" stroke="#1f8a4c" stroke-width="2"/>
          <circle cx="33" cy="37" r="8" fill="#2d3830" stroke="#fff" stroke-width="2"/>
          <circle cx="55" cy="39" r="6" fill="#2d3830" stroke="#fff" stroke-width="2"/>
          <path d="M26 32L14 55M20 32L10 50" stroke="#5d3a1a" stroke-width="3" stroke-linecap="round"/>
          <circle cx="110" cy="52" r="3" fill="#e7f0fc"/>
          <circle cx="128" cy="56" r="2.5" fill="#e7f0fc"/>
          <circle cx="145" cy="50" r="3.2" fill="#e7f0fc"/>
          <path d="M122 42V22M122 30c-5 0-8-4-8-8 5 0 8 3 8 8ZM122 27c5 0 8-4 8-8-5 0-8 3-8 8Z" stroke="#1f8a4c" stroke-width="2.2" stroke-linecap="round"/>
          <path d="M122 42c-2 10-7 18-12 28M122 42c2 12 6 22 10 32M122 42v36" stroke="#f6eee4" stroke-width="1.8" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "sez-cereali",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#fdf4dc"/>
          <path d="M0 90c50-10 110-8 180 0v20H0V90Z" fill="#d9a441"/>
          <g stroke="#8a5a2b" stroke-width="2.2" stroke-linecap="round">
            <path d="M48 95V22"/>
            <path d="M90 95V16"/>
            <path d="M132 95V24"/>
          </g>
          <g fill="#f2b01e" stroke="#8a5a2b" stroke-width="1.4">
            <ellipse cx="42" cy="30" rx="5" ry="3" transform="rotate(-30 42 30)"/>
            <ellipse cx="54" cy="34" rx="5" ry="3" transform="rotate(30 54 34)"/>
            <ellipse cx="42" cy="42" rx="5" ry="3" transform="rotate(-30 42 42)"/>
            <ellipse cx="54" cy="46" rx="5" ry="3" transform="rotate(30 54 46)"/>
            <ellipse cx="85" cy="24" rx="4.5" ry="7" transform="rotate(-20 85 24)"/>
            <ellipse cx="95" cy="24" rx="4.5" ry="7" transform="rotate(20 95 24)"/>
            <ellipse cx="85" cy="36" rx="4.5" ry="7" transform="rotate(-20 85 36)"/>
            <ellipse cx="95" cy="36" rx="4.5" ry="7" transform="rotate(20 95 36)"/>
            <ellipse cx="85" cy="48" rx="4.5" ry="7" transform="rotate(-20 85 48)"/>
            <ellipse cx="95" cy="48" rx="4.5" ry="7" transform="rotate(20 95 48)"/>
            <ellipse cx="126" cy="32" rx="5" ry="3" transform="rotate(-30 126 32)"/>
            <ellipse cx="138" cy="36" rx="5" ry="3" transform="rotate(30 138 36)"/>
            <ellipse cx="126" cy="44" rx="5" ry="3" transform="rotate(-30 126 44)"/>
            <ellipse cx="138" cy="48" rx="5" ry="3" transform="rotate(30 138 48)"/>
          </g>
        </svg>"##,
    ),
    (
        "sez-foraggere",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#e6f5ec"/>
          <rect x="0" y="76" width="180" height="34" fill="#8a5a2b"/>
          <path d="M55 76V42M120 76V36" stroke="#1f8a4c" stroke-width="2.6" stroke-linecap="round"/>
          <circle cx="46" cy="38" r="9" fill="#2ea85c"/>
          <circle cx="64" cy="38" r="9" fill="#2ea85c"/>
          <circle cx="55" cy="27" r="9" fill="#1f8a4c"/>
          <circle cx="120" cy="26" r="8" fill="#8e44c9"/>
          <circle cx="110" cy="48" r="7" fill="#2ea85c"/>
          <circle cx="130" cy="48" r="7" fill="#2ea85c"/>
          <path d="M55 76v24M55 84l-12 10M55 86l14 10M120 76v28M120 86l-10 12M120 88l12 10" stroke="#e9efe7" stroke-width="2" stroke-linecap="round"/>
          <circle cx="46" cy="91" r="3" fill="#f2b01e"/>
          <circle cx="64" cy="93" r="3" fill="#f2b01e"/>
          <circle cx="113" cy="94" r="3" fill="#f2b01e"/>
          <circle cx="128" cy="94" r="3" fill="#f2b01e"/>
        </svg>"##,
    ),
    (
        "sez-ovini",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#e8f4ec"/>
          <path d="M0 74c50-14 110-10 180 2v34H0V74Z" fill="#4ca965"/>
          <g transform="translate(36,34)">
            <path d="M12 30a10 10 0 0 1 4-19 12 12 0 0 1 22-2 9 9 0 0 1 8 15v6H12Z" fill="#fff" stroke="#3b2a1a" stroke-width="2.2"/>
            <ellipse cx="48" cy="19" rx="8" ry="5.5" fill="#2b2118"/>
            <circle cx="50" cy="18" r="1.3" fill="#fff"/>
            <path d="M18 30v12M25 30v12M36 30v12M42 30v12" stroke="#2b2118" stroke-width="2.6" stroke-linecap="round"/>
          </g>
          <g transform="translate(118,46)">
            <circle cx="18" cy="16" r="14" fill="#e8b84a" stroke="#8a5a2b" stroke-width="2"/>
            <circle cx="18" cy="16" r="6" fill="none" stroke="#8a5a2b" stroke-width="1.6" stroke-dasharray="3 3"/>
          </g>
        </svg>"##,
    ),
    (
        "sez-orticole",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#fdeee6"/>
          <rect x="0" y="84" width="180" height="26" fill="#7a5230"/>
          <path d="M12 84h156" stroke="#2a6fd0" stroke-width="3" stroke-dasharray="6 6"/>
          <circle cx="48" cy="54" r="16" fill="#d23c3c"/>
          <path d="M42 38c4 2 8 2 12 0M48 34v7" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
          <path d="M95 36c8 0 14 10 12 24-2 11-8 18-15 18s-11-7-10-18c1-14 6-24 13-24Z" fill="#6c3fa8"/>
          <path d="M94 32v7" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
          <path d="M138 42l-7 36h14l-7-36Z" fill="#ea7a45"/>
          <path d="M138 42c-4-6-8-8-10-8m10 8V30m0 12c4-6 8-8 10-8" stroke="#1f8a4c" stroke-width="2.2" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "sez-canapa",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#e3f3e8"/>
          <rect x="0" y="84" width="180" height="26" fill="#8a5a2b"/>
          <path d="M90 16v88" stroke="#17703c" stroke-width="3" stroke-linecap="round"/>
          <path d="M90 52 62 32M90 52l28-20M90 52 56 50M90 52l34-2M90 52 68 66M90 52l22 14" stroke="#1f8a4c" stroke-width="3.2" stroke-linecap="round"/>
          <path d="M90 84l-14 16M90 90l14 12" stroke="#f6eee4" stroke-width="2" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "sez-fonti",
        r##"<svg viewBox="0 0 180 110" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="180" height="110" rx="14" fill="#e7f0fc"/>
          <rect x="18" y="24" width="42" height="58" rx="6" fill="#fff" stroke="#2a6fd0" stroke-width="2.2"/>
          <path d="M26 38h26M26 48h26M26 58h18" stroke="#2a6fd0" stroke-width="2.2" stroke-linecap="round"/>
          <path d="M68 53h24m0 0-6-6m6 6-6 6" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"/>
          <rect x="102" y="24" width="58" height="58" rx="10" fill="#fff" stroke="#1f8a4c" stroke-width="2.2"/>
          <path d="M114 66l10-14 8 6 14-20" stroke="#1f8a4c" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
          <circle cx="146" cy="38" r="3.5" fill="#1f8a4c"/>
        </svg>"##,
    ),
];

/// Illustrazioni per i 9 consigli operativi dinamici in «Oggi in campo» (64×64).
pub const ILLUSTRAZIONI_CONSIGLI: &[(&str, &str)] = &[
    (
        "cons-semina",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#e6f5ec"/>
          <path d="M10 44h44v10a4 4 0 0 1-4 4H14a4 4 0 0 1-4-4V44Z" fill="#8a5a2b"/>
          <circle cx="32" cy="48" r="4" fill="#f2b01e"/>
          <path d="M32 44V20M32 30c-8 0-13-6-13-12 8 0 13 5 13 12ZM32 26c8 0 13-6 13-12-8 0-13 5-13 12Z" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "cons-lavorazioni",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#f6eee4"/>
          <path d="M8 44c8-4 16 2 24-2s16 2 24-2v12H8V44Z" fill="#8a5a2b"/>
          <rect x="18" y="22" width="26" height="14" rx="3" fill="#1f8a4c"/>
          <rect x="32" y="13" width="12" height="10" rx="2" fill="#fff" stroke="#1f8a4c" stroke-width="2"/>
          <circle cx="24" cy="38" r="7" fill="#2d3830" stroke="#fff" stroke-width="2"/>
          <circle cx="41" cy="40" r="5" fill="#2d3830" stroke="#fff" stroke-width="2"/>
        </svg>"##,
    ),
    (
        "cons-gelate",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#e7f0fc"/>
          <path d="M32 12v40M15 22l34 20M49 22 15 42" stroke="#2a6fd0" stroke-width="3" stroke-linecap="round"/>
          <circle cx="32" cy="32" r="5" fill="#fff" stroke="#2a6fd0" stroke-width="2.5"/>
        </svg>"##,
    ),
    (
        "cons-acqua",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#e7f0fc"/>
          <path d="M32 12c-8 11-14 18-14 26a14 14 0 1 0 28 0c0-8-6-15-14-26Z" fill="#9cc2f2" fill-opacity="0.5" stroke="#2a6fd0" stroke-width="2.8"/>
          <path d="M25 40a7 7 0 0 0 7 6" stroke="#2a6fd0" stroke-width="2.4" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "cons-azoto",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#fdf4dc"/>
          <circle cx="22" cy="24" r="4" fill="#2a6fd0"/>
          <circle cx="34" cy="20" r="4" fill="#1f8a4c"/>
          <circle cx="42" cy="28" r="4" fill="#2a6fd0"/>
          <circle cx="28" cy="34" r="4" fill="#1f8a4c"/>
          <path d="M12 46h40" stroke="#8a5a2b" stroke-width="4" stroke-linecap="round"/>
          <text x="20" y="57" font-size="11" font-weight="800" fill="#8a5a2b">N · P · K</text>
        </svg>"##,
    ),
    (
        "cons-erbai",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#e6f5ec"/>
          <path d="M18 50V24M26 50V18M34 50V22M44 50V26" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
          <circle cx="44" cy="22" r="5" fill="#2ea85c"/>
          <path d="M12 50h40" stroke="#8a5a2b" stroke-width="3.5" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "cons-ovini",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#fdf4dc"/>
          <path d="M14 38a7 7 0 0 1 3-13 9 9 0 0 1 17-1 6.5 6.5 0 0 1 6 11v3H14Z" fill="#fff" stroke="#8a5a2b" stroke-width="2.4"/>
          <ellipse cx="43" cy="30" rx="6" ry="4.2" fill="#3b2a1a"/>
          <circle cx="44.5" cy="29" r="1" fill="#fff"/>
          <path d="M19 38v10M25 38v10M33 38v10M38 38v10" stroke="#3b2a1a" stroke-width="2.4" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "cons-peronospora",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#fdeee6"/>
          <path d="M16 46c0-18 12-30 32-30 0 18-10 32-28 32-2 0-4-.8-4-2Z" fill="#7ed39a" fill-opacity="0.35" stroke="#1f8a4c" stroke-width="2.6"/>
          <circle cx="30" cy="30" r="4.5" fill="#d4581f"/>
          <circle cx="38" cy="24" r="3" fill="#f2b01e"/>
        </svg>"##,
    ),
    (
        "cons-fieno",
        r##"<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#fdf4dc"/>
          <circle cx="32" cy="34" r="16" fill="#e8b84a" stroke="#8a5a2b" stroke-width="2.6"/>
          <circle cx="32" cy="34" r="7" fill="none" stroke="#8a5a2b" stroke-width="2" stroke-dasharray="4 3"/>
          <circle cx="50" cy="16" r="5" fill="#f2b01e"/>
        </svg>"##,
    ),
];
