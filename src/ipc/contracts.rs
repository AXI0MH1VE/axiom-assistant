use serde::{Serialize, Deserialize};

/// Contract definitions for inter-process communication (IPC).
/// 
/// These structures define the data formats exchanged between
/// the frontend (UI) and backend (Rust orchestrator).

/// Routing decision made by the neuro-symbolic router
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoutingDecision {
    /// The classified intent (creative, logical, hybrid)
    pub intent: String,
    /// Modules to be invoked for this query
    pub modules: Vec<String>,
    /// Strategy for merging results from multiple modules
    pub merge_strategy: String,
}

impl RoutingDecision {
    /// Create a new routing decision
    pub fn new(intent: impl Into<String>) -> Self {
        RoutingDecision {
            intent: intent.into(),
            modules: Vec::new(),
            merge_strategy: String::from("sequential"),
        }
    }

    /// Add a module to the execution plan
    pub fn add_module(&mut self, module: impl Into<String>) {
        self.modules.push(module.into());
    }

    /// Set the merge strategy
    pub fn set_merge_strategy(&mut self, strategy: impl Into<String>) {
        self.merge_strategy = strategy.into();
    }
}
