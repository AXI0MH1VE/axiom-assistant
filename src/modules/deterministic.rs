use serde::{Serialize, Deserialize};
use evalexpr::*;

/// Deterministic module implementing math evaluation and a few mocked logic responses.
pub struct DeterministicModule {}

impl DeterministicModule {
    pub fn init_deterministic_module() -> anyhow::Result<Self> {
        // In production load SWI-Prolog and rules. For now return mock.
        Ok(DeterministicModule {})
    }

    /// Execute a logic/math query. For `math` queries evaluate the expression using `evalexpr`.
    /// For `logic` queries return simple canned proofs for demonstration.
    pub fn execute_logic(&self, query: &str) -> anyhow::Result<String> {
        // Heuristic: if the query looks like a math expression, evaluate it.
        if looks_like_math(query) {
            match eval_float(query) {
                Ok(v) => return Ok(format!("{}", v)),
                Err(e) => return Err(anyhow::anyhow!(e)),
            }
        }

        // Very small mock Prolog-like responses
        if query.contains("ancestor") {
            let proof = vec![
                "ancestor(zeus, hercules) :- parent(zeus, hercules).".to_string(),
                "parent(zeus, hercules).".to_string(),
                "Therefore ancestor(zeus, hercules).".to_string(),
            ];
            return Ok(proof.join("\n"));
        }

        Ok("[deterministic: no action matched]".to_string())
    }
}

fn looks_like_math(s: &str) -> bool {
    let math_chars = ['+', '-', '*', '/', '^', '%'];
    s.chars().any(|c| math_chars.contains(&c)) || s.trim().chars().all(|c| c.is_digit(10) || c.is_whitespace() || "().".contains(c))
}

#[derive(Serialize, Deserialize)]
pub struct DetRequest {
    pub query_type: QueryType,
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub enum QueryType {
    Math,
    Logic,
    Code,
}

#[derive(Serialize, Deserialize)]
pub struct DetResponse {
    pub result: String,
    pub proof: Option<Vec<String>>,
    pub deterministic: bool,
}
