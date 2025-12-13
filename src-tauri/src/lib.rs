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
                    
                    --primary-text: #E6E1E5 !important; 
                    --secondary-text: #CDC4CE !important;
                    --messenger-card-background: #49454F !important;
                    --window-bg: #2b2930; 
                }

                html, body {
                    width: 100% !important;
                    height: 100% !important;
                    overflow: hidden !important;
                    margin: 0 !important;
                    padding: 0 !important;
                    background-color: var(--window-bg) !important;
                    font-family: 'Segoe UI', 'Roboto', sans-serif !important;
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
                    
                    box-shadow: 0 4px 12px rgba(0,0,0,0.2) !important; 
                    background-color: transparent !important;
                    border: none !important;
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

                /* FORCE FILL: Ensure chat content fills the rounded card */
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
                                BLOAT REMOVAL
                   ========================================= */
                div[role="navigation"][aria-label="Przełącznik skrzynki odbiorczej"],
                div[role="navigation"][aria-label="Inbox switch"],
                div[role="banner"] { display: none !important; }

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
                    padding-left: 16px; font-size: 13px; color: #E6E1E5; font-weight: 500;
                }
                .titlebar-controls { display: flex; height: 100%; }
                .titlebar-button {
                    width: 46px; height: 100%; display: flex; justify-content: center; align-items: center;
                    color: #E6E1E5; cursor: default;
                }
                .titlebar-button:hover { background: #49454F; }
                .titlebar-button#titlebar-close:hover { background: #B3261E; }
                .titlebar-icon { width: 10px; height: 10px; fill: currentColor; }

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

                input[type="search"], input[aria-label="Szukaj w Messengerze"] {
                    border-radius: 50px !important;
                    background-color: var(--messenger-card-background) !important;
                    color: var(--primary-text) !important;
                    text-align: center;
                }
            "#;

            // We use data-tauri-drag-region to tell Windows "dragging this area moves the window"
            let titlebar_html = r#"
                <div id="custom-titlebar">
                    <div class="titlebar-drag-region" data-tauri-drag-region>
                        <!-- Re-adding the logo here since we deleted the sidebar -->
                        <span style="margin-right: 8px;">💬</span> Messterial
                    </div>
                    <div class="titlebar-controls">
                        <div class="titlebar-button" id="titlebar-minimize">
                            <svg class="titlebar-icon" viewBox="0 0 10 1"><path d="M0 0h10v1H0z"/></svg>
                        </div>
                        <div class="titlebar-button" id="titlebar-maximize">
                            <svg class="titlebar-icon" viewBox="0 0 10 10"><path d="M0 0h10v10H0V0zm1 1v8h8V1H1z"/></svg>
                        </div>
                        <div class="titlebar-button" id="titlebar-close">
                            <svg class="titlebar-icon" viewBox="0 0 10 10"><path d="M9.3 0.7L5 5 0.7 0.7 0 1.4 4.3 5.7 0 10l0.7 0.7L5 6.4l4.3 4.3 0.7-0.7L5.7 5 10 0.7z"/></svg>
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
            .decorations(false) // <--- THIS DISABLES THE DEFAULT WINDOWS FRAME
            .initialization_script(&init_script)
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}