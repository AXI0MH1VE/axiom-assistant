//! Axiom Assistant - starter skeleton based on provided blueprint.
//!
//! This file contains a minimal `main` and wiring points for the orchestrator,
//! modules, and the Tauri integration. Fill in implementations in the module
//! files under `src/modules`, `src/engine`, and `src/ipc`.

mod modules;
mod engine;
mod ipc;

use modules::{ProbabilisticModule, DeterministicModule, NeuroSymbolicRouter};
use ipc::orchestrator::Orchestrator;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("[AxiomAssistant] Demo CLI starting (mock implementations)");

    // Initialize mock modules
    let prob = ProbabilisticModule::load_local_llm().await.expect("load llm");
    let det = DeterministicModule::init_deterministic_module().expect("init det");
    let router = NeuroSymbolicRouter::new();

    let orchestrator = Orchestrator::new(prob, det, router);

    println!("Type a query and press Enter. Ctrl+C to exit.");

    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin).lines();

    while let Some(Ok(line)) = reader.next_line().await {
        if line.trim().is_empty() { continue; }

        let mut stream = orchestrator.process_query(&line).await;

        while let Some(token) = stream.next().await {
            print!("{}", token);
            use std::io::Write;
            std::io::stdout().flush().ok();
        }

        println!("\n--- response complete ---\n");
    }
}
