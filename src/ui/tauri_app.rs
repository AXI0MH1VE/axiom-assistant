use tauri::{AppHandle, Manager, Emitter};
use serde::{Serialize, Deserialize};
use crate::ipc::orchestrator::Orchestrator;
use crate::modules::{ProbabilisticModule, DeterministicModule, NeuroSymbolicRouter};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
struct QueryRequest {
    query: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct CompletionResponse {
    completed: bool,
}

/// Initialize Tauri application with command handlers (backward-compatible version)
pub fn init_tauri() {
    try_init_tauri().unwrap();
}

/// Initialize Tauri application with command handlers (returns Result)
pub fn try_init_tauri() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize orchestrator and store in app state
            let runtime = tokio::runtime::Runtime::new()?;
            let orchestrator = runtime.block_on(async {
                let prob = ProbabilisticModule::load_local_llm()
                    .await
                    .expect("Failed to load probabilistic module");
                let det = DeterministicModule::init_deterministic_module()
                    .expect("Failed to initialize deterministic module");
                let router = NeuroSymbolicRouter::new();
                Arc::new(Mutex::new(Orchestrator::new(prob, det, router)))
            });

            app.manage(orchestrator);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            process_query,
            health_check,
            get_system_status
        ])
        .run(tauri::generate_context!())
        .expect("Error running Tauri application");

    Ok(())
}

/// Tauri command to process a user query
#[tauri::command]
async fn process_query(
    query: String,
    app: AppHandle,
    orchestrator: tauri::State<'_, Arc<Mutex<Orchestrator>>>,
) -> Result<String, String> {
    let orch = orchestrator.lock().await;
    let mut stream = orch.process_query(&query).await;

    // Spawn task to emit tokens as they arrive
    tokio::spawn(async move {
        while let Some(token) = stream.next().await {
            let _ = app.emit("query-token", TokenResponse { token });
        }
        let _ = app.emit("query-complete", CompletionResponse { completed: true });
    });

    Ok("Processing started".to_string())
}

/// Health check endpoint for Tauri
#[tauri::command]
async fn health_check() -> Result<String, String> {
    Ok("Axiom Assistant is running".to_string())
}

/// Get system status information
#[tauri::command]
async fn get_system_status() -> Result<SystemStatus, String> {
    Ok(SystemStatus {
        modules_loaded: true,
        orchestrator_ready: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[derive(Serialize)]
struct SystemStatus {
    modules_loaded: bool,
    orchestrator_ready: bool,
    version: String,
}
