use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tauri::{Emitter, Listener};
use tauri::{webview::NewWindowResponse, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_updater::UpdaterExt;

static WINDOW_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
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
                div[role="navigation"][aria-label="Inbox switch"] { 
                    width: 0 !important;
                    height: 0 !important;
                    opacity: 0 !important;
                    overflow: hidden !important;
                    position: absolute !important;
                    pointer-events: none !important;
                }

                /* Hide Banner bloat */
                a[href="https://www.facebook.com/"],
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
                            DIALOGS & MODALS
                   ========================================= */
                
                div[role="dialog"] {
                    position: fixed !important;
                    top: 50% !important;
                    left: 50% !important;
                    transform: translate(-50%, -50%) !important;
                    max-height: calc(100vh - var(--titlebar-height) - 60px) !important;
                    max-width: calc(100vw - 60px) !important;
                    overflow-y: auto !important;
                    overflow-x: hidden !important;
                    margin: 0 !important;
                }
                
                div[role="dialog"] > div {
                    max-height: inherit !important;
                    overflow: visible !important;
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

                /* Nuke Native Overlays */
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

                div.x11lfxj5:has(> div[role="button"][aria-label="Zamknij tryb wpisywania"]) ~ label,
                div.x11lfxj5:has(> div[role="button"][aria-label="Close typing mode"]) ~ label,
                div.x11lfxj5:has(> div[role="button"][aria-label="Zamknij tryb wpisywania"]) ~ div:has(input[type="search"]),
                div.x11lfxj5:has(> div[role="button"][aria-label="Close typing mode"]) ~ div:has(input[type="search"]) {
                    margin-left: 26px !important;
                    transition: margin-left 0.3s cubic-bezier(0.05, 0.7, 0.1, 1.0) !important;
                }

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
                        <svg class="titlebar-icon" viewBox="0 0 24 24"><path d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z"/></svg>
                    </div>
                    <div class="titlebar-controls">
                        <div class="titlebar-button" id="titlebar-minimize"><svg class="titlebar-icon" viewBox="0 0 24 24"><path d="M5 19h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1s.45 1 1 1z"/></svg></div>
                        <div class="titlebar-button" id="titlebar-maximize"><svg class="titlebar-icon" viewBox="0 0 24 24"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/></svg></div>
                        <div class="titlebar-button" id="titlebar-close"><svg class="titlebar-icon" viewBox="0 0 24 24"><path d="M18.3 5.71a.9959.9959 0 0 0-1.41 0L12 10.59 7.11 5.7a.9959.9959 0 0 0-1.41 0c-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/></svg></div>
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
                                'div[role=\"navigation\"] div[role=\"button\"][aria-label=\"Ustawienia użytkownika\"]', // Polish
                                'div[role=\"navigation\"] div[role=\"button\"][aria-label=\"Account settings\"]', // English
                                'div[role=\"navigation\"] div[role=\"button\"][aria-label*=\"Profil\"]',
                                // Fallback: try to find the very last button in the hidden sidebar rail
                                'div[role=\"navigation\"] > div:last-child div[role=\"button\"]' 
                            ];
                            
                            for (const selector of selectors) {{
                                const btn = document.querySelector(selector);
                                if (btn) {{
                                    console.log('Messterial: Found settings button via selector:', selector);
                                    btn.click();
                                    return;
                                }}
                            }}
                            console.error(\"Messterial: Could not find settings button.\");
                        }});
                    }};

                    const turboMode = () => {{
                        // Disable smooth scrolling for instant feel
                        const css = 'html, body {{ scroll-behavior: auto !important; }} * {{ transition-delay: 0ms !important; }}';
                        const s = document.createElement('style');
                        s.innerHTML = css;
                        document.head.appendChild(s);

                        // Block Analytics/Logging
                        const originalFetch = window.fetch;
                        window.fetch = async (...args) => {{
                            const url = args[0] ? args[0].toString() : '';
                            if (url.includes('/logging') || url.includes('/falco') || url.includes('analytics')) {{
                                return new Response();
                            }}
                            return originalFetch(...args);
                        }};
                    }};
                    turboMode();

                    // External link handler
                    const setupExternalLinks = () => {{
                        const openExternalUrl = async (href) => {{
                            if (!window.__TAURI__) return false;
                            try {{
                                // Tauri 2 plugin invoke pattern
                                await window.__TAURI__.core.invoke('plugin:opener|open_url', {{ url: href }});
                                return true;
                            }} catch (err) {{
                                console.error('Messterial: Failed to open URL:', err);
                                return false;
                            }}
                        }};

                        document.addEventListener('click', async (e) => {{
                            const link = e.target.closest('a[href]');
                            if (!link) return;
                            
                            const href = link.getAttribute('href');
                            if (!href) return;
                            
                            // Check if it's an external link (not messenger.com)
                            const isExternal = href.startsWith('http://') || href.startsWith('https://');
                            const isMessengerInternal = href.includes('messenger.com') || href.includes('facebook.com/messages') || href.includes('facebook.com/login');
                            
                            if (isExternal && !isMessengerInternal) {{
                                e.preventDefault();
                                e.stopPropagation();
                                await openExternalUrl(href);
                            }}
                        }}, true);
                        
                        // Also handle middle-click
                        document.addEventListener('auxclick', async (e) => {{
                            if (e.button !== 1) return; // Middle click only
                            
                            const link = e.target.closest('a[href]');
                            if (!link) return;
                            
                            const href = link.getAttribute('href');
                            if (!href) return;
                            
                            const isExternal = href.startsWith('http://') || href.startsWith('https://');
                            const isMessengerInternal = href.includes('messenger.com') || href.includes('facebook.com/messages') || href.includes('facebook.com/login');
                            
                            if (isExternal && !isMessengerInternal) {{
                                e.preventDefault();
                                e.stopPropagation();
                                await openExternalUrl(href);
                            }}
                        }}, true);
                    }};
                    setupExternalLinks();

                    // Badge notification
                    const setupBadgeNotifications = () => {{
                        let lastBadgeCount = -1; // Start at -1 to force first update
                        console.log('Messterial: Badge notifications initialized');
                        
                        // Create a badge icon with number overlay
                        const createBadgeIcon = async (count) => {{
                            const size = 16;
                            const canvas = document.createElement('canvas');
                            canvas.width = size;
                            canvas.height = size;
                            const ctx = canvas.getContext('2d');
                            
                            // Draw red circle
                            ctx.fillStyle = '#e53935';
                            ctx.beginPath();
                            ctx.arc(size/2, size/2, size/2, 0, Math.PI * 2);
                            ctx.fill();
                            
                            // Draw text
                            ctx.fillStyle = 'white';
                            ctx.font = 'bold 11px Arial';
                            ctx.textAlign = 'center';
                            ctx.textBaseline = 'middle';
                            const text = count > 9 ? '9+' : count.toString();
                            ctx.fillText(text, size/2, size/2 + 1);
                            
                            // Create Tauri Image
                            const imageData = ctx.getImageData(0, 0, size, size);
                            const rgba = new Uint8Array(imageData.data);
                            const Image = window.__TAURI__.image.Image;
                            return await Image.new(rgba, size, size);
                        }};
                        
                        const countUnreadChats = () => {{
                            // Count unread indicators in the chat list
                            const chatRows = document.querySelectorAll('div[role=\"navigation\"] div[role=\"row\"]');
                            let unreadCount = 0;
                            
                            chatRows.forEach((row, idx) => {{
                                // Look for the unread indicator dot
                                const unreadIndicator = row.querySelector('div[aria-hidden=\"true\"][role=\"button\"][tabindex=\"-1\"]');
                                if (unreadIndicator) {{
                                    unreadCount++;
                                }}
                            }});
                            
                            return unreadCount;
                        }};
                        
                        const updateBadge = async () => {{
                            if (!window.__TAURI__) return;
                            
                            const count = countUnreadChats();
                            
                            if (count !== lastBadgeCount) {{
                                lastBadgeCount = count;
                                try {{
                                    const appWindow = window.__TAURI__.window.getCurrentWindow();
                                    
                                    if (count > 0) {{
                                        // Create overlay icon with the count
                                        const icon = await createBadgeIcon(count);
                                        await appWindow.setOverlayIcon(icon);
                                        console.log('Messterial: Badge set to', count);
                                    }} else {{
                                        // Clear overlay
                                        await appWindow.setOverlayIcon(null);
                                        console.log('Messterial: Badge cleared');
                                    }}
                                }} catch (err) {{
                                    console.error('Messterial: Failed to update badge:', err);
                                }}
                            }}
                        }};

                        // Observe the navigation/chat list for changes
                        const observeChatList = () => {{
                            const nav = document.querySelector('div[role=\"navigation\"]');
                            if (nav) {{
                                const observer = new MutationObserver(() => updateBadge());
                                observer.observe(nav, {{ childList: true, subtree: true, attributes: true }});
                                console.log('Messterial: Observing chat list for unread changes');
                            }}
                        }};

                        // Poll periodically to catch all updates and initialize observer
                        setInterval(() => {{
                            updateBadge();
                            // Try to set up observer if not already done
                            if (!document.querySelector('div[role=\"navigation\"].__messterial_observed')) {{
                                const nav = document.querySelector('div[role=\"navigation\"]');
                                if (nav) {{
                                    nav.classList.add('__messterial_observed');
                                    observeChatList();
                                }}
                            }}
                        }}, 2000);
                        
                        // Initial check
                        setTimeout(updateBadge, 3000);
                    }};
                    setupBadgeNotifications();

                    // Update consent prompt via events
                    const setupUpdateConsent = () => {{
                        if (!window.__TAURI__ || !window.__TAURI__.event) return;
                        window.__TAURI__.event.listen('messterial:prompt-update', async (event) => {{
                            try {{
                                const payload = event && event.payload ? event.payload : {{}};
                                const current = payload.current || '';
                                const version = payload.version || '';
                                const message = `Version ${{version}} is available.\n\nCurrent version: ${{current}}\nNew version: ${{version}}\n\nInstall now?`;
                                const approved = window.confirm(message);
                                await window.__TAURI__.event.emit('messterial:update-decision', {{ approved }});
                            }} catch (err) {{
                                console.error('Messterial: Update consent failed:', err);
                                try {{
                                    await window.__TAURI__.event.emit('messterial:update-decision', {{ approved: false }});
                                }} catch {{}}
                            }}
                        }});
                    }};
                    setupUpdateConsent();

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

            let app_handle_for_new_window = app.handle().clone();

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://www.messenger.com/login".parse().unwrap())
            )
            .title("Messterial")
            .inner_size(1200.0, 800.0)
            .decorations(false)
            .initialization_script(&init_script)
            .on_new_window(move |url, features| {
                // Allow Messenger call windows and other legitimate popups
                let url_str = url.as_str();
                println!("Messterial: New window request for URL: {}", url_str);
                
                // Check if this is a Messenger/Facebook related URL (calls, auth, etc.)
                let is_messenger_related = url_str.contains("messenger.com") 
                    || url_str.contains("facebook.com")
                    || url_str.contains("fbcdn.net")
                    || url_str.starts_with("about:blank"); // Calls often start with about:blank
                
                if is_messenger_related {
                    // Generate a unique window label
                    let window_id = WINDOW_COUNTER.fetch_add(1, Ordering::SeqCst);
                    let label = format!("popup-{}", window_id);
                    
                    // Create the popup window for calls/auth
                    let builder = WebviewWindowBuilder::new(
                        &app_handle_for_new_window,
                        &label,
                        WebviewUrl::External(url.clone()),
                    )
                    .window_features(features)
                    .title("Messenger")
                    .inner_size(800.0, 600.0)
                    .center()
                    .on_document_title_changed(|window, title| {
                        let _ = window.set_title(&title);
                    });
                    
                    match builder.build() {
                        Ok(window) => NewWindowResponse::Create { window },
                        Err(e) => {
                            eprintln!("Messterial: Failed to create popup window: {}", e);
                            NewWindowResponse::Deny
                        }
                    }
                } else {
                    // For non-Messenger URLs, open in external browser
                    println!("Messterial: Denying external URL: {}", url_str);
                    NewWindowResponse::Deny
                }
            })
            .build()?;

            // Check for updates and ask for user consent via JS confirm
            {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    match app_handle.updater() {
                        Ok(updater) => match updater.check().await {
                            Ok(Some(update)) => {
                                println!("Messterial: Update found: {}", update.version);
                                // Small delay to let the webview initialize its listeners
                                std::thread::sleep(Duration::from_millis(1500));

                                let _ = app_handle.emit(
                                    "messterial:prompt-update",
                                    serde_json::json!({
                                        "current": update.current_version,
                                        "version": update.version
                                    }),
                                );

                                let (tx, rx) = std::sync::mpsc::channel::<bool>();
                                let tx_clone = tx.clone();
                                app_handle.listen("messterial:update-decision", move |event| {
                                    let payload_str = event.payload();
                                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(payload_str) {
                                        if let Some(appr) = val.get("approved").and_then(|v| v.as_bool()) {
                                            let _ = tx_clone.send(appr);
                                        }
                                    }
                                });

                                let approved = rx.recv_timeout(Duration::from_secs(60)).unwrap_or(false);
                                if approved {
                                    println!("Messterial: User approved update, downloading...");
                                    if let Err(e) = update.download_and_install(|_,_| {}, || {}).await {
                                        println!("Messterial: Update failed: {}", e);
                                    } else {
                                        println!("Messterial: Update installed! Restarting...");
                                        app_handle.restart();
                                    }
                                } else {
                                    println!("Messterial: User declined update or timeout");
                                }
                            }
                            Ok(None) => println!("Messterial: You are on the latest version."),
                            Err(e) => println!("Messterial: Failed to check for updates: {}", e),
                        },
                        Err(e) => println!("Messterial: Failed to initialize updater: {}", e),
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
