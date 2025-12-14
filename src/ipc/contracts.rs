use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RoutingDecision {
    pub intent: String,
    pub modules: Vec<String>,
    pub merge_strategy: String,
}
