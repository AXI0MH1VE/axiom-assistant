/// NeuroSymbolicRouter intelligently routes queries to appropriate processing modules.
/// 
/// This router analyzes the intent of user queries and determines whether to:
/// - Use probabilistic/LLM reasoning (Creative)
/// - Use deterministic logic/math (Logical)
/// - Combine both approaches (Hybrid)
/// 
/// The routing decision ensures optimal processing while maintaining
/// verification and reproducibility where possible.
pub struct NeuroSymbolicRouter {}

/// Intent classification for query routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Intent {
    /// Requires creative, open-ended reasoning (LLM)
    Creative,
    /// Requires deterministic logic or math
    Logical,
    /// Benefits from both LLM generation and logical verification
    Hybrid,
}

impl NeuroSymbolicRouter {
    /// Create a new router instance
    pub fn new() -> Self {
        NeuroSymbolicRouter {}
    }

    /// Classify the intent of a user query.
    /// 
    /// Analyzes keywords and patterns to determine the best processing approach.
    /// 
    /// # Arguments
    /// 
    /// * `query` - The user's input query
    /// 
    /// # Returns
    /// 
    /// An `Intent` classification:
    /// - `Creative`: Open-ended questions, explanations, creative writing
    /// - `Logical`: Math, logic proofs, deterministic computation
    /// - `Hybrid`: Claims requiring verification, mixed reasoning
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// let router = NeuroSymbolicRouter::new();
    /// 
    /// assert_eq!(router.classify_intent("Calculate 2 + 2"), Intent::Logical);
    /// assert_eq!(router.classify_intent("Explain quantum physics"), Intent::Creative);
    /// assert_eq!(router.classify_intent("What is 10% of 500?"), Intent::Hybrid);
    /// ```
    pub fn classify_intent(&self, query: &str) -> Intent {
        let query_lower = query.to_lowercase();
        
        // Keywords indicating mathematical/logical operations
        let math_keywords = [
            "calculate", "solve", "prove", "verify", "compute",
            "=", "+", "-", "*", "/", "^", "%", "math", "equation"
        ];
        
        // Keywords indicating creative/explanatory requests
        let creative_keywords = [
            "write", "suggest", "explain", "describe", "why",
            "how does", "what is", "tell me about", "story", "poem"
        ];
        
        // Count keyword matches
        let has_math = math_keywords.iter()
            .any(|&kw| query_lower.contains(kw));
        let has_creative = creative_keywords.iter()
            .any(|&kw| query_lower.contains(kw));
        
        // Classify based on keyword presence
        match (has_math, has_creative) {
            (true, false) => Intent::Logical,
            (false, true) => Intent::Creative,
            (true, true) => Intent::Hybrid,  // Both aspects present
            (false, false) => {
                // Default heuristic: check for numbers
                if query.chars().any(|c| c.is_ascii_digit()) {
                    Intent::Hybrid
                } else {
                    Intent::Creative
                }
            }
        }
    }

    /// Get a human-readable description of an intent
    pub fn describe_intent(&self, intent: Intent) -> &'static str {
        match intent {
            Intent::Creative => "Creative reasoning (LLM)",
            Intent::Logical => "Logical/mathematical (deterministic)",
            Intent::Hybrid => "Hybrid (LLM + verification)",
        }
    }
}

impl Default for NeuroSymbolicRouter {
    fn default() -> Self {
        Self::new()
    }
}
