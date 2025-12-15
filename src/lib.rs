//! Axiom Assistant - Production-ready hybrid intelligence system
//! 
//! This library provides the core modules for combining probabilistic AI (LLM)
//! with deterministic symbolic reasoning.

pub mod modules;
pub mod engine;
pub mod ipc;

pub use modules::{ProbabilisticModule, DeterministicModule, NeuroSymbolicRouter};
pub use ipc::orchestrator::Orchestrator;
pub use engine::{AxiomEngine, Scene};
