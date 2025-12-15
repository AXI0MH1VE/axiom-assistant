# Models Directory

This directory is intended for local AI model files used by Axiom Assistant.

## Supported Formats

- **GGUF** files for Candle-based LLM inference
- **ONNX** models for specialized inference
- **Custom** model formats as needed

## Recommended Models

For testing and development, you can use:

1. **TinyLlama-1.1B** (GGUF format)
   - Size: ~600 MB
   - Good for development testing
   
2. **Phi-2** (GGUF format)
   - Size: ~2.7 GB
   - Better quality, still lightweight

3. **Llama-2-7B** (GGUF format)
   - Size: ~4-7 GB depending on quantization
   - Production quality

## Downloading Models

Models are NOT included in the repository due to their size.

### Using Hugging Face CLI

```bash
# Install huggingface_hub
pip install huggingface_hub

# Download a model (example: TinyLlama)
huggingface-cli download TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF \
    tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf \
    --local-dir ./models \
    --local-dir-use-symlinks False
```

### Manual Download

Visit [Hugging Face](https://huggingface.co/models) and download GGUF format models.

Place downloaded files directly in this directory:

```
models/
├── README.md (this file)
├── model-name.gguf
└── model-name-config.json (if applicable)
```

## Important Notes

⚠️ **Security**: Only download models from trusted sources.

⚠️ **Size**: Model files can be several GB. Ensure adequate disk space.

⚠️ **Licensing**: Verify model licenses before use in production.

## Configuration

Update your configuration to point to the model file:

```rust
// In code
let model_path = "models/your-model.gguf";
```

Or via environment variable:
```bash
export AXIOM_MODEL_PATH="./models/your-model.gguf"
```

## .gitignore

Model files are excluded from git tracking by default. See `.gitignore`:

```
models/*.gguf
models/*.bin
```
