use futures::{stream, StreamExt, stream::BoxStream};
use crate::modules::probabilistic::ProbabilisticModule;
use crate::modules::deterministic::DeterministicModule;
use crate::modules::neuro_symbolic::{NeuroSymbolicRouter, Intent};

pub struct Orchestrator {
    pub prob_module: ProbabilisticModule,
    pub det_module: DeterministicModule,
    pub router: NeuroSymbolicRouter,
}

impl Orchestrator {
    pub fn new(prob: ProbabilisticModule, det: DeterministicModule, router: NeuroSymbolicRouter) -> Self {
        Self { prob_module: prob, det_module: det, router }
    }

    /// Process a query and return a boxed stream of token strings.
    pub async fn process_query(&self, query: &str) -> BoxStream<'static, String> {
        match self.router.classify_intent(query) {
            Intent::Creative => {
                // Stream LLM tokens
                let s = self.prob_module.stream_tokens(query).await;
                s.map(|t| t).boxed()
            }
            Intent::Logical => {
                let res = self.det_module.execute_logic(query).unwrap_or_else(|e| format!("[error] {}", e));
                stream::once(async move { res }).boxed()
            }
            Intent::Hybrid => {
                // Draft from LLM streamed, then append deterministic verification result
                let llm_stream = self.prob_module.stream_tokens(query).await;
                let draft_full = self.prob_module.infer(query).await.unwrap_or_else(|_| "[draft]".to_string());

                // Simple claim extraction: numbers found in draft
                let claims: Vec<String> = extract_claims(&draft_full);
                let mut verification = String::new();
                for c in claims {
                    if let Ok(v) = self.det_module.execute_logic(&c) {
                        verification.push_str(&format!("Claim: {} -> {}\n", c, v));
                    }
                }

                let verification_stream = stream::once(async move { format!("\n[Verified]\n{}", verification) });

                // Chain the LLM stream with the verification stream
                llm_stream.map(|t| t).chain(verification_stream).boxed()
            }
        }
    }
}

fn extract_claims(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\d+(?:\.\d+)?").unwrap();
    re.find_iter(text).map(|m| m.as_str().to_string()).collect()
}
