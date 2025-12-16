pub struct NeuroSymbolicRouter {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Intent {
    Creative,
    Logical,
    Hybrid,
}

impl NeuroSymbolicRouter {
    pub fn new() -> Self {
        NeuroSymbolicRouter {}
    }

    pub fn classify_intent(&self, query: &str) -> Intent {
        let query_lower = query.to_lowercase();
        let math_keywords = ["calculate", "solve", "prove", "=", "+", "-"];
        let creative_keywords = ["write", "suggest", "explain", "describe"];
        let has_math = math_keywords.iter().any(|&kw| query_lower.contains(kw));
        let has_creative = creative_keywords.iter().any(|&kw| query_lower.contains(kw));
        match (has_math, has_creative) {
            (true, false) => Intent::Logical,
            (false, true) => Intent::Creative,
            _ => Intent::Hybrid,
        }
    }
}
