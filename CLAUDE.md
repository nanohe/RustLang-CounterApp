# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # Build the project
cargo run            # Build and run the GUI app
cargo check          # Fast type/borrow check without building
cargo clippy         # Lint
cargo test           # Run tests
```

## Architecture

This is a minimal `eframe`/`egui` desktop GUI app (Rust 2024 edition).

- `src/main.rs` — the entire application. Contains `main()` which launches the native window, and `CounterApp` struct that implements `eframe::App`.
- The `update()` method on `CounterApp` is called every frame by eframe and is where all UI rendering and state mutation happens (immediate-mode GUI pattern).
- State lives directly on the `CounterApp` struct; there is no separate state management layer.
