# 🎧 UniSystem AudioShare

> **High-performance, extreme low-latency audio capture and network streaming engine.**

![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-lightgrey.svg)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Status](https://img.shields.io/badge/status-Active%20Development-success.svg)

**AudioShare** is a powerful cross-platform application designed to capture raw loopback audio directly from your hardware with strict zero-loss performance. By decoupling system-specific audio drivers (PipeWire on Linux, WASAPI on Windows) from a beautiful frontend UI (Vue/React + Tauri), AudioShare provides a seamless, ultra-low-latency network streaming experience for the modern open-source ecosystem.

---

## ✨ Features
- **Zero-Latency Capture**: Reads float32 PCM blocks directly from hardware/software loopback buffer queues without skipping samples.
- **Cross-Platform**: Architected elegantly via conditional compilation to support PipeWire (Linux) and WASAPI (Windows) effortlessly.
- **Ultra-Low Latency Streaming (Coming Soon)**: Asynchronous UDP network loops for zero-loss stream playback over local network layers.
- **Modern Interface (Coming Soon)**: Breathtaking Tauri-based application shell wrapping the native Rust audio pipelines.

## 🚀 Getting Started (CLI MVP)

Currently, Phase 1 (Core Command Line Engine) is complete for Linux.

### Prerequisites (Linux)
Make sure you have Rust installed along with the PipeWire development headers.
```bash
# Ubuntu/Debian
sudo apt install libpipewire-0.3-dev clang pkg-config
```

### Build & Run
Test the core audio extraction engine in real-time. This command will print out the peak amplitude of your system's current audio stream.
```bash
git clone https://github.com/imuzax/UniSystem-AudioShare.git
cd UniSystem-AudioShare
cargo run --release --bin unisystem-cli
```
*Tip: Play some music in the background while running the CLI to see the volume meter react dynamically!*

## 🏗 Architecture
The codebase is structured into independent, highly modular layers to minimize cognitive friction for new contributors:
- `unisystem-core/`: Shared Rust logic, traits, and networking envelopes.
- `unisystem-audio/`: Cross-platform low-level audio engineering (PipeWire).
- `unisystem-tauri/`: Application shell handling security and async native jobs.
- `unisystem-cli/`: Real-time testing tools and terminal volume meters.

## 📜 License
This project is dual-licensed under the **MIT** and **Apache-2.0** licenses. See the [LICENSE](LICENSE) file for more information.

We welcome contributions! Whether you're a frontend UI developer or a low-level Rust systems engineer, your PRs are immensely valued.
