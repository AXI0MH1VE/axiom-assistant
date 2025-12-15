//! Integration tests for Axiom Assistant modules

use axiom_assistant::modules::{
    DeterministicModule, NeuroSymbolicRouter, ProbabilisticModule,
};
use axiom_assistant::ipc::orchestrator::Orchestrator;
use futures::StreamExt;

#[tokio::test]
async fn test_probabilistic_module_initialization() {
    let result = ProbabilisticModule::load_local_llm().await;
    assert!(result.is_ok(), "Probabilistic module should initialize successfully");
}

#[tokio::test]
async fn test_deterministic_module_initialization() {
    let result = DeterministicModule::init_deterministic_module();
    assert!(result.is_ok(), "Deterministic module should initialize successfully");
}

#[test]
fn test_router_classification() {
    let router = NeuroSymbolicRouter::new();
    
    // Test logical intent
    let intent = router.classify_intent("Calculate 2 + 2");
    assert_eq!(intent, axiom_assistant::modules::neuro_symbolic::Intent::Logical);
    
    // Test creative intent
    let intent = router.classify_intent("Explain quantum physics");
    assert_eq!(intent, axiom_assistant::modules::neuro_symbolic::Intent::Creative);
}

#[tokio::test]
async fn test_deterministic_math_evaluation() {
    let module = DeterministicModule::init_deterministic_module()
        .expect("Failed to initialize deterministic module");
    
    // Test basic arithmetic
    let result = module.execute_logic("2 + 2").expect("Math evaluation failed");
    assert_eq!(result, "4");
    
    // Test multiplication
    let result = module.execute_logic("5 * 10").expect("Math evaluation failed");
    assert_eq!(result, "50");
}

#[tokio::test]
async fn test_orchestrator_initialization() {
    let prob = ProbabilisticModule::load_local_llm().await
        .expect("Failed to load probabilistic module");
    let det = DeterministicModule::init_deterministic_module()
        .expect("Failed to initialize deterministic module");
    let router = NeuroSymbolicRouter::new();
    
    let _orchestrator = Orchestrator::new(prob, det, router);
    // If we get here without panic, initialization succeeded
}

#[tokio::test]
async fn test_orchestrator_logical_query() {
    let prob = ProbabilisticModule::load_local_llm().await.unwrap();
    let det = DeterministicModule::init_deterministic_module().unwrap();
    let router = NeuroSymbolicRouter::new();
    let orchestrator = Orchestrator::new(prob, det, router);
    
    let mut stream = orchestrator.process_query("Calculate 10 + 5").await;
    let result = stream.next().await;
    
    assert!(result.is_some(), "Should return a result");
    assert!(result.unwrap().contains("15"), "Should calculate correctly");
}

#[tokio::test]
async fn test_orchestrator_creative_query() {
    let prob = ProbabilisticModule::load_local_llm().await.unwrap();
    let det = DeterministicModule::init_deterministic_module().unwrap();
    let router = NeuroSymbolicRouter::new();
    let orchestrator = Orchestrator::new(prob, det, router);
    
    let mut stream = orchestrator.process_query("Explain something").await;
    let mut token_count = 0;
    
    while let Some(_token) = stream.next().await {
        token_count += 1;
        if token_count > 100 {
            break; // Prevent infinite loop
        }
    }
    
    assert!(token_count > 0, "Should stream at least one token");
}

#[tokio::test]
async fn test_token_streaming() {
    let module = ProbabilisticModule::load_local_llm().await.unwrap();
    let mut stream = module.stream_tokens("Hello world").await;
    
    let mut tokens = Vec::new();
    while let Some(token) = stream.next().await {
        tokens.push(token);
        if tokens.len() > 10 {
            break; // Prevent infinite loop
        }
    }
    
    assert!(!tokens.is_empty(), "Should produce tokens");
}
