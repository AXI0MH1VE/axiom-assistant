# Axiom Assistant

**Axiom Assistant** is a production-ready, zero-egress, deterministic, local-first hybrid intelligence system combining probabilistic AI (LLM) with deterministic symbolic reasoning.

## Architecture

The system consists of three main modules:

1. **Probabilistic Module** (`src/modules/probabilistic.rs`) - Local LLM inference with token streaming
2. **Deterministic Module** (`src/modules/deterministic.rs`) - Logic reasoning and mathematical computation
3. **Neuro-Symbolic Router** (`src/modules/neuro_symbolic.rs`) - Intent classification and module orchestration

## Project Structure

```
.
├── Cargo.toml              # Rust dependencies
├── Dockerfile              # Production container image
├── src/
│   ├── main.rs            # CLI entry point
│   ├── modules/           # AI modules (probabilistic, deterministic, router)
│   ├── ipc/               # Orchestrator and contracts
│   ├── engine/            # AxiomEngine GPU rendering
│   └── ui/                # Tauri application bridge
├── ui/                    # Tauri/React frontend
└── models/                # Local GGUF/model files (not tracked)
```

## Prerequisites

### For Development

- **Rust** 1.75+ (`rustup` recommended)
- **Node.js** 18+ and npm (for UI)
- **System libraries** (Linux):
  - `libgtk-3-dev`
  - `libwebkit2gtk-4.1-dev`
  - `libayatana-appindicator3-dev`
  - `librsvg2-dev`
  - `libglib2.0-dev`

Install on Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev librsvg2-dev libglib2.0-dev \
    libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
```

### For Production (Docker)

- **Docker** 20.10+
- **Docker Compose** (optional)

## Building

### Build from Source

```bash
# Clone the repository
git clone https://github.com/AXI0MH1VE/axiom-assistant.git
cd axiom-assistant

# Build the Rust backend
cargo build --release

# The binary will be at target/release/axiom-assistant
```

### Build with Docker

```bash
# Build the Docker image
docker build -t axiom-assistant:latest .

# Run the container
docker run -it --rm axiom-assistant:latest
```

## Running

### CLI Mode (Development)

```bash
# Run with cargo
cargo run

# Or run the built binary
./target/release/axiom-assistant
```

The CLI will prompt for queries. Type your question and press Enter.

### Tauri Desktop Application

```bash
# Install UI dependencies
cd ui
npm install

# Run in development mode (requires cargo)
npm run tauri dev

# Build production application
npm run tauri build
```

### Docker Deployment

```bash
# Run with environment variables
docker run -d \
  --name axiom-assistant \
  -e RUST_LOG=info \
  -p 8080:8080 \
  -v $(pwd)/models:/app/models \
  axiom-assistant:latest
```

### Docker Compose (Recommended for Production)

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  axiom-assistant:
    build: .
    container_name: axiom-assistant
    restart: unless-stopped
    environment:
      - RUST_LOG=info
      - RUST_BACKTRACE=1
    volumes:
      - ./models:/app/models:ro
    ports:
      - "8080:8080"
    healthcheck:
      test: ["CMD", "pgrep", "-f", "axiom-assistant"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 5s
```

Run with:
```bash
docker-compose up -d
```

## Configuration

### Environment Variables

- `RUST_LOG` - Logging level (trace, debug, info, warn, error)
- `RUST_BACKTRACE` - Enable backtrace (0, 1, full)

### Models

Place local GGUF or model files in the `models/` directory:

```bash
models/
├── llm-model.gguf
└── README.md
```

## Features

✅ **Zero-Egress**: All processing happens locally, no external API calls  
✅ **Deterministic**: Symbolic reasoning with reproducible results  
✅ **Hybrid Intelligence**: Combines LLM creativity with logical verification  
✅ **Token Streaming**: Real-time response generation  
✅ **GPU Acceleration**: wgpu-based deterministic rendering  
✅ **Cross-Platform**: Linux, macOS, Windows support via Tauri  

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Security

- ✅ No hardcoded secrets (use environment variables)
- ✅ Local-only processing (no external API calls)
- ✅ Non-root Docker container execution
- ✅ Minimal container attack surface

## Troubleshooting

### Build Fails on Linux

Ensure all system dependencies are installed:
```bash
sudo apt-get install -y build-essential pkg-config libssl-dev
```

### GPU Not Detected

The AxiomEngine will fall back to CPU rendering if GPU is unavailable. Check:
```bash
# Verify GPU drivers
vulkaninfo  # or use wgpu adapter info
```

### Models Not Loading

Ensure models are in the correct format (GGUF for Candle) and placed in `models/` directory.

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Support

For issues and questions, please open a GitHub issue at:
https://github.com/AXI0MH1VE/axiom-assistant/issues

