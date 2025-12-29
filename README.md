# Koto DAW

A modern Digital Audio Workstation built with Rust and WebGPU.

## Overview

Koto is a cross-platform DAW designed to compete with industry-standard tools like Cubase and Pro Tools, featuring:

- High-performance audio engine with lock-free real-time processing
- Modern UI built with egui and WebGPU
- MIDI recording and playback
- Audio recording and playback
- Plugin support (VST3/AU/CLAP planned)

## Requirements

- Rust 1.75+
- Audio device with CoreAudio (macOS), WASAPI (Windows), or ALSA (Linux)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## License

MIT
