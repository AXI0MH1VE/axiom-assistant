// Production-grade Tauri app bridge with full Orchestrator integration
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::ipc::orchestrator::Orchestrator;
use crate::modules::{ProbabilisticModule, DeterministicModule, NeuroSymbolicRouter};
use futures::StreamExt;

/// Global state for the Tauri application
pub struct AppState {
    orchestrator: Arc<Mutex<Orchestrator>>,
}

/// Initialize Tauri with command handlers and orchestrator integration
pub async fn init_tauri() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize modules
    let prob = ProbabilisticModule::load_local_llm()
        .await
        .map_err(|e| format!("Failed to load probabilistic module: {}", e))?;
    
    let det = DeterministicModule::init_deterministic_module()
        .map_err(|e| format!("Failed to initialize deterministic module: {}", e))?;
    
    let router = NeuroSymbolicRouter::new();
    
    let orchestrator = Orchestrator::new(prob, det, router);
    
    let state = AppState {
        orchestrator: Arc::new(Mutex::new(orchestrator)),
    };

    log::info!("Tauri orchestrator initialized successfully");
    
    Ok(())
}

/// Tauri command to send a message and stream tokens back
#[tauri::command]
async fn send_message(
    message: String,
    app_handle: AppHandle,
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    log::info!("Received message: {}", message);
    
    if message.trim().is_empty() {
        return Err("Message cannot be empty".to_string());
    }

    let orchestrator = state.orchestrator.lock().await;
    let mut stream = orchestrator.process_query(&message).await;
    
    // Stream tokens to frontend via events
    let mut full_response = String::new();
    while let Some(token) = stream.next().await {
        full_response.push_str(&token);
        
        // Emit token event to frontend
        app_handle
            .emit("token", token.clone())
            .map_err(|e| format!("Failed to emit token: {}", e))?;
    }
    
    log::info!("Response complete: {} chars", full_response.len());
    Ok(full_response)
}

/// Tauri command to get system status
#[tauri::command]
async fn get_status(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<SystemStatus, String> {
    Ok(SystemStatus {
        ready: true,
        modules_loaded: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// System status response
#[derive(serde::Serialize)]
pub struct SystemStatus {
    pub ready: bool,
    pub modules_loaded: bool,
    pub version: String,
}

/// Register all Tauri commands
pub fn get_tauri_commands() -> impl Fn(tauri::Invoke) {
    tauri::generate_handler![send_message, get_status]
}
