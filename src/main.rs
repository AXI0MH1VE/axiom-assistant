//! Axiom Assistant - Production-ready neuro-symbolic AI assistant
//!
//! This application combines probabilistic (LLM) and deterministic (logic/math) reasoning
//! with a local-first, zero-egress architecture for secure AI processing.

mod modules;
mod engine;
mod ipc;

use modules::{ProbabilisticModule, DeterministicModule, NeuroSymbolicRouter};
use ipc::orchestrator::Orchestrator;
use futures::StreamExt;
use tokio::io::AsyncBufReadExt;

/// Initialize logging with environment-based configuration
fn init_logging() {
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string());
    
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(log_level)
    )
    .format_timestamp_millis()
    .init();
    
    log::info!("Logging initialized");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();
    
    log::info!("=== Axiom Assistant v{} ===", env!("CARGO_PKG_VERSION"));
    log::info!("Starting CLI interface with production modules");

    // Initialize modules with error handling
    let prob = match ProbabilisticModule::load_local_llm().await {
        Ok(m) => {
            log::info!("✓ Probabilistic module loaded");
            m
        }
        Err(e) => {
            log::error!("✗ Failed to load probabilistic module: {}", e);
            return Err(e.into());
        }
    };
    
    let det = match DeterministicModule::init_deterministic_module() {
        Ok(m) => {
            log::info!("✓ Deterministic module loaded");
            m
        }
        Err(e) => {
            log::error!("✗ Failed to initialize deterministic module: {}", e);
            return Err(e.into());
        }
    };
    
    let router = NeuroSymbolicRouter::new();
    log::info!("✓ Neuro-symbolic router initialized");

    let orchestrator = Orchestrator::new(prob, det, router);
    log::info!("✓ Orchestrator ready");

    println!("\n🤖 Axiom Assistant is ready!");
    println!("📝 Type your query and press Enter");
    println!("🔧 Commands: 'stats' (show statistics), 'help' (show help), Ctrl+C (exit)\n");

    let stdin = tokio::io::stdin();
    let reader = tokio::io::BufReader::new(stdin);
    let mut lines = reader.lines();

    loop {
        print!("> ");
        use std::io::Write;
        std::io::stdout().flush().ok();

        match lines.next_line().await {
            Ok(Some(line)) => {
                let trimmed = line.trim();
                
                if trimmed.is_empty() {
                    continue;
                }
                
                // Handle special commands
                match trimmed.to_lowercase().as_str() {
                    "exit" | "quit" => {
                        log::info!("User requested exit");
                        break;
                    }
                    "stats" => {
                        let stats = orchestrator.get_stats();
                        println!("\n📊 Statistics:");
                        println!("  Total queries: {}", stats.queries_processed);
                        println!("  Creative: {}", stats.creative_queries);
                        println!("  Logical: {}", stats.logical_queries);
                        println!("  Hybrid: {}", stats.hybrid_queries);
                        println!();
                        continue;
                    }
                    "help" => {
                        println!("\n📖 Help:");
                        println!("  - Type any question or command");
                        println!("  - Math queries: '2 + 2', 'sqrt(16)'");
                        println!("  - Logic queries: 'ancestor(zeus, hercules)'");
                        println!("  - Creative queries: 'explain quantum physics'");
                        println!("  - 'stats' - Show processing statistics");
                        println!("  - 'exit' or Ctrl+C - Exit the application");
                        println!();
                        continue;
                    }
                    _ => {}
                }
                
                log::info!("Processing query: {}", trimmed);

                let mut stream = orchestrator.process_query(trimmed).await;
                let mut response_chars = 0;

                while let Some(token) = stream.next().await {
                    print!("{}", token);
                    response_chars += token.len();
                    std::io::stdout().flush().ok();
                }

                log::debug!("Response complete: {} characters", response_chars);
                println!("\n");
            }
            Ok(None) => {
                log::info!("EOF reached");
                break;
            }
            Err(e) => {
                log::error!("Error reading input: {}", e);
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    log::info!("Axiom Assistant shutting down");
    println!("👋 Goodbye!");
    
    Ok(())
}
