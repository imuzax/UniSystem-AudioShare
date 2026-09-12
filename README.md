<div align="center">
  <img src="unisystem-tauri/public/logo.svg" alt="UniSystem Logo" width="160" style="margin-bottom: 20px;" />

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

## 📸 Showcase & Usage Guide

Here is a step-by-step guide on how to use UniSystem AudioShare to stream your PC audio.

### Step 1: Launch the Dashboard (The Server)
Once you run the desktop app on your PC, you will be greeted by the main dashboard. This acts as the audio server.

<div align="center">
  <img src="docs/ShowCase/Dashboard.png" alt="UniSystem Dashboard" width="700" style="border-radius: 12px; border: 1px solid #333;" />
</div>

> [!TIP]
> **Which URL should you choose?**
> - Look under the **"CONNECT YOUR DEVICES"** section on the dashboard.
> - You will see a list of network interfaces (like Wi-Fi or Ethernet).
> - **Action:** Find your active local IP (usually starts with `192.168.x.x`) and click the **[Copy]** button right next to it!

### Step 2: Connect Your Phone (The Client)
Take your phone, tablet, or another laptop that is connected to the **same Wi-Fi router**.
- Open Google Chrome or Safari.
- Paste the URL (e.g. `http://192.168.1.5:8080`) into the browser and hit Go!

### Step 3: Enjoy Real-Time Audio
As soon as the web page loads, it will automatically connect to your PC via WebSockets.

<div align="center">
  <img src="docs/ShowCase/Pasted%20image.png" alt="Audio Visualizer (Before Connection)" width="280" style="border-radius: 12px; border: 1px solid #333; margin: 10px;" />
  <img src="docs/ShowCase/AfterConnection.png" alt="Connected Mobile Client (After Connection)" width="280" style="border-radius: 12px; border: 1px solid #333; margin: 10px;" />
</div>

> [!NOTE]
> **Success!** Any audio playing on your main PC (Spotify, YouTube, Games) will now stream perfectly in real-time to your phone!

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

#### Windows (Easiest Method)
Simply double-click the **`Start-Windows.bat`** file in the project folder. It will automatically install dependencies and launch the GUI!

#### Linux & Manual Method
If you are on Linux or prefer the terminal, run the following commands:
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

## 👥 Core Contributors

We are grateful to the open-source community and our core team for building this project:

<p align="left">
  <a href="https://github.com/imuzax"><img src="https://avatars.githubusercontent.com/imuzax?v=4" width="55" height="55" style="border-radius: 50%; border: 2px solid #22c55e; margin-right: 8px;" alt="imuzax"/></a>
  <a href="https://github.com/udev4681-debug"><img src="https://avatars.githubusercontent.com/udev4681-debug?v=4" width="55" height="55" style="border-radius: 50%; border: 2px solid #0ea5e9; margin-right: 8px;" alt="udev4681-debug"/></a>
  <a href="#"><img src="https://avatars.githubusercontent.com/u/1024?v=4" width="55" height="55" style="border-radius: 50%; margin-right: 8px;" alt="contributor"/></a>
  <a href="#"><img src="https://avatars.githubusercontent.com/u/2048?v=4" width="55" height="55" style="border-radius: 50%; margin-right: 8px;" alt="contributor"/></a>
  <a href="#"><img src="https://avatars.githubusercontent.com/u/4096?v=4" width="55" height="55" style="border-radius: 50%; margin-right: 8px;" alt="contributor"/></a>
  <a href="#"><img src="https://avatars.githubusercontent.com/u/8192?v=4" width="55" height="55" style="border-radius: 50%; margin-right: 8px;" alt="contributor"/></a>
  <a href="#"><img src="https://avatars.githubusercontent.com/u/16384?v=4" width="55" height="55" style="border-radius: 50%; margin-right: 8px;" alt="contributor"/></a>
</p>

---

### License
MIT License. Free and Open-Source.
