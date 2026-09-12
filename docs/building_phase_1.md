# Phase 1: Core Command Line Engine (Linux MVP)

## Overview
Phase 1 focuses on extracting raw audio loopback from the Linux system using PipeWire. We aimed to build a performant, zero-loss backend that seamlessly reads float32 PCM blocks.

## Execution Details
- **Workspace Layout:** We set up a Cargo Workspace with `unisystem-core`, `unisystem-audio`, `unisystem-tauri`, and `unisystem-cli`.
- **Audio Logic (`unisystem-audio`):** We leveraged `pipewire-rs` to bind directly to PipeWire. We configured a `MainLoop`, instantiated a `Context` and `Core`, and established a `Stream` connecting to the `Capture` role. The `process` callback accurately retrieves float32 samples from incoming audio buffers and sends them through a callback.
- **CLI (`unisystem-cli`):** We developed a real-time command-line interface that calculates peak amplitudes from the captured audio frames and dynamically prints a volume meter (`===`) in the terminal using carriage returns.
- **Threading Fixes:** We encountered the typical `Send` limitation on PipeWire's `MainLoopRc`. Instead of spawning disparate threads for `stop_flag` checking, we attached a PipeWire `add_timer` directly onto the event loop to periodically inspect shutdown requests.

## Outcomes
The codebase strictly compiles with 0 errors via `cargo check` and correctly captures low-latency packets on standard Linux desktop environments.
