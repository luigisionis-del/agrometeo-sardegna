//! Illustrazioni botaniche e tecniche (72×72) per tutte le 36 schede dell'Almanacco.

pub const ILLUSTRAZIONI_COLTURE: &[(&str, &str)] = &[
    // --- LAVORAZIONI DEL TERRENO ---
    (
        "scheda-aratura",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#f6eee4"/>
          <path d="M8 46c10-6 20 2 30-3s18 3 26-2v19H8V46Z" fill="#8a5a2b"/>
          <path d="M16 44c8-10 18-14 32-14-6 8-10 14-10 18H16Z" fill="#b88e61" stroke="#5d3a1a" stroke-width="2.2"/>
          <path d="M46 18v24M34 26h20" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-conservative",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <rect x="8" y="44" width="56" height="18" rx="4" fill="#8a5a2b"/>
          <path d="M12 44h48" stroke="#d9a441" stroke-width="3.5" stroke-dasharray="5 4" stroke-linecap="round"/>
          <circle cx="28" cy="36" r="10" fill="#fff" stroke="#1f8a4c" stroke-width="2.6"/>
          <circle cx="28" cy="36" r="3" fill="#1f8a4c"/>
          <path d="M48 44V22M48 30c-5 0-8-4-8-8 5 0 8 3 8 8ZM48 27c5 0 8-4 8-8-5 0-8 3-8 8Z" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-letto-semina",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdf4dc"/>
          <rect x="8" y="38" width="56" height="24" rx="4" fill="#a87948"/>
          <circle cx="22" cy="44" r="2.2" fill="#f2b01e"/>
          <circle cx="36" cy="47" r="3.2" fill="#f2b01e"/>
          <circle cx="50" cy="52" r="4.5" fill="#f2b01e"/>
          <path d="M22 38V24M36 38V20M50 38V18" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
          <path d="M12 38h48" stroke="#5d3a1a" stroke-width="2" stroke-dasharray="3 3"/>
        </svg>"##,
    ),

    // --- CEREALI ---
    (
        "scheda-avena",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdf4dc"/>
          <path d="M36 62V16" stroke="#8a5a2b" stroke-width="2.5" stroke-linecap="round"/>
          <path d="M36 22c-8 2-12 6-14 11M36 28c8 2 12 6 14 11M36 34c-8 2-12 6-14 11M36 40c8 2 12 6 14 11" stroke="#8a5a2b" stroke-width="1.8" stroke-linecap="round"/>
          <ellipse cx="21" cy="34" rx="4.5" ry="2.6" transform="rotate(-25 21 34)" fill="#f2b01e" stroke="#8a5a2b" stroke-width="1.4"/>
          <ellipse cx="51" cy="40" rx="4.5" ry="2.6" transform="rotate(25 51 40)" fill="#f2b01e" stroke="#8a5a2b" stroke-width="1.4"/>
          <ellipse cx="21" cy="46" rx="4.5" ry="2.6" transform="rotate(-25 21 46)" fill="#f2b01e" stroke="#8a5a2b" stroke-width="1.4"/>
          <ellipse cx="51" cy="52" rx="4.5" ry="2.6" transform="rotate(25 51 52)" fill="#f2b01e" stroke="#8a5a2b" stroke-width="1.4"/>
        </svg>"##,
    ),
    (
        "scheda-orzo",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdf4dc"/>
          <path d="M36 62V20" stroke="#8a5a2b" stroke-width="2.6" stroke-linecap="round"/>
          <path d="M31 22l-6-10M41 22l6-10M30 30l-7-10M42 30l7-10M36 18V8" stroke="#b07800" stroke-width="1.5" stroke-linecap="round"/>
          <g fill="#f2b01e" stroke="#8a5a2b" stroke-width="1.4">
            <ellipse cx="32" cy="26" rx="3.8" ry="6" transform="rotate(-18 32 26)"/>
            <ellipse cx="40" cy="26" rx="3.8" ry="6" transform="rotate(18 40 26)"/>
            <ellipse cx="32" cy="36" rx="3.8" ry="6" transform="rotate(-18 32 36)"/>
            <ellipse cx="40" cy="36" rx="3.8" ry="6" transform="rotate(18 40 36)"/>
            <ellipse cx="32" cy="46" rx="3.8" ry="6" transform="rotate(-18 32 46)"/>
            <ellipse cx="40" cy="46" rx="3.8" ry="6" transform="rotate(18 40 46)"/>
          </g>
        </svg>"##,
    ),

    // --- FORAGGERE ---
    (
        "scheda-trifoglio-sotterraneo",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <rect x="8" y="46" width="56" height="18" rx="4" fill="#8a5a2b"/>
          <path d="M32 46V26" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <circle cx="25" cy="24" r="7" fill="#2ea85c"/>
          <circle cx="39" cy="24" r="7" fill="#2ea85c"/>
          <circle cx="32" cy="16" r="7" fill="#1f8a4c"/>
          <path d="M32 38c10 0 16 6 18 16" stroke="#1f8a4c" stroke-width="2.2" stroke-linecap="round"/>
          <circle cx="50" cy="55" r="4.5" fill="#f2b01e" stroke="#fff" stroke-width="1.5"/>
        </svg>"##,
    ),
    (
        "scheda-trifoglio-alessandrino",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M26 58V28M46 58V28" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <ellipse cx="26" cy="22" rx="6" ry="9" fill="#fff" stroke="#1f8a4c" stroke-width="2"/>
          <ellipse cx="46" cy="22" rx="6" ry="9" fill="#d23c3c"/>
          <circle cx="20" cy="40" r="5" fill="#2ea85c"/>
          <circle cx="32" cy="40" r="5" fill="#2ea85c"/>
          <circle cx="40" cy="42" r="5" fill="#2ea85c"/>
          <circle cx="52" cy="42" r="5" fill="#2ea85c"/>
        </svg>"##,
    ),
    (
        "scheda-erba-medica",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#f1eafb"/>
          <path d="M36 62V20" stroke="#1f8a4c" stroke-width="2.6" stroke-linecap="round"/>
          <circle cx="36" cy="16" r="4.5" fill="#8e44c9"/>
          <circle cx="31" cy="21" r="4" fill="#8e44c9"/>
          <circle cx="41" cy="21" r="4" fill="#8e44c9"/>
          <ellipse cx="25" cy="36" rx="7" ry="4" transform="rotate(-25 25 36)" fill="#2ea85c"/>
          <ellipse cx="47" cy="36" rx="7" ry="4" transform="rotate(25 47 36)" fill="#2ea85c"/>
          <ellipse cx="26" cy="48" rx="7" ry="4" transform="rotate(-20 26 48)" fill="#1f8a4c"/>
          <ellipse cx="46" cy="48" rx="7" ry="4" transform="rotate(20 46 48)" fill="#1f8a4c"/>
        </svg>"##,
    ),
    (
        "scheda-loietto",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M36 60V14" stroke="#17703c" stroke-width="2.5" stroke-linecap="round"/>
          <path d="M36 56c-10-8-16-18-18-30M36 56c10-8 16-18 18-30" stroke="#2ea85c" stroke-width="2.6" stroke-linecap="round"/>
          <ellipse cx="31" cy="18" rx="3" ry="5" transform="rotate(-20 31 18)" fill="#1f8a4c"/>
          <ellipse cx="41" cy="24" rx="3" ry="5" transform="rotate(20 41 24)" fill="#1f8a4c"/>
          <ellipse cx="31" cy="30" rx="3" ry="5" transform="rotate(-20 31 30)" fill="#1f8a4c"/>
          <ellipse cx="41" cy="36" rx="3" ry="5" transform="rotate(20 41 36)" fill="#1f8a4c"/>
        </svg>"##,
    ),

    // --- OVINI ---
    (
        "scheda-calendario-foraggero",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <circle cx="36" cy="36" r="22" fill="#fff" stroke="#1f8a4c" stroke-width="3"/>
          <path d="M36 14a22 22 0 0 1 22 22H36V14Z" fill="#7ed39a" fill-opacity="0.5"/>
          <path d="M16 40a5 5 0 0 1 2-9 7 7 0 0 1 13-1 5 5 0 0 1 4 8v2H16Z" fill="#fff" stroke="#3b2a1a" stroke-width="2"/>
          <circle cx="37" cy="35" r="3.2" fill="#3b2a1a"/>
        </svg>"##,
    ),
    (
        "scheda-miscugli-erbai",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M24 58V18" stroke="#b07800" stroke-width="2.5" stroke-linecap="round"/>
          <ellipse cx="20" cy="24" rx="3.5" ry="5.5" transform="rotate(-20 20 24)" fill="#f2b01e"/>
          <ellipse cx="28" cy="30" rx="3.5" ry="5.5" transform="rotate(20 28 30)" fill="#f2b01e"/>
          <path d="M46 58V24" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <circle cx="40" cy="28" r="6" fill="#2ea85c"/>
          <circle cx="52" cy="28" r="6" fill="#2ea85c"/>
          <circle cx="46" cy="20" r="6" fill="#8e44c9"/>
        </svg>"##,
    ),
    (
        "scheda-pascolamento",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M12 52h48" stroke="#1f8a4c" stroke-width="4" stroke-linecap="round"/>
          <path d="M18 52V28M54 52V28" stroke="#8a5a2b" stroke-width="3" stroke-linecap="round"/>
          <path d="M18 34h36M18 42h36" stroke="#d4581f" stroke-width="2" stroke-dasharray="4 3"/>
          <path d="M26 26l4-5 4 5" stroke="#1f8a4c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "scheda-fieno-insilato",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdf4dc"/>
          <circle cx="28" cy="40" r="15" fill="#e8b84a" stroke="#8a5a2b" stroke-width="2.4"/>
          <circle cx="28" cy="40" r="6" fill="none" stroke="#8a5a2b" stroke-width="1.8" stroke-dasharray="3 3"/>
          <rect x="42" y="26" width="18" height="26" rx="5" fill="#d9efe0" stroke="#1f8a4c" stroke-width="2.2"/>
        </svg>"##,
    ),
    (
        "scheda-rischi-sanitari",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdeee6"/>
          <path d="M36 12 58 52H14L36 12Z" fill="#fff" stroke="#d4581f" stroke-width="2.6" stroke-linejoin="round"/>
          <path d="M36 26v12" stroke="#d23c3c" stroke-width="3" stroke-linecap="round"/>
          <circle cx="36" cy="44" r="2.2" fill="#d23c3c"/>
        </svg>"##,
    ),
    (
        "scheda-fabbisogni",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e7f0fc"/>
          <rect x="14" y="38" width="10" height="20" rx="2" fill="#9cc2f2"/>
          <rect x="31" y="28" width="10" height="30" rx="2" fill="#2a6fd0"/>
          <rect x="48" y="18" width="10" height="40" rx="2" fill="#1f8a4c"/>
        </svg>"##,
    ),

    // --- ORTICOLE ---
    (
        "scheda-principi-orto",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <circle cx="36" cy="36" r="20" stroke="#1f8a4c" stroke-width="2.6" stroke-dasharray="8 5"/>
          <circle cx="36" cy="16" r="5" fill="#d23c3c"/>
          <circle cx="56" cy="36" r="5" fill="#2ea85c"/>
          <circle cx="36" cy="56" r="5" fill="#f2b01e"/>
          <circle cx="16" cy="36" r="5" fill="#8e44c9"/>
        </svg>"##,
    ),
    (
        "scheda-pomodoro",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fbe7e7"/>
          <circle cx="36" cy="40" r="18" fill="#d23c3c"/>
          <path d="M27 34a11 11 0 0 1 7-7" stroke="#fff" stroke-width="2.2" stroke-linecap="round" opacity="0.7"/>
          <path d="M28 22c5 3 11 3 16 0M36 16v8M30 19l6 5 6-5" stroke="#1f8a4c" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "scheda-peperone",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdeee6"/>
          <path d="M25 26c-4 0-7 5-6 14 1 11 5 17 11 17h12c6 0 10-6 11-17 1-9-2-14-6-14-3 0-5 2-6 2s-3-2-5-2-3 2-5 2-3-2-6-2Z" fill="#ea7a45" stroke="#b84318" stroke-width="2.2"/>
          <path d="M36 26v-9c0-2 2-3 4-3" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-melanzana",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#f1eafb"/>
          <path d="M38 22c9 0 14 11 12 24-2 10-8 15-15 15s-12-5-11-15c1-13 6-24 14-24Z" fill="#5e2d89"/>
          <path d="M32 22l6-4 6 4M38 14v8" stroke="#1f8a4c" stroke-width="2.8" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "scheda-zucchina",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <rect x="27" y="18" width="18" height="38" rx="9" transform="rotate(25 36 37)" fill="#1f8a4c"/>
          <path d="M46 18l4-5" stroke="#17703c" stroke-width="3" stroke-linecap="round"/>
          <circle cx="26" cy="52" r="4" fill="#f2b01e"/>
        </svg>"##,
    ),
    (
        "scheda-melone",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdf4dc"/>
          <circle cx="36" cy="38" r="18" fill="#f2b01e" stroke="#b07800" stroke-width="2.2"/>
          <path d="M36 20c-6 6-6 30 0 36M36 20c6 6 6 30 0 36M18 38h36" stroke="#fff" stroke-width="1.6" opacity="0.65"/>
        </svg>"##,
    ),
    (
        "scheda-anguria",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fbe7e7"/>
          <path d="M14 28a22 22 0 0 0 44 0H14Z" fill="#d23c3c" stroke="#1f8a4c" stroke-width="4" stroke-linejoin="round"/>
          <circle cx="28" cy="36" r="1.6" fill="#17221b"/>
          <circle cx="36" cy="39" r="1.6" fill="#17221b"/>
          <circle cx="44" cy="36" r="1.6" fill="#17221b"/>
        </svg>"##,
    ),
    (
        "scheda-carciofo",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#f1eafb"/>
          <path d="M36 60V46" stroke="#1f8a4c" stroke-width="4" stroke-linecap="round"/>
          <path d="M36 16l-10 14h20L36 16Z" fill="#6c3fa8"/>
          <path d="M24 24l-6 14h16L24 24ZM48 24l6 14H38l10-14Z" fill="#1f8a4c"/>
          <path d="M22 36l-4 10h36l-4-10H22Z" fill="#2ea85c"/>
        </svg>"##,
    ),
    (
        "scheda-fava",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M20 52C18 34 32 18 52 18c2 18-12 34-32 34Z" fill="#2ea85c" stroke="#17703c" stroke-width="2.2"/>
          <circle cx="30" cy="40" r="4" fill="#a8e0b8"/>
          <circle cx="38" cy="33" r="4" fill="#a8e0b8"/>
          <circle cx="45" cy="26" r="3.5" fill="#a8e0b8"/>
        </svg>"##,
    ),
    (
        "scheda-pisello",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M18 48C20 30 34 20 54 22c-2 18-16 28-36 26Z" fill="#1f8a4c"/>
          <circle cx="28" cy="38" r="3.5" fill="#7ed39a"/>
          <circle cx="36" cy="35" r="3.5" fill="#7ed39a"/>
          <circle cx="44" cy="31" r="3.5" fill="#7ed39a"/>
        </svg>"##,
    ),
    (
        "scheda-fagiolo",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdeee6"/>
          <path d="M26 22c-8 4-10 18-2 26 7 7 20 6 24-2 3-6-2-10-6-10s-5-3-5-7-4-10-11-7Z" fill="#a8442a" stroke="#6e2816" stroke-width="2.2"/>
          <ellipse cx="36" cy="35" rx="3" ry="2" fill="#fff"/>
        </svg>"##,
    ),
    (
        "scheda-patata",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#f6eee4"/>
          <ellipse cx="36" cy="38" rx="19" ry="14" transform="rotate(-12 36 38)" fill="#d9a86c" stroke="#8a5a2b" stroke-width="2.2"/>
          <circle cx="28" cy="35" r="1.5" fill="#8a5a2b"/>
          <circle cx="42" cy="33" r="1.5" fill="#8a5a2b"/>
          <circle cx="38" cy="43" r="1.5" fill="#8a5a2b"/>
        </svg>"##,
    ),
    (
        "scheda-cipolla",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdf4dc"/>
          <path d="M36 22c-10 8-15 14-15 22a15 15 0 0 0 30 0c0-8-5-14-15-22Z" fill="#d98b3a" stroke="#8a5a2b" stroke-width="2.2"/>
          <path d="M36 22V12M32 22l-3-8M40 22l3-8" stroke="#1f8a4c" stroke-width="2.4" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-aglio",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#f6f8f5"/>
          <path d="M36 20c-11 9-16 15-16 23a16 16 0 0 0 32 0c0-8-5-14-16-23Z" fill="#fff" stroke="#8a5a2b" stroke-width="2.2"/>
          <path d="M36 22v36M29 28c-3 8-3 20 2 28M43 28c3 8 3 20-2 28" stroke="#c7d0c3" stroke-width="1.6"/>
          <path d="M36 20v-8" stroke="#1f8a4c" stroke-width="2.6" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-lattuga",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <circle cx="30" cy="34" r="12" fill="#7ed39a"/>
          <circle cx="42" cy="34" r="12" fill="#52bf78"/>
          <circle cx="36" cy="26" r="12" fill="#2ea85c"/>
          <path d="M36 32v22M30 40l6 6 6-6" stroke="#fff" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "scheda-cavoli",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M18 38c0 12 8 18 18 18s18-6 18-18" fill="#1f8a4c"/>
          <circle cx="30" cy="32" r="8" fill="#fff"/>
          <circle cx="42" cy="32" r="8" fill="#fff"/>
          <circle cx="36" cy="25" r="9" fill="#f6f8f5"/>
        </svg>"##,
    ),
    (
        "scheda-finocchio",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <ellipse cx="36" cy="44" rx="14" ry="12" fill="#fff" stroke="#1f8a4c" stroke-width="2.2"/>
          <path d="M32 32V16M36 32V14M40 32V16M28 22l-5-4M44 22l5-4" stroke="#1f8a4c" stroke-width="2.2" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-carota",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fdeee6"/>
          <path d="M26 26h20L36 60 26 26Z" fill="#ea7a45" stroke="#b84318" stroke-width="2"/>
          <path d="M36 26V14M36 24l-8-8M36 24l8-8" stroke="#1f8a4c" stroke-width="2.6" stroke-linecap="round"/>
        </svg>"##,
    ),
    (
        "scheda-spinacio",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M36 14C22 18 18 32 24 44c4 7 12 8 12 8s8-1 12-8c6-12 2-26-12-30Z" fill="#1f8a4c"/>
          <path d="M36 22v36M36 34l-7-5M36 40l7-5" stroke="#a8e0b8" stroke-width="2.2" stroke-linecap="round"/>
        </svg>"##,
    ),

    // --- CANAPA ---
    (
        "scheda-canapa-normativa",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#fbe7e7"/>
          <path d="M36 14v42M22 24h28M20 56h32" stroke="#8c1d1d" stroke-width="2.6" stroke-linecap="round"/>
          <path d="M22 24l-6 14h12l-6-14ZM50 24l-6 14h12l-6-14Z" fill="#f6eee4" stroke="#8c1d1d" stroke-width="2" stroke-linejoin="round"/>
        </svg>"##,
    ),
    (
        "scheda-canapa",
        r##"<svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="72" height="72" rx="16" fill="#e6f5ec"/>
          <path d="M36 14v46M36 38 18 22M36 38l18-16M36 38 14 38M36 38h22M36 38 22 48M36 38l14 10" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
        </svg>"##,
    ),
];
