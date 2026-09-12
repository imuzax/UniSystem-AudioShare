# 🚀 UniSystem AudioShare: Future Development Roadmap

This document outlines the strategic future plans for UniSystem AudioShare, focusing on transitioning the project from a local-network utility into a highly secure, globally accessible, and feature-rich audio streaming platform.

## 1. 🔐 Enhanced Security: Zero-Trust PIN Authentication
**Goal:** Prevent unauthorized access to the audio stream on public or shared networks.

**Implementation Strategy:**
- **Dynamic PIN Generation:** The Rust server (Tauri backend) will generate a random 4-digit PIN upon startup and display it on the Desktop UI.
- **Client Handshake:** When a user opens the web client via the provided URL, instead of connecting directly to the audio stream, they will be greeted by a lock screen.
- **WebSocket Validation:** The client sends the entered PIN via a WebSocket authentication payload. If the PIN matches the server's state, the connection is upgraded to an active audio session. If it fails, the socket drops.

## 2. 🌍 Global Connectivity: Streaming Over Any Network
**Goal:** Allow users to stream their PC audio to their phone even if the phone is on 5G mobile data or a completely different Wi-Fi network globally.

**Implementation Strategy (Two Approaches):**
- **Approach A (P2P WebRTC):** Implement a WebRTC signaling server (using STUN/TURN protocols) to pierce NATs and firewalls. This enables direct, ultra-low latency peer-to-peer audio streaming over the internet without routing all audio traffic through a central server.
- **Approach B (Cloud Tunneling):** Integrate a lightweight, automated reverse-tunnel (e.g., Cloudflare Tunnel or localtunnel) directly into the Tauri app. The app generates a secure public URL (`https://audio-share.trycloudflare.com`) that routes traffic securely to the local PC.

## 3. 📱 Native Android App (.APK)
**Goal:** Prevent the mobile operating system from suspending the audio stream when the screen is locked or the browser is sent to the background.

**Implementation Strategy:**
- Utilize Tauri v2's native mobile compilation capabilities to build a standalone Android `APK`.
- Implement an Android **Foreground Service** (`FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK`) in Kotlin/Java.
- This registers the application as an active music player with the OS, ensuring the WebSocket connection and audio decoding remain alive indefinitely in the background.

## 4. 🎛️ Remote Control Interface
**Goal:** Turn the mobile client into a powerful remote controller for the host PC.

**Implementation Strategy:**
- Build bidirectional WebSocket communication.
- The web client sends JSON commands (e.g., `{"command": "volume_up", "value": 10}`).
- The Rust server receives these commands and interfaces with the host OS (Windows/Linux) using native audio crates to control the master volume, mute, or skip tracks.

## 5. 🎧 Spatial Audio & Surround Sound Networking (Dolby-style)
**Goal:** Transform multiple connected mobile devices into a synchronized, multi-channel surround sound system (e.g., 5.1 or 7.1 Home Theater).

**Implementation Strategy:**
- **Channel Splitting (Backend):** The Rust server will capture the host PC's audio and separate the channels (Front-Left, Front-Right, Surround-Left, Subwoofer, etc.).
- **Client Role Assignment (UI):** The React web client will allow users to select their device's physical location/role in the room.
- **Targeted Streaming:** The server will stream only the specific audio channel data to the corresponding connected client.
- **Clock Synchronization:** Implement NTP (Network Time Protocol) logic over WebSockets to ensure all devices play their respective channels at the exact same millisecond, eliminating phasing and echo.

---
*This roadmap serves as the architectural blueprint for the next major versions of UniSystem AudioShare. These features will be developed and integrated incrementally.*
