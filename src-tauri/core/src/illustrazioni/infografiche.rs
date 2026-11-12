//! Infografiche tecniche esplicative (SVG panoramici) per l'Almanacco,
//! il Cruscotto, il Bilancio Idrico e la Calibrazione Stazione.

pub const INFOGRAFICHE: &[(&str, &str)] = &[
    (
        "info-tempera",
        r##"<svg viewBox="0 0 680 185" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="185" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">STATO DI TEMPERA E RISCHIO DI SUOLA DI LAVORAZIONE (STRATO 9–27 CM)</text>
          <!-- Pannello 1: Secco -->
          <g transform="translate(16,36)">
            <rect width="204" height="134" rx="9" fill="#fdf4dc" stroke="#f2b01e"/>
            <text x="12" y="22" font-size="12" font-weight="700" fill="#7a5600">1. SUOLO SECCO (&lt; 40% AD)</text>
            <rect x="12" y="34" width="180" height="58" rx="6" fill="#d9be9b"/>
            <path d="M35 34l12 24-10 34M90 34l-14 28 12 30M150 34l8 30-14 28" stroke="#5d3a1a" stroke-width="2.5"/>
            <text x="12" y="108" font-size="10.5" fill="#3f4c44">• Aratura: zolle grandi e alti consumi</text>
            <text x="12" y="123" font-size="10.5" fill="#146c3a" font-weight="600">✓ Ottimo invece per la ripuntatura</text>
          </g>
          <!-- Pannello 2: Tempera -->
          <g transform="translate(238,36)">
            <rect width="204" height="134" rx="9" fill="#e6f5ec" stroke="#1f9d55" stroke-width="1.5"/>
            <text x="12" y="22" font-size="12" font-weight="700" fill="#146c3a">2. IN TEMPERA (40–100% AD)</text>
            <rect x="12" y="34" width="180" height="58" rx="6" fill="#b88e61"/>
            <circle cx="38" cy="52" r="5" fill="#8a5a2b"/><circle cx="58" cy="64" r="6" fill="#8a5a2b"/><circle cx="84" cy="50" r="5" fill="#8a5a2b"/>
            <circle cx="112" cy="66" r="5.5" fill="#8a5a2b"/><circle cx="142" cy="54" r="6" fill="#8a5a2b"/><circle cx="166" cy="68" r="4.5" fill="#8a5a2b"/>
            <circle cx="48" cy="74" r="3" fill="#e7f0fc"/><circle cx="98" cy="58" r="3" fill="#e7f0fc"/><circle cx="152" cy="72" r="3" fill="#e7f0fc"/>
            <text x="12" y="108" font-size="10.5" fill="#146c3a" font-weight="600">✓ Zolla si sbriciola, versoio pulito</text>
            <text x="12" y="123" font-size="10.5" fill="#3f4c44">• Macropori per aria e radici intatti</text>
          </g>
          <!-- Pannello 3: Eccesso -->
          <g transform="translate(460,36)">
            <rect width="204" height="134" rx="9" fill="#fbe7e7" stroke="#d23c3c"/>
            <text x="12" y="22" font-size="12" font-weight="700" fill="#9c2020">3. ECCESSO (&gt; CAP. CAMPO)</text>
            <rect x="12" y="34" width="180" height="58" rx="6" fill="#7a5838"/>
            <rect x="12" y="64" width="180" height="10" fill="#3b2a1a"/>
            <text x="24" y="72" font-size="8.5" font-weight="700" fill="#fff">SUOLA DI LAVORAZIONE COMPATTATA</text>
            <path d="M102 34v28m0 0-8-5m8 5 8-5" stroke="#fdeee6" stroke-width="2" stroke-linecap="round"/>
            <text x="12" y="108" font-size="10.5" fill="#9c2020" font-weight="600">✕ Asfissia radicale e ristagno</text>
            <text x="12" y="123" font-size="10.5" fill="#3f4c44">• Vietato entrare con le macchine</text>
          </g>
        </svg>"##,
    ),
    (
        "info-profondita-seme",
        r##"<svg viewBox="0 0 680 165" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="165" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">REGOLA DELLA PROFONDITÀ DI SEMINA (2–3 VOLTE IL DIAMETRO DEL SEME)</text>
          <rect x="18" y="42" width="644" height="106" rx="8" fill="#d9be9b"/>
          <rect x="18" y="76" width="644" height="72" rx="0" fill="#b88e61"/>
          <line x1="18" y1="42" x2="662" y2="42" stroke="#5d3a1a" stroke-width="2.5"/>
          <text x="28" y="58" font-size="10.5" font-weight="700" fill="#5d3a1a">0–1,5 cm (letto fine + rullatura)</text>
          <text x="28" y="96" font-size="10.5" font-weight="700" fill="#3b2a1a">3–4 cm (cereali)</text>
          <text x="28" y="134" font-size="10.5" font-weight="700" fill="#3b2a1a">4–6 cm (leguminose a seme grosso)</text>
          <!-- Seme piccolo -->
          <circle cx="235" cy="54" r="3" fill="#f2b01e" stroke="#3b2a1a"/>
          <circle cx="250" cy="55" r="3" fill="#f2b01e" stroke="#3b2a1a"/>
          <text x="268" y="58" font-size="11" font-weight="600" fill="#17221b">Medica, Trifogli, Loietto, Carota (0,5–1,5 cm)</text>
          <!-- Seme medio -->
          <ellipse cx="345" cy="92" rx="5" ry="3" fill="#f2b01e" stroke="#3b2a1a"/>
          <text x="362" y="96" font-size="11" font-weight="600" fill="#17221b">Avena, Orzo, Frumento (3–4 cm)</text>
          <!-- Seme grosso -->
          <ellipse cx="430" cy="130" rx="7.5" ry="5" fill="#a8442a" stroke="#fff" stroke-width="1.2"/>
          <text x="450" y="134" font-size="11" font-weight="600" fill="#17221b">Fava, Pisello, Fagiolo (3–6 cm)</text>
        </svg>"##,
    ),
    (
        "info-levata",
        r##"<svg viewBox="0 0 680 175" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="175" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">FISIOLOGIA DEL PASCOLAMENTO DEI CEREALI: ACCESTIMENTO VS LEVATA</text>
          <g transform="translate(18,36)">
            <rect width="312" height="124" rx="9" fill="#e6f5ec" stroke="#1f9d55"/>
            <text x="14" y="22" font-size="12" font-weight="700" fill="#146c3a">FASE DI ACCESTIMENTO (PASCOLABILE)</text>
            <line x1="14" y1="98" x2="298" y2="98" stroke="#8a5a2b" stroke-width="3"/>
            <line x1="14" y1="68" x2="298" y2="68" stroke="#1f8a4c" stroke-width="1.5" stroke-dasharray="4 4"/>
            <text x="175" y="63" font-size="10" font-weight="600" fill="#146c3a">Soglia uscita: 5–7 cm</text>
            <path d="M75 98V36M75 98L55 44M75 98l20-54" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
            <circle cx="75" cy="94" r="4.5" fill="#d23c3c"/>
            <text x="98" y="92" font-size="10.5" font-weight="700" fill="#d23c3c">← Apice al suolo (protetto)</text>
            <text x="14" y="116" font-size="10.5" fill="#3f4c44">Il morso non tocca l'apice: ottimo ricaccio</text>
          </g>
          <g transform="translate(350,36)">
            <rect width="312" height="124" rx="9" fill="#fbe7e7" stroke="#d23c3c"/>
            <text x="14" y="22" font-size="12" font-weight="700" fill="#9c2020">INIZIO LEVATA (SOSPENDERE PASCOLO)</text>
            <line x1="14" y1="98" x2="298" y2="98" stroke="#8a5a2b" stroke-width="3"/>
            <line x1="14" y1="68" x2="298" y2="68" stroke="#d23c3c" stroke-width="1.5" stroke-dasharray="4 4"/>
            <path d="M75 98V28M75 76L55 34M75 76l20-42" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
            <circle cx="75" cy="58" r="5" fill="#d23c3c"/>
            <text x="95" y="61" font-size="10.5" font-weight="700" fill="#9c2020">← Apice salito (&gt; 1 cm)</text>
            <text x="14" y="116" font-size="10.5" fill="#9c2020" font-weight="600">✕ Il pascolo asporta la futura spiga</text>
          </g>
        </svg>"##,
    ),
    (
        "info-geocarpia",
        r##"<svg viewBox="0 0 680 165" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="165" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">CICLO DI AUTORISEMINA DEL TRIFOGLIO SOTTERRANEO (GEOCARPIA)</text>
          <rect x="18" y="104" width="644" height="46" rx="6" fill="#8a5a2b"/>
          <g transform="translate(36,42)">
            <text x="0" y="12" font-size="11" font-weight="700" fill="#146c3a">1. Autunno–Inverno</text>
            <text x="0" y="27" font-size="10" fill="#3f4c44">Emergenza e pascolo</text>
            <path d="M45 62V36" stroke="#1f8a4c" stroke-width="2.5"/><circle cx="40" cy="34" r="5" fill="#2ea85c"/><circle cx="50" cy="34" r="5" fill="#2ea85c"/>
          </g>
          <g transform="translate(205,42)">
            <text x="0" y="12" font-size="11" font-weight="700" fill="#146c3a">2. Primavera</text>
            <text x="0" y="27" font-size="10" fill="#3f4c44">Fioritura e interramento</text>
            <path d="M40 62V36c18 0 26 14 28 38" stroke="#1f8a4c" stroke-width="2.4" fill="none"/>
            <circle cx="68" cy="76" r="5" fill="#f2b01e" stroke="#fff"/>
          </g>
          <g transform="translate(385,42)">
            <text x="0" y="12" font-size="11" font-weight="700" fill="#7a5600">3. Estate secca</text>
            <text x="0" y="27" font-size="10" fill="#3f4c44">Glomeruli protetti nel suolo</text>
            <circle cx="35" cy="80" r="5" fill="#f2b01e"/><circle cx="52" cy="84" r="5" fill="#f2b01e"/><circle cx="69" cy="79" r="5" fill="#f2b01e"/>
          </g>
          <g transform="translate(545,42)">
            <text x="0" y="12" font-size="11" font-weight="700" fill="#146c3a">4. Prime piogge</text>
            <text x="0" y="27" font-size="10" fill="#3f4c44">Nuova nascita spontanea</text>
            <circle cx="45" cy="80" r="4" fill="#f2b01e"/>
            <path d="M45 76V42" stroke="#2ea85c" stroke-width="2.5"/><circle cx="45" cy="38" r="5" fill="#2ea85c"/>
          </g>
        </svg>"##,
    ),
    (
        "info-tre-foglie",
        r##"<svg viewBox="0 0 680 170" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="170" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">LA REGOLA DELLE 3 FOGLIE NEL LOIETTO (FULKERSON &amp; DONAGHY)</text>
          <g transform="translate(18,36)">
            <rect width="204" height="118" rx="9" fill="#fdf4dc" stroke="#f2b01e"/>
            <text x="12" y="20" font-size="11.5" font-weight="700" fill="#7a5600">1 FOGLIA (TROPPO PRESTO)</text>
            <path d="M102 90V52l18-12" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
            <text x="12" y="106" font-size="10" fill="#7a5600">Riserve di fruttani esaurite</text>
          </g>
          <g transform="translate(238,36)">
            <rect width="204" height="118" rx="9" fill="#e6f5ec" stroke="#1f9d55" stroke-width="1.5"/>
            <text x="12" y="20" font-size="11.5" font-weight="700" fill="#146c3a">2,5–3 FOGLIE (OTTIMALE)</text>
            <path d="M102 90V36M102 68 78 50M102 54l24-16M102 42 82 28" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
            <text x="12" y="106" font-size="10" font-weight="600" fill="#146c3a">✓ Ingresso 15–20 cm · Uscita 5–7 cm</text>
          </g>
          <g transform="translate(458,36)">
            <rect width="204" height="118" rx="9" fill="#fdeee6" stroke="#ea7a45"/>
            <text x="12" y="20" font-size="11.5" font-weight="700" fill="#9c2020">&gt; 3 FOGLIE (TROPPO TARDI)</text>
            <path d="M102 90V32M102 54l24-16M102 42 82 28" stroke="#1f8a4c" stroke-width="3" stroke-linecap="round"/>
            <path d="M102 74 76 84" stroke="#b07800" stroke-width="3" stroke-linecap="round"/>
            <text x="12" y="106" font-size="10" fill="#9c2020">La 1ª foglia ingiallisce e muore</text>
          </g>
        </svg>"##,
    ),
    (
        "info-ciclo-pecora",
        r##"<svg viewBox="0 0 680 180" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="180" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">CURVA FORAGGERA NATURALE VS FABBISOGNO DELLA PECORA SARDA</text>
          <line x1="40" y1="142" x2="648" y2="142" stroke="#c7d0c3" stroke-width="1.5"/>
          <text x="58" y="160" font-size="10.5" font-weight="700" fill="#3f4c44">AUTUNNO (Ott–Dic)</text>
          <text x="218" y="160" font-size="10.5" font-weight="700" fill="#3f4c44">INVERNO (Gen–Feb)</text>
          <text x="378" y="160" font-size="10.5" font-weight="700" fill="#3f4c44">PRIMAVERA (Mar–Mag)</text>
          <text x="548" y="160" font-size="10.5" font-weight="700" fill="#3f4c44">ESTATE (Giu–Set)</text>
          <!-- Curva pascolo naturale (verde) -->
          <path d="M48 128 C 120 120, 220 118, 300 105 C 370 50, 440 42, 500 85 C 550 130, 600 136, 640 136" stroke="#1f8a4c" stroke-width="3.5" fill="none"/>
          <!-- Curva fabbisogno pecora (arancio tratteggiato) -->
          <path d="M48 64 C 130 48, 220 58, 320 72 C 410 78, 490 95, 640 118" stroke="#d4581f" stroke-width="3" stroke-dasharray="6 4" fill="none"/>
          <rect x="52" y="72" width="142" height="36" rx="6" fill="#fdeee6" stroke="#ea7a45"/>
          <text x="60" y="88" font-size="10" font-weight="700" fill="#9c2020">DEFICIT AUTUNNale</text>
          <text x="60" y="101" font-size="9.5" fill="#3f4c44">Colmare con erbai precoci</text>
          <rect x="364" y="82" width="136" height="36" rx="6" fill="#e6f5ec" stroke="#1f9d55"/>
          <text x="372" y="98" font-size="10" font-weight="700" fill="#146c3a">SURPLUS PRIMAVERILE</text>
          <text x="372" y="111" font-size="9.5" fill="#3f4c44">Fieno e balle fasciate</text>
          <text x="480" y="45" font-size="10.5" font-weight="700" fill="#1f8a4c">— Pascolo naturale</text>
          <text x="480" y="60" font-size="10.5" font-weight="700" fill="#d4581f">-- Fabbisogno gregge</text>
        </svg>"##,
    ),
    (
        "info-thi-ovini",
        r##"<svg viewBox="0 0 680 135" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="135" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">SCALA DI STRESS TERMICO (THI NRC) PER OVINI E CAPRINI AL PASCOLO</text>
          <g transform="translate(18,38)">
            <rect x="0" y="0" width="156" height="78" rx="8" fill="#e6f5ec" stroke="#1f9d55"/>
            <text x="12" y="24" font-size="13" font-weight="800" fill="#146c3a">THI &lt; 68 · VERDE</text>
            <text x="12" y="44" font-size="10.5" fill="#17221b" font-weight="600">Benessere termico</text>
            <text x="12" y="60" font-size="10" fill="#3f4c44">Ingestione e latte regolari</text>
          </g>
          <g transform="translate(182,38)">
            <rect x="0" y="0" width="156" height="78" rx="8" fill="#fdf4dc" stroke="#f2b01e"/>
            <text x="12" y="24" font-size="13" font-weight="800" fill="#7a5600">68–72 · GIALLO</text>
            <text x="12" y="44" font-size="10.5" fill="#17221b" font-weight="600">Disagio lieve</text>
            <text x="12" y="60" font-size="10" fill="#3f4c44">Garantire ombra e acqua</text>
          </g>
          <g transform="translate(346,38)">
            <rect x="0" y="0" width="156" height="78" rx="8" fill="#fdeee6" stroke="#ea7a45"/>
            <text x="12" y="24" font-size="13" font-weight="800" fill="#9c2020">72–78 · ARANCIO</text>
            <text x="12" y="44" font-size="10.5" fill="#17221b" font-weight="600">Stress moderato</text>
            <text x="12" y="60" font-size="10" fill="#3f4c44">Pascolo alba/sera, calo latte</text>
          </g>
          <g transform="translate(510,38)">
            <rect x="0" y="0" width="152" height="78" rx="8" fill="#fbe7e7" stroke="#d23c3c"/>
            <text x="12" y="24" font-size="13" font-weight="800" fill="#9c2020">THI ≥ 78 · ROSSO</text>
            <text x="12" y="44" font-size="10.5" fill="#17221b" font-weight="600">Stress severo</text>
            <text x="12" y="60" font-size="10" fill="#3f4c44">Pascolo notturno e ventilazione</text>
          </g>
        </svg>"##,
    ),
    (
        "info-rotazione-orto",
        r##"<svg viewBox="0 0 680 155" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="155" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">SCHEMA DI AVVICENDAMENTO QUADRIENNALE PER FAMIGLIE ORTICOLE</text>
          <g transform="translate(18,38)">
            <rect width="148" height="98" rx="8" fill="#e6f5ec" stroke="#1f9d55"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#146c3a">ANNO 1 · FABACEE</text>
            <text x="12" y="42" font-size="10.5" fill="#17221b">Fava, Pisello, Fagiolo</text>
            <text x="12" y="62" font-size="10" fill="#3f4c44">Arricchiscono il suolo</text>
            <text x="12" y="76" font-size="10" fill="#146c3a" font-weight="600">+30–60 kg N/ha</text>
          </g>
          <path d="M170 87h12" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <g transform="translate(186,38)">
            <rect width="148" height="98" rx="8" fill="#fbe7e7" stroke="#d23c3c"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#9c2020">ANNO 2 · SOLANACEE</text>
            <text x="12" y="42" font-size="10.5" fill="#17221b">Pomodoro, Patata, Pep.</text>
            <text x="12" y="62" font-size="10" fill="#3f4c44">Forti consumatrici:</text>
            <text x="12" y="76" font-size="10" fill="#3f4c44">sfruttano la fertilità</text>
          </g>
          <path d="M338 87h12" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <g transform="translate(354,38)">
            <rect width="148" height="98" rx="8" fill="#fdf4dc" stroke="#f2b01e"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#7a5600">ANNO 3 · BULBI/FOGLIA</text>
            <text x="12" y="42" font-size="10.5" fill="#17221b">Cipolla, Aglio, Lattuga</text>
            <text x="12" y="62" font-size="10" fill="#3f4c44">Apparato superficiale,</text>
            <text x="12" y="76" font-size="10" fill="#3f4c44">interrompono i patogeni</text>
          </g>
          <path d="M506 87h12" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <g transform="translate(522,38)">
            <rect width="140" height="98" rx="8" fill="#e7f0fc" stroke="#2a6fd0"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#1d4f99">ANNO 4 · RADICI/CUC.</text>
            <text x="12" y="42" font-size="10.5" fill="#17221b">Carota, Finocchio, Zucc.</text>
            <text x="12" y="62" font-size="10" fill="#3f4c44">Chiudono il ciclo prima</text>
            <text x="12" y="76" font-size="10" fill="#3f4c44">del ritorno alle Fabacee</text>
          </g>
        </svg>"##,
    ),
    (
        "info-bilancio-idrico",
        r##"<svg viewBox="0 0 680 148" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="148" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">SCHEMA DEL BILANCIO IDRICO COLTURALE FAO-56 (PENMAN-MONTEITH)</text>
          <g transform="translate(18,38)">
            <rect width="146" height="94" rx="8" fill="#e7f0fc" stroke="#2a6fd0"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#1d4f99">+ PIOGGIA EFFICACE</text>
            <text x="12" y="42" font-size="10" fill="#17221b">Acqua meteorica che</text>
            <text x="12" y="56" font-size="10" fill="#17221b">infiltra nella zona</text>
            <text x="12" y="70" font-size="10" fill="#3f4c44">radicale utile (mm)</text>
          </g>
          <text x="172" y="90" font-size="20" font-weight="800" fill="#3f4c44">−</text>
          <g transform="translate(192,38)">
            <rect width="168" height="94" rx="8" fill="#fdf4dc" stroke="#f2b01e"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#7a5600">ETc = ET0 × Kc</text>
            <text x="12" y="42" font-size="10" fill="#17221b">Domanda atmosferica (ET0)</text>
            <text x="12" y="56" font-size="10" fill="#17221b">× coefficiente colturale</text>
            <text x="12" y="72" font-size="9.5" font-weight="600" fill="#7a5600">Kc ini · Kc med · Kc fin</text>
          </g>
          <text x="368" y="90" font-size="20" font-weight="800" fill="#3f4c44">=</text>
          <g transform="translate(388,38)">
            <rect width="274" height="94" rx="8" fill="#e6f5ec" stroke="#1f9d55"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#146c3a">SALDO IDRICO SETTIMANALE (mm)</text>
            <text x="12" y="42" font-size="10" fill="#146c3a" font-weight="600">• Saldo ≥ 0 mm: riserva suolo ricaricata</text>
            <text x="12" y="58" font-size="10" fill="#7a5600" font-weight="600">• −15..0 mm: intacca l'acqua facilmente disp.</text>
            <text x="12" y="74" font-size="10" fill="#9c2020" font-weight="600">• &lt; −15 mm: deficit → restituire con adacquata</text>
          </g>
        </svg>"##,
    ),
    (
        "info-calibrazione",
        r##"<svg viewBox="0 0 680 148" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="680" height="148" rx="12" fill="#f6f8f5" stroke="#dfe5dc"/>
          <text x="18" y="24" font-size="13" font-weight="700" fill="#17221b">COME FUNZIONA LA CALIBRAZIONE LOCALE CON DATI STAZIONE (ARPAS / PLUVIOMETRO)</text>
          <g transform="translate(18,38)">
            <rect width="194" height="94" rx="8" fill="#e7f0fc" stroke="#2a6fd0"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#1d4f99">1. MISURA IN CAMPO (CSV)</text>
            <text x="12" y="42" font-size="10" fill="#17221b">Pluviometro aziendale o</text>
            <text x="12" y="56" font-size="10" fill="#17221b">stazione ARPAS Sardegna</text>
            <text x="12" y="74" font-size="9.5" fill="#1d4f99" font-weight="600">data, pioggia_mm, tmin, tmax</text>
          </g>
          <path d="M218 85h18" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <g transform="translate(242,38)">
            <rect width="194" height="94" rx="8" fill="#fdf4dc" stroke="#f2b01e"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#7a5600">2. CONFRONTO GRIGLIA</text>
            <text x="12" y="42" font-size="10" fill="#17221b">Incrocio automatico giorno</text>
            <text x="12" y="56" font-size="10" fill="#17221b">per giorno col modello</text>
            <text x="12" y="74" font-size="9.5" fill="#7a5600" font-weight="600">Scarto = Modello − Stazione</text>
          </g>
          <path d="M442 85h18" stroke="#1f8a4c" stroke-width="2.5" stroke-linecap="round"/>
          <g transform="translate(466,38)">
            <rect width="196" height="94" rx="8" fill="#e6f5ec" stroke="#1f9d55"/>
            <text x="12" y="22" font-size="11.5" font-weight="700" fill="#146c3a">3. INDICI DI AFFIDABILITÀ</text>
            <text x="12" y="42" font-size="10" fill="#17221b">• Bias pioggia e temp. (scarto)</text>
            <text x="12" y="56" font-size="10" fill="#17221b">• MAE (errore medio assoluto)</text>
            <text x="12" y="72" font-size="10" fill="#146c3a" font-weight="600">• Concordanza secco/pioggia %</text>
          </g>
        </svg>"##,
    ),
];
