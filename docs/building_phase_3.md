# Phase 3: Ultra-Low Latency Network Streaming Engine

## Overview
Phase 3 expands on the core audio capture by transmitting the raw PCM audio floats across a local network using WebSockets. This circumvents the need for dedicated mobile apps, allowing a simple phone browser to act as a zero-latency audio receiver via the Web Audio API.

## Execution Details
- **WebSocket Server (`unisystem-core`):** We utilized `tokio`, `axum`, and `axum-extra::ws` to build an asynchronous Web Server that broadcasts messages to connected clients.
- **Broadcast Channel Integration (`unisystem-cli`):** The `tokio::sync::broadcast` channel seamlessly connects the PipeWire capture loop to the web server, pushing the `Vec<f32>` arrays globally.
- **Web Client (`web-client`):** We built a pure HTML5/JavaScript frontend using `AudioContext` and `Float32Array`. It automatically pulls binary packets from the WebSocket, unpacks the float samples, and feeds them into dynamically created `AudioBufferSourceNode` objects for fluid continuous playback with a basic software jitter buffer mechanism.

## Outcomes
The codebase compiles safely, and running `cargo run --release --bin unisystem-cli` now concurrently starts the audio capture daemon and hosts the HTTP server on port `8080`.
