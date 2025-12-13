# Messterial
A material 3-ish wrapper for Meta Messenger I made because they're removing the desktop app.

# Features
- Sleek and modern Material Design UI
- Responsive layout so you can use the app however you like
- Enchanced performance with removed trackers, instant scrolling and reduced transition delays
- Custom Title Bar for full 
- More privacy with analytics and logging blocking (this still needs improvement)

# Instalation
### Using the prebuilt app
You can download the latest release from the [Releases](https://github.com/Nexenek/messterial/releases) page. Just pick the right version for your OS (Windows, macOS, Linux) and install it like any other app.
### Building from source
1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) installed.
2. Clone this repository:
```bash
git clone https://github.com/Nexenek/messterial.git
cd messterial
```
3. Install the dependencies:
```bash
bun install
```
4. Build the app:
```bash
bun tauri build
# or for development
# bun tauri dev
```
You'll find the installer or the app in the `src-tauri/target/release` folder.

# Usage
After launching the application, you will be greeted with the Messenger login page. Enter your credentials to start chatting.

If you encounter any bugs or have feature requests, please open an issue on the [GitHub Issues](https://github.com/Nexenek/messterial/issues) page.

# What is planned (roadmap)

#### Miscellaneous:
- [x] Custom Title Bar
- [x] Move settings from the sidebar via native menu
- [x] Improve performance with better hardware acceleration
- [ ] Persist Window State
- [ ] Custom app logo
#### Theming:
- [x] Custom UI layout
- [x] Basic material implementation
- [x] Round chat bubbles
- [x] Custom animations
- [ ] Custom scrollbars
- [ ] Accent color picker
- [ ] Dynamic font sizing
#### Privacy & Anti-Bloat:
- [x] Remove sidebar (marketplace, stories, etc.)
- [x] Remove facebook connections
- [ ] Block "seen" indicators
- [ ] Block "typing" indicators
- [ ] Further tracker blocking
#### System Integration:
- [ ] Tray icon
- [ ] Native notifications
- [ ] Taskbar badge

---

I'm not affiliated with Meta or Messenger in any way. This is a personal project made for educational purposes.