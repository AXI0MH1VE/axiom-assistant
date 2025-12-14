# Axiom Assistant (skeleton)

This repository contains a starter skeleton for the Axiom Assistant blueprint (Zero-Egress, Deterministic, Local-First, Hybrid Intelligence).

Structure:
- `Cargo.toml` — dependencies from blueprint
- `src/` — Rust backend stubs
- `ui/` — Tauri/React frontend skeleton
- `models/` — placeholder for local GGUF models

Next steps:
- Implement Candle-based model loading in `src/modules/probabilistic.rs`
- Integrate SWI-Prolog in `src/modules/deterministic.rs`
- Wire `ipc/orchestrator.rs` to stream tokens to the Tauri frontend
- Add CI and tests
