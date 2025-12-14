use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use std::time::Duration;

/// Mock ProbabilisticModule for local development.
/// Streams tokens with a small delay to simulate an LLM.
pub struct ProbabilisticModule {}

impl ProbabilisticModule {
    pub async fn load_local_llm() -> anyhow::Result<Self> {
        // In production, load Candle/GGUF model here. For now return a mock.
        Ok(ProbabilisticModule {})
    }

    pub async fn infer(&self, prompt: &str) -> anyhow::Result<String> {
        // Mock: return the prompt echoed with a suffix to indicate a draft.
        Ok(format!("{}\n\n[LLM draft]", prompt))
    }

    /// Streams whitespace tokens from `prompt` with a small delay between tokens.
    pub async fn stream_tokens(&self, prompt: &str) -> ReceiverStream<String> {
        let (tx, rx) = mpsc::channel(16);
        let prompt_owned = prompt.to_string();

        tokio::spawn(async move {
            let tokens: Vec<String> = prompt_owned
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            for token in tokens.into_iter() {
                if tx.send(format!("{} ", token)).await.is_err() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(80)).await;
            }

            // final punctuation to indicate completion
            let _ = tx.send("\n".to_string()).await;
        });

        ReceiverStream::new(rx)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProbRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ProbResponse {
    pub text: String,
    pub confidence: f32,
    pub tokens_per_sec: f32,
}
