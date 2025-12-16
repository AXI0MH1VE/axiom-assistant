use futures::{stream, StreamExt, stream::BoxStream};
use crate::modules::probabilistic::ProbabilisticModule;
use crate::modules::deterministic::DeterministicModule;
use crate::modules::neuro_symbolic::{NeuroSymbolicRouter, Intent};

/// Production-grade orchestrator with comprehensive error handling and logging
pub struct Orchestrator {
    pub prob_module: ProbabilisticModule,
    pub det_module: DeterministicModule,
    pub router: NeuroSymbolicRouter,
    pub stats: OrchestratorStats,
}

#[derive(Default)]
pub struct OrchestratorStats {
    pub queries_processed: std::sync::atomic::AtomicU64,
    pub creative_queries: std::sync::atomic::AtomicU64,
    pub logical_queries: std::sync::atomic::AtomicU64,
    pub hybrid_queries: std::sync::atomic::AtomicU64,
}

impl Orchestrator {
    pub fn new(prob: ProbabilisticModule, det: DeterministicModule, router: NeuroSymbolicRouter) -> Self {
        log::info!("Orchestrator initialized");
        Self { 
            prob_module: prob, 
            det_module: det, 
            router,
            stats: OrchestratorStats::default(),
        }
    }

    /// Process a query and return a boxed stream of token strings
    /// Implements neuro-symbolic routing with full error recovery
    pub async fn process_query(&self, query: &str) -> BoxStream<'static, String> {
        if query.is_empty() {
            log::warn!("Empty query received");
            return stream::once(async { "[error] Query cannot be empty".to_string() }).boxed();
        }
        
        if query.len() > 50000 {
            log::warn!("Query too long: {} chars", query.len());
            return stream::once(async { "[error] Query exceeds maximum length".to_string() }).boxed();
        }
        
        // Classify intent
        let intent = self.router.classify_intent(query);
        log::info!("Query classified as: {:?}", intent);
        
        // Update statistics
        self.stats.queries_processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        match intent {
            Intent::Creative => {
                self.stats.creative_queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.handle_creative(query).await
            }
            Intent::Logical => {
                self.stats.logical_queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.handle_logical(query).await
            }
            Intent::Hybrid => {
                self.stats.hybrid_queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.handle_hybrid(query).await
            }
        }
    }
    
    /// Handle creative queries with LLM streaming
    async fn handle_creative(&self, query: &str) -> BoxStream<'static, String> {
        log::debug!("Processing creative query");
        let s = self.prob_module.stream_tokens(query).await;
        s.map(|t| t).boxed()
    }
    
    /// Handle logical queries with deterministic execution
    async fn handle_logical(&self, query: &str) -> BoxStream<'static, String> {
        log::debug!("Processing logical query");
        
        match self.det_module.execute_logic(query) {
            Ok(result) => {
                log::debug!("Logical query succeeded: {} chars", result.len());
                stream::once(async move { result }).boxed()
            }
            Err(e) => {
                log::error!("Logical query failed: {}", e);
                stream::once(async move { 
                    format!("[deterministic error] {}", e) 
                }).boxed()
            }
        }
    }
    
    /// Handle hybrid queries with LLM draft + deterministic verification
    async fn handle_hybrid(&self, query: &str) -> BoxStream<'static, String> {
        log::debug!("Processing hybrid query");
        
        // Get LLM stream
        let llm_stream = self.prob_module.stream_tokens(query).await;
        
        // Get full draft for verification
        let draft_result = self.prob_module.infer(query).await;
        
        match draft_result {
            Ok(draft_full) => {
                log::debug!("LLM draft generated: {} chars", draft_full.len());
                
                // Extract and verify claims
                let claims = extract_claims(&draft_full);
                log::debug!("Extracted {} claims for verification", claims.len());
                
                let mut verification = String::new();
                let mut verified_count = 0;
                let mut failed_count = 0;
                
                for claim in claims.iter() {
                    match self.det_module.execute_logic(claim) {
                        Ok(v) => {
                            verification.push_str(&format!("✓ Claim: {} → {}\n", claim, v));
                            verified_count += 1;
                        }
                        Err(e) => {
                            verification.push_str(&format!("✗ Claim: {} → Error: {}\n", claim, e));
                            failed_count += 1;
                        }
                    }
                }
                
                if verified_count > 0 || failed_count > 0 {
                    verification = format!(
                        "\n[Verification Results: {} verified, {} failed]\n{}", 
                        verified_count, failed_count, verification
                    );
                }
                
                log::debug!("Verification complete: {} verified, {} failed", verified_count, failed_count);
                
                let verification_stream = stream::once(async move { verification });
                llm_stream.map(|t| t).chain(verification_stream).boxed()
            }
            Err(e) => {
                log::error!("Failed to generate draft: {}", e);
                stream::once(async move { 
                    format!("[error] Failed to process hybrid query: {}", e) 
                }).boxed()
            }
        }
    }
    
    /// Get orchestrator statistics
    pub fn get_stats(&self) -> OrchestratorStatsSnapshot {
        OrchestratorStatsSnapshot {
            queries_processed: self.stats.queries_processed.load(std::sync::atomic::Ordering::Relaxed),
            creative_queries: self.stats.creative_queries.load(std::sync::atomic::Ordering::Relaxed),
            logical_queries: self.stats.logical_queries.load(std::sync::atomic::Ordering::Relaxed),
            hybrid_queries: self.stats.hybrid_queries.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OrchestratorStatsSnapshot {
    pub queries_processed: u64,
    pub creative_queries: u64,
    pub logical_queries: u64,
    pub hybrid_queries: u64,
}

/// Extract numerical claims from text for verification
fn extract_claims(text: &str) -> Vec<String> {
    use once_cell::sync::Lazy;
    
    static EXPR_RE: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r"\d+(?:\.\d+)?(?:\s*[+\-*/]\s*\d+(?:\.\d+)?)+").unwrap()
    });
    
    static NUM_RE: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r"\d+(?:\.\d+)?").unwrap()
    });
    
    let mut claims: Vec<String> = EXPR_RE.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect();
    
    // Also extract simple numbers as potential claims
    if claims.is_empty() {
        claims = NUM_RE.find_iter(text)
            .take(5) // Limit to avoid excessive verification
            .map(|m| m.as_str().to_string())
            .collect();
    }
    
    claims
}
