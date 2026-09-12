<div align="center">
  <img src="unisystem-tauri/public/logo.jpg" alt="UniSystem Logo" width="200" style="border-radius: 20px; box-shadow: 0 0 20px rgba(74, 222, 128, 0.5); margin-bottom: 20px;" />

  # UniSystem AudioShare

  **Stream your PC audio to any device on your local network in real-time.** <br>
  An ultra-low latency, cross-platform audio sharing engine built with Rust, Tauri, and WebSockets.

  <p>
    <img alt="Platform Linux" src="https://img.shields.io/badge/Platform-Linux-blue?logo=linux&style=for-the-badge">
    <img alt="Platform Windows" src="https://img.shields.io/badge/Platform-Windows-blue?logo=windows&style=for-the-badge">
    <img alt="Rust" src="https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust&style=for-the-badge">
    <img alt="Tauri" src="https://img.shields.io/badge/GUI-Tauri-yellow?logo=tauri&style=for-the-badge">
  </p>
</div>

---

## 🚀 Features

- **True Real-Time Streaming:** Achieves near-zero latency using raw PCM `f32` chunks over binary WebSockets.
- **Cross-Platform Audio Engine:**
  - **Linux:** Powered by `PipeWire` for the absolute fastest desktop audio capture.
  - **Windows:** Powered by `CPAL` (WASAPI Loopback) for native Windows audio capture.
- **Zero Client Installation:** Just open the provided local IP link on your phone, tablet, or another PC's web browser.
- **Premium Desktop GUI:** A stunning, animated, glassmorphism dark-mode GUI built with Tauri and React.

---

## 🛠️ Prerequisites & Installation

### Linux (Ubuntu / Debian / Pop!_OS)
To compile the audio engine and Tauri, you will need the following development libraries:
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libpipewire-0.3-dev \
    libspa-0.2-dev
```

### Arch Linux / Manjaro
```bash
sudo pacman -Syu
sudo pacman -S --needed \
    webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    pipewire
```

### Windows
1. Install [Rust](https://rustup.rs/).
2. Install [Node.js](https://nodejs.org/).
3. Ensure you have the Visual Studio C++ Build Tools installed (via standard Rust setup).

---

## 🎮 How to Run

### 1. Run the GUI Desktop App (Recommended)
This launches the beautiful Tauri interface with the built-in network visualizer and one-click connection links.
```bash
cd unisystem-tauri
npm install
npm run tauri dev
```
> **Note:** The first compilation will take a few minutes as it builds the Rust Tauri backend. Once running, open the provided URL (e.g. `http://192.168.1.5:8080`) on your mobile browser!

### 2. Run the CLI Version (Headless)
If you prefer a terminal-only experience (great for headless Linux servers or minimalists):
```bash
cd unisystem-cli
cargo run
```
Then, manually open your browser to the IP address printed in the terminal.

---

## 🗺️ Roadmap & Future Plans

- [x] Phase 1: Core PipeWire Audio Engine (Linux)
- [x] Phase 2: React + Tauri Desktop GUI
- [x] Phase 3: WebSocket Web Audio Client
- [x] Phase 4: Windows CPAL / WASAPI Support
- [x] Phase 5: UI Polish & Open-Source Branding
- [ ] **Future:** Pre-compiled standalone `.exe` and `.AppImage` releases on GitHub.
- [ ] **Future:** Native Android App client for enhanced background listening.

---

### License
MIT License. Free and Open-Source.
