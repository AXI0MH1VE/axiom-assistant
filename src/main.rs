//! Axiom Assistant - Production-ready hybrid intelligence system
//!
//! This is the main CLI entry point for Axiom Assistant, demonstrating
//! the orchestration of probabilistic (LLM), deterministic (logic/math),
//! and neuro-symbolic routing capabilities.

mod modules;
mod engine;
mod ipc;

use modules::{ProbabilisticModule, DeterministicModule, NeuroSymbolicRouter};
use ipc::orchestrator::Orchestrator;
use futures::StreamExt;
use std::io::Write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║          Axiom Assistant - CLI Interface             ║");
    println!("║  Hybrid Intelligence: LLM + Logic + Verification     ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // Initialize modules with error handling
    println!("⚙️  Initializing modules...");
    
    let prob = match ProbabilisticModule::load_local_llm().await {
        Ok(module) => {
            println!("✅ Probabilistic module loaded");
            module
        }
        Err(e) => {
            eprintln!("❌ Failed to load probabilistic module: {}", e);
            return Err(e);
        }
    };

    let det = match DeterministicModule::init_deterministic_module() {
        Ok(module) => {
            println!("✅ Deterministic module initialized");
            module
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize deterministic module: {}", e);
            return Err(e);
        }
    };

    let router = NeuroSymbolicRouter::new();
    println!("✅ Neuro-symbolic router ready");

    let orchestrator = Orchestrator::new(prob, det, router);
    println!("✅ Orchestrator initialized\n");

    println!("📝 Type your query and press Enter. Press Ctrl+C to exit.\n");
    println!("Examples:");
    println!("  - 'Calculate 25 * 4 + 100'");
    println!("  - 'Explain quantum entanglement'");
    println!("  - 'Prove that ancestor(zeus, hercules)'\n");
    println!("{}", "─".repeat(60));

    // Read input from stdin
    use tokio::io::{AsyncBufReadExt, BufReader};
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        let query = line.trim();
        
        if query.is_empty() {
            continue;
        }

        // Handle special commands
        if query.eq_ignore_ascii_case("exit") || query.eq_ignore_ascii_case("quit") {
            println!("\n👋 Goodbye!");
            break;
        }

        if query.eq_ignore_ascii_case("help") {
            print_help();
            continue;
        }

        println!("\n🔍 Query: {}", query);
        println!("{}", "─".repeat(60));

        // Process query and stream response
        match process_query_safely(&orchestrator, query).await {
            Ok(mut stream) => {
                while let Some(token) = stream.next().await {
                    print!("{}", token);
                    std::io::stdout().flush().ok();
                }
                println!("\n{}", "─".repeat(60));
                println!("✓ Response complete\n");
            }
            Err(e) => {
                eprintln!("❌ Error processing query: {}\n", e);
            }
        }
    }

    Ok(())
}

/// Process a query with error handling
async fn process_query_safely(
    orchestrator: &Orchestrator,
    query: &str,
) -> anyhow::Result<futures::stream::BoxStream<'static, String>> {
    let stream = orchestrator.process_query(query).await;
    Ok(stream)
}

/// Print help information
fn print_help() {
    println!("\n{}", "═".repeat(60));
    println!("  Axiom Assistant - Help");
    println!("{}", "═".repeat(60));
    println!("\nCommands:");
    println!("  help     - Show this help message");
    println!("  exit     - Exit the application");
    println!("  quit     - Exit the application");
    println!("\nQuery Types:");
    println!("  • Creative - Questions requiring LLM reasoning");
    println!("    Example: 'Explain how photosynthesis works'");
    println!("\n  • Logical - Math and logic operations");
    println!("    Example: 'Calculate 42 * 7 + 15'");
    println!("    Example: 'Prove ancestor(zeus, hercules)'");
    println!("\n  • Hybrid - Combines LLM + verification");
    println!("    Example: 'Calculate the area of circle with radius 5'");
    println!("\n{}", "═".repeat(60));
    println!();
}
