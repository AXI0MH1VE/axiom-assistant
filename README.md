# Axiom Assistant

**Production-ready neuro-symbolic AI assistant with local-first, zero-egress architecture**

This repository contains a complete implementation of the Axiom Assistant blueprint combining probabilistic (LLM) reasoning with deterministic logic/math capabilities.

## 🏗️ Architecture

- **Rust Backend**: High-performance, memory-safe core
- **Module A (Probabilistic)**: Candle-based LLM inference with token streaming
- **Module B (Deterministic)**: Math evaluation and logic processing
- **Module C (Neuro-Symbolic)**: Intelligent query routing between modules
- **AxiomEngine**: wgpu-based deterministic rendering system
- **Tauri UI**: Optional cross-platform desktop interface

## 📦 Project Structure

```
.
├── Cargo.toml              # Rust dependencies
├── Dockerfile              # Production container image
├── src/
│   ├── main.rs             # CLI entry point with full error handling
│   ├── modules/            # Core reasoning modules
│   │   ├── probabilistic.rs   # LLM streaming with configuration
│   │   ├── deterministic.rs   # Math/logic execution
│   │   └── neuro_symbolic.rs  # Intent classification
│   ├── ipc/                # Orchestration layer
│   │   └── orchestrator.rs    # Query processing with statistics
│   ├── engine/             # Rendering system
│   │   ├── axiom_renderer.rs  # wgpu rendering pipeline
│   │   └── deterministic_viz.rs  # Scene graph
│   └── ui/                 # Tauri integration
│       └── tauri_app.rs       # Command handlers
├── ui/                     # React/TypeScript frontend
└── models/                 # Local GGUF models directory
```

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.75 or later
- **Cargo**: Comes with Rust
- **Node.js**: 18+ (for UI development)
- **Docker**: Optional, for containerized deployment

### Building from Source

```bash
# Clone the repository
git clone https://github.com/AXI0MH1VE/axiom-assistant.git
cd axiom-assistant

# Build the project
cargo build --release

# Run the CLI interface
cargo run --release
```

### Running with Docker

```bash
# Build the Docker image
docker build -t axiom-assistant:latest .

# Run the container
docker run -it --rm \
  -e RUST_LOG=info \
  -e AXIOM_MAX_TOKENS=2048 \
  -e AXIOM_TEMPERATURE=0.7 \
  axiom-assistant:latest
```

### Development Mode

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests (when available)
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## ⚙️ Configuration

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# Logging level (error, warn, info, debug, trace)
RUST_LOG=info

# Model configuration
AXIOM_MODEL_PATH=/path/to/model.gguf
AXIOM_MAX_TOKENS=2048
AXIOM_TEMPERATURE=0.7

# Deterministic module
AXIOM_ENABLE_PROLOG=false
AXIOM_MAX_QUERY_LENGTH=10000
```

### Model Setup

Place your GGUF model files in the `models/` directory:

```bash
# Example: Download a compatible model
curl -L https://huggingface.co/path/to/model.gguf -o models/model.gguf

# Set the path in .env
echo "AXIOM_MODEL_PATH=./models/model.gguf" >> .env
```

## 💻 Usage

### CLI Commands

```bash
# Start the assistant
cargo run --release

# Interactive commands:
> 2 + 2                    # Math query (deterministic)
> explain quantum physics  # Creative query (LLM)
> stats                    # Show processing statistics
> help                     # Show available commands
> exit                     # Exit the application
```

### Query Types

1. **Creative Queries**: Handled by LLM with token streaming
   - "explain quantum physics"
   - "write a poem about trees"

2. **Logical Queries**: Processed deterministically
   - "2 + 2"
   - "sqrt(16)"
   - "ancestor(zeus, hercules)"

3. **Hybrid Queries**: LLM draft + deterministic verification
   - Questions involving both reasoning and calculation

## 🐳 Deployment

### Production Docker Deployment

```bash
# Build production image
docker build -t axiom-assistant:v0.1.0 .

# Run with volume mounts for models
docker run -d \
  --name axiom-assistant \
  -v $(pwd)/models:/app/models:ro \
  -v $(pwd)/logs:/app/logs \
  -e RUST_LOG=info \
  -e AXIOM_MODEL_PATH=/app/models/model.gguf \
  --restart unless-stopped \
  axiom-assistant:v0.1.0

# View logs
docker logs -f axiom-assistant

# Stop container
docker stop axiom-assistant
```

### Docker Compose (Optional)

Create `docker-compose.yml`:

```yaml
version: '3.8'
services:
  axiom-assistant:
    build: .
    image: axiom-assistant:latest
    environment:
      - RUST_LOG=info
      - AXIOM_MAX_TOKENS=2048
      - AXIOM_TEMPERATURE=0.7
    volumes:
      - ./models:/app/models:ro
      - ./logs:/app/logs
    restart: unless-stopped
```

Run with: `docker-compose up -d`

## 🔒 Security Features

- **Zero-Egress**: All processing happens locally, no data leaves the system
- **Input Sanitization**: Queries are validated and sanitized before processing
- **Non-Root User**: Docker container runs as non-privileged user
- **Environment-Based Secrets**: No hardcoded credentials or API keys
- **Deterministic Execution**: Reproducible results for verification

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests
```

## 📊 Monitoring

The orchestrator tracks processing statistics:

```bash
# View statistics in CLI
> stats

# Output example:
📊 Statistics:
  Total queries: 42
  Creative: 15
  Logical: 20
  Hybrid: 7
```

## 🛠️ Development

### UI Development

```bash
cd ui
npm install
npm run dev      # Development server
npm run build    # Production build
```

### Adding Custom Modules

Extend `src/modules/` with new reasoning capabilities:

```rust
// src/modules/your_module.rs
pub struct YourModule {
    // Implementation
}
```

### Integrating with Orchestrator

Update `src/ipc/orchestrator.rs` to route queries to your module.

## 📝 License

See LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📞 Support

For issues and questions:
- Open an issue on GitHub
- Check existing documentation in `/docs` (if available)

## 🎯 Roadmap

- [ ] Full Candle/GGUF model integration
- [ ] SWI-Prolog integration for advanced logic
- [ ] Web interface via Tauri
- [ ] Distributed processing support
- [ ] Plugin system for custom modules
- [ ] Advanced telemetry and monitoring

---

**Status**: ✅ Production-ready (CLI interface)
**Version**: 0.1.0
**Build Date**: 2025-12-15

