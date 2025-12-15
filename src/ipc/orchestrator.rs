use futures::{stream, StreamExt, stream::BoxStream};
use crate::modules::probabilistic::ProbabilisticModule;
use crate::modules::deterministic::DeterministicModule;
use crate::modules::neuro_symbolic::{NeuroSymbolicRouter, Intent};

/// Orchestrator coordinates the three main modules to process user queries.
/// 
/// The orchestrator:
/// 1. Uses the router to classify query intent
/// 2. Delegates to appropriate module(s)
/// 3. Streams results back to the user
/// 4. Performs verification when applicable (Hybrid mode)
/// 
/// This is the main entry point for query processing.
pub struct Orchestrator {
    /// Probabilistic/LLM module for creative reasoning
    pub prob_module: ProbabilisticModule,
    /// Deterministic module for logic and math
    pub det_module: DeterministicModule,
    /// Router for intent classification
    pub router: NeuroSymbolicRouter,
}

impl Orchestrator {
    /// Create a new orchestrator with initialized modules
    /// 
    /// # Arguments
    /// 
    /// * `prob` - Initialized probabilistic module
    /// * `det` - Initialized deterministic module
    /// * `router` - Intent classification router
    pub fn new(
        prob: ProbabilisticModule,
        det: DeterministicModule,
        router: NeuroSymbolicRouter
    ) -> Self {
        Self {
            prob_module: prob,
            det_module: det,
            router,
        }
    }

    /// Process a user query and return a stream of response tokens.
    /// 
    /// The processing flow:
    /// 1. Classify intent (Creative, Logical, or Hybrid)
    /// 2. Route to appropriate module(s)
    /// 3. Stream results back incrementally
    /// 4. For Hybrid: verify LLM output with deterministic checks
    /// 
    /// # Arguments
    /// 
    /// * `query` - The user's input query
    /// 
    /// # Returns
    /// 
    /// A `BoxStream` that emits response tokens as strings
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use futures::StreamExt;
    /// 
    /// async fn process(orchestrator: &Orchestrator) {
    ///     let mut stream = orchestrator.process_query("Calculate 2 + 2").await;
    ///     while let Some(token) = stream.next().await {
    ///         print!("{}", token);
    ///     }
    /// }
    /// ```
    pub async fn process_query(&self, query: &str) -> BoxStream<'static, String> {
        let intent = self.router.classify_intent(query);
        
        match intent {
            Intent::Creative => {
                // Pure LLM generation with token streaming
                let stream = self.prob_module.stream_tokens(query).await;
                stream.boxed()
            }
            
            Intent::Logical => {
                // Deterministic evaluation (math/logic)
                let result = self.det_module
                    .execute_logic(query)
                    .unwrap_or_else(|e| format!("[Error] {}", e));
                stream::once(async move { result }).boxed()
            }
            
            Intent::Hybrid => {
                // Hybrid: LLM generation + deterministic verification
                self.process_hybrid_query(query).await
            }
        }
    }

    /// Process a hybrid query requiring both LLM and verification
    async fn process_hybrid_query(&self, query: &str) -> BoxStream<'static, String> {
        // Stream LLM tokens
        let llm_stream = self.prob_module.stream_tokens(query).await;
        
        // Get full response for verification
        let draft_full = self.prob_module
            .infer(query)
            .await
            .unwrap_or_else(|_| "[Draft generation failed]".to_string());

        // Extract verifiable claims (numbers, expressions)
        let claims = extract_claims(&draft_full);
        
        // Build verification results
        let mut verification = String::new();
        if !claims.is_empty() {
            verification.push_str("\n\n[Verification]\n");
            for claim in claims {
                match self.det_module.execute_logic(&claim) {
                    Ok(v) => {
                        verification.push_str(&format!("✓ {} = {}\n", claim, v));
                    }
                    Err(e) => {
                        verification.push_str(&format!("✗ {}: {}\n", claim, e));
                    }
                }
            }
        }

        // Create verification stream
        let verification_stream = stream::once(async move { verification });

        // Chain LLM stream with verification stream
        llm_stream.chain(verification_stream).boxed()
    }
}

/// Extract numerical claims from text for verification.
/// 
/// Finds expressions and numbers that can be verified deterministically.
/// 
/// # Arguments
/// 
/// * `text` - The text to extract claims from
/// 
/// # Returns
/// 
/// Vector of claim strings (expressions or numbers)
fn extract_claims(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*([+\-*/])\s*(\d+(?:\.\d+)?)")
        .unwrap();
    
    let mut claims = Vec::new();
    
    // Extract mathematical expressions
    for cap in re.captures_iter(text) {
        if let Some(expr) = cap.get(0) {
            claims.push(expr.as_str().to_string());
        }
    }
    
    // If no expressions found, extract standalone numbers for reference
    if claims.is_empty() {
        let num_re = regex::Regex::new(r"\d+(?:\.\d+)?").unwrap();
        claims = num_re
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .take(3) // Limit to first 3 numbers
            .collect();
    }
    
    claims
}
