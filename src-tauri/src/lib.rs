use tauri::{WebviewUrl, WebviewWindowBuilder};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let material_css = r#"
                /* =========================================
                                GLOBAL VARIABLES
                   ========================================= */
                :root {
                    --titlebar-height: 32px;
                    --gap-size: 8px;
                    --middle-gap: 2px;
                    --card-radius: 18px; 
                    
                    /* Material 3 Dark Palette */
                    --md-sys-color-surface: #1E1F22;
                    --md-sys-color-on-surface: #E6E1E5;
                    --md-sys-color-on-surface-variant: #CAC4D0;
                    --md-sys-color-primary: #D0BCFF;
                    --md-sys-color-outline: #938F99;
                    
                    /* State Layers */
                    --md-hover-layer: rgba(255, 255, 255, 0.08);
                    --md-active-layer: rgba(255, 255, 255, 0.12);
                    
                    --window-bg: #141414;
                }

                html, body {
                    width: 100% !important;
                    height: 100% !important;
                    overflow: hidden !important;
                    margin: 0 !important;
                    padding: 0 !important;
                    background-color: var(--window-bg) !important;
                    font-family: 'Roboto', 'Segoe UI', sans-serif !important;
                }

                div[id^="mount_"] {
                    position: fixed !important;
                    top: var(--titlebar-height) !important;
                    left: 0 !important;
                    right: 0 !important;
                    height: calc(100vh - var(--titlebar-height)) !important;
                    width: 100% !important;
                    z-index: 1;
                    background-color: var(--window-bg) !important; 
                }

                /* =========================================
                              DUAL FLOATING CARDS
                   ========================================= */

                div[role="navigation"],
                div[role="main"] {
                    height: calc(100% - (var(--gap-size) * 2)) !important;
                    margin-top: var(--gap-size) !important;
                    margin-bottom: var(--gap-size) !important;
                    
                    clip-path: inset(0 0 0 0 round var(--card-radius)) !important;
                    -webkit-clip-path: inset(0 0 0 0 round var(--card-radius)) !important;
                    
                    box-shadow: 0 4px 8px rgba(0,0,0,0.4) !important; 
                    background-color: transparent !important;
                    border: none !important;
                    contain: layout paint style !important;
                    transform: translateZ(0) !important; 
                }

                /* --- LEFT CARD (Sidebar) --- */
                div[role="navigation"] {
                    margin-left: var(--gap-size) !important;
                    margin-right: calc(var(--middle-gap) / 2) !important; 
                    padding-right: 4px !important; 
                }

                /* --- RIGHT CARD (Chat View) --- */
                div[role="main"] {
                    margin-left: -11px !important; 
                    margin-right: var(--gap-size) !important;
                    padding: 0 !important;
                }

                /* --- RESPONSIVE FIX (Single Column Mode) --- */
                @media (max-width: 707px) { /* For some reason messenger triggers single column at 707px */
                    div[role="main"] {
                        margin-left: calc(var(--gap-size) * -1) !important;
                    }
                }

                /* FORCE FILL */
                div[role="main"] > div,
                div[role="main"] > div > div,
                div[role="main"] > div > div > div {
                    width: 100% !important;
                    height: 100% !important;
                    min-height: 100% !important;
                    max-height: 100% !important;
                    margin: 0 !important;
                    padding: 0 !important;
                    border-radius: 0 !important; 
                }

                /* =========================================
                                BLOAT REMOVAL & FIXES
                   ========================================= */
                div[role="navigation"][aria-label="Przełącznik skrzynki odbiorczej"],
                div[role="navigation"][aria-label="Inbox switch"],
                a[href="https://www.facebook.com/"],
                div[role="banner"] { display: none !important; }

                /* Settings Button Logic */
                div[role="banner"] { 
                    display: block !important; 
                    position: fixed !important;
                    top: 40px !important; 
                    left: 10px !important;
                    width: 40px !important;
                    height: 40px !important;
                    opacity: 0 !important; 
                    z-index: 0 !important; 
                    pointer-events: none !important; 
                }
                div[role="banner"] div[role="button"] {
                    pointer-events: auto !important;
                }

                div:has(> div[role="navigation"]),
                div:has(> div[role="main"]) {
                    padding: 0 !important;
                    margin: 0 !important;
                    background-color: transparent !important;
                }
                div:has(> div[role="navigation"][aria-label="Przełącznik skrzynki odbiorczej"]) {
                    padding-left: 0px !important;
                    display: flex !important; 
                }

                /* =========================================
                                  TITLE BAR
                   ========================================= */
                #custom-titlebar {
                    position: fixed; top: 0; left: 0; width: 100%;
                    height: var(--titlebar-height);
                    background: var(--window-bg);
                    display: flex; justify-content: space-between; align-items: center;
                    z-index: 9999999; user-select: none;
                }

                .titlebar-drag-region {
                    flex-grow: 1; height: 100%; display: flex; align-items: center;
                    padding-left: 20px; 
                    font-family: 'Roboto', sans-serif;
                    font-size: 14px; 
                    font-weight: 500;
                    letter-spacing: 0.1px;
                    color: var(--md-sys-color-on-surface);
                }
                
                .app-icon {
                    font-size: 18px;
                    margin-right: 12px;
                    filter: grayscale(100%);
                    opacity: 0.8;
                }

                .titlebar-controls { 
                    display: flex; 
                    height: 100%; 
                    padding-right: 12px;
                    align-items: center;
                    gap: 6px;
                }

                .titlebar-button {
                    width: 32px; 
                    height: 32px; 
                    border-radius: 50%;
                    display: flex; 
                    justify-content: center; 
                    align-items: center;
                    color: var(--md-sys-color-on-surface-variant);
                    cursor: default;
                    transition: background-color 0.15s cubic-bezier(0.4, 0, 0.2, 1), transform 0.1s ease;
                }

                .titlebar-button:hover { 
                    background-color: var(--md-hover-layer); 
                    color: var(--md-sys-color-on-surface);
                }
                
                .titlebar-button:active {
                    background-color: var(--md-active-layer);
                    transform: scale(0.95);
                }

                .titlebar-button#titlebar-close:hover { 
                    background-color: #B3261E; 
                    color: #FFFFFF;
                }

                .titlebar-icon { 
                    width: 18px; 
                    height: 18px; 
                    fill: currentColor; 
                }

                /* =========================================
                               SCROLLBARS & UI
                   ========================================= */
                *::-webkit-scrollbar {
                    width: 6px !important;
                    background: transparent !important;
                }
                *::-webkit-scrollbar-thumb {
                    background-color: rgba(255, 255, 255, 0.15) !important;
                    border-radius: 3px !important;
                }
                *::-webkit-scrollbar-thumb:hover {
                    background-color: rgba(255, 255, 255, 0.3) !important;
                }

                /* =========================================
                            MATERIAL UI OVERRIDES
                   ========================================= */
                
                input[type="search"], input[aria-label="Szukaj w Messengerze"] {
                    border-radius: 50px !important;
                    background-color: var(--messenger-card-background) !important;
                    color: var(--primary-text) !important;
                    text-align: center;
                    transition: all 0.2s ease;
                }
                input[type="search"]:focus {
                    background-color: #555 !important;
                    text-align: left !important;
                    padding-left: 20px !important;
                }

                div[role="navigation"] div[role="row"] div[role="none"][style*="inset"] {
                    display: none !important;
                }

                /* Reset Background of ALL children so they don't bleed out */
                div[role="navigation"] div[role="row"] a,
                div[role="navigation"] div[role="row"] a:hover,
                div[role="navigation"] div[role="row"] a:active,
                div[role="navigation"] div[role="row"] a:focus,
                div[role="navigation"] div[role="row"] div[role="presentation"] {
                    background-color: transparent !important;
                    outline: none !important;
                }

                /* Apply Everything to the Parent Container */
                div[role="navigation"] div[role="row"] {
                    border-radius: 24px !important;
                    margin: 2px 4px !important;
                    overflow: hidden !important;
                    position: relative !important;
                    transition: background-color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1) !important;
                }

                /* Custom Hover State */
                div[role="navigation"] div[role="row"]:hover {
                    background-color: var(--md-hover-layer) !important;
                    transform: scale(1.01) !important;
                    z-index: 10 !important;
                }

                /* Custom Active Click */
                div[role="navigation"] div[role="row"]:active,
                div[role="navigation"] div[role="row"]:has(a:active) {
                    background-color: var(--md-active-layer) !important;
                    transform: scale(0.98) !important;
                }

                /* Active State (Purple Pill) */
                div[role="navigation"] div[role="row"]:has(a[aria-current="page"]) {
                    background-color: #4A4458 !important;
                }
                /* Re-apply text color to the link since we nuked its styles */
                div[role="navigation"] div[role="row"]:has(a[aria-current="page"]) a {
                    color: #E8DEF8 !important;
                    transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                }

                /* --- CHAT BUBBLES --- */
                div[role="main"] div[role="row"] {
                    border-radius: 0 !important;
                    overflow: visible !important;
                    margin: 0 !important;
                }

                div[role="main"] div[dir="auto"] {
                     border-radius: 18px !important;
                }
                
                /* --- THREE DOTS BUTTON --- */
                div[role="navigation"] div[role="gridcell"] div[role="button"] {
                    background-color: rgba(255,255,255,0.1) !important; 
                    border-radius: 50% !important;
                }
                div[role="navigation"] div[role="gridcell"] div[role="button"]:hover {
                    background-color: rgba(255,255,255,0.2) !important;
                }

                /* =========================================
                                ANIMATIONS
                   ========================================= */
                @keyframes fadeScaleIn {
                    from { opacity: 0; transform: scale(0.8); }
                    to { opacity: 1; transform: scale(1); }
                }

                @keyframes fadeScaleOut {
                    from { opacity: 1; transform: scale(1); }
                    to { opacity: 0; transform: scale(0.8); }
                }

                /* Prevent layout shift by making the arrow container not affect layout */
                div.x11lfxj5:has(> div[role="button"][aria-label="Zamknij tryb wpisywania"]),
                div.x11lfxj5:has(> div[role="button"][aria-label="Close typing mode"]) {
                    width: 0 !important;
                    overflow: visible !important;
                    flex-shrink: 0 !important;
                }

                div[role="button"][aria-label="Zamknij tryb wpisywania"],
                div[role="button"][aria-label="Close typing mode"] {
                    position: relative !important;
                    left: -12px !important;
                    animation: fadeScaleIn 0.4s cubic-bezier(0.05, 0.7, 0.1, 1.0) forwards !important;
                }

                /* Shift entire search container (input + icon) when arrow is visible */
                div.x11lfxj5:has(> div[role="button"][aria-label="Zamknij tryb wpisywania"]) ~ label,
                div.x11lfxj5:has(> div[role="button"][aria-label="Close typing mode"]) ~ label,
                div.x11lfxj5:has(> div[role="button"][aria-label="Zamknij tryb wpisywania"]) ~ div:has(input[type="search"]),
                div.x11lfxj5:has(> div[role="button"][aria-label="Close typing mode"]) ~ div:has(input[type="search"]) {
                    margin-left: 26px !important;
                    transition: margin-left 0.3s cubic-bezier(0.05, 0.7, 0.1, 1.0) !important;
                }

                /* Default state for search container - animate back when arrow disappears */
                label:has(input[type="search"]),
                div:has(> input[type="search"]) {
                    margin-left: 0 !important;
                    transition: margin-left 0.3s cubic-bezier(0.05, 0.7, 0.1, 1.0) !important;
                }
            "#;

            let titlebar_html = r#"
                <div id="custom-titlebar">
                    <div class="titlebar-drag-region" data-tauri-drag-region>
                        <span class="app-icon">💬</span> Messterial
                    </div>
                    <div class="titlebar-button" id="titlebar-settings" title="Settings">
                        <svg class="titlebar-icon" viewBox="0 0 24 24">
                            <path d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z"/>
                        </svg>
                    </div>
                    <div class="titlebar-controls">
                        <div class="titlebar-button" id="titlebar-minimize" title="Minimize">
                            <svg class="titlebar-icon" viewBox="0 0 24 24">
                                <path d="M5 19h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1s.45 1 1 1z"/>
                            </svg>
                        </div>
                        <div class="titlebar-button" id="titlebar-maximize" title="Maximize">
                            <svg class="titlebar-icon" viewBox="0 0 24 24">
                                <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/>
                            </svg>
                        </div>
                        <div class="titlebar-button" id="titlebar-close" title="Close">
                            <svg class="titlebar-icon" viewBox="0 0 24 24">
                                <path d="M18.3 5.71a.9959.9959 0 0 0-1.41 0L12 10.59 7.11 5.7a.9959.9959 0 0 0-1.41 0c-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/>
                            </svg>
                        </div>
                    </div>
                </div>
            "#;

            // =========================================================================
            //                            LAYOUT MANAGER
            // =========================================================================
            let init_script = format!(
                "
                window.addEventListener('DOMContentLoaded', () => {{ 
                    const style = document.createElement('style');
                    style.innerHTML = `{css}`;
                    document.head.append(style);

                    document.body.insertAdjacentHTML('afterbegin', `{html}`);

                    const initWindowControls = () => {{
                        if (!window.__TAURI__) return;
                        const appWindow = window.__TAURI__.window.getCurrentWindow();
                        document.getElementById('titlebar-minimize').addEventListener('click', () => appWindow.minimize());
                        document.getElementById('titlebar-maximize').addEventListener('click', () => appWindow.toggleMaximize());
                        document.getElementById('titlebar-close').addEventListener('click', () => appWindow.close());

                        document.getElementById('titlebar-settings').addEventListener('click', () => {{
                            const selectors = [
                                'div[role=\"banner\"] div[role=\"button\"]', 
                                'div[role=\"button\"][aria-label=\"Ustawienia użytkownika\"]',
                                'div[role=\"button\"][aria-label=\"Account settings\"]',
                                'div[role=\"button\"][aria-label*=\"Profil\"]'
                            ];
                                    
                            for (const selector of selectors) {{
                                const btn = document.querySelector(selector);
                                if (btn) {{
                                    btn.click();
                                    return;
                                }}
                            }}
                            console.error(\"Could not find settings button.\");
                        }});
                    }};
                    
                    let tauriInterval = setInterval(() => {{
                        if (window.__TAURI__) {{
                            clearInterval(tauriInterval);
                            initWindowControls();
                        }}
                    }}, 100);
                }});
                ", 
                css = material_css,
                html = titlebar_html
            );

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://www.messenger.com/login".parse().unwrap())
            )
            .title("Messterial")
            .inner_size(1200.0, 800.0)
            .decorations(false)
            .initialization_script(&init_script)
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}