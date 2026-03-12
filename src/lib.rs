//! # Reflex Engine SDK
//!
//! Local-first deterministic geospatial evaluation with replayable audit artifacts.
//!
//! ## Features
//!
//! - **Deterministic Evaluation**: Same input always produces same output
//! - **Policy-Based Rules**: Configurable spatial guardrails and constraints
//! - **Audit Artifacts**: Tamper-evident JSON records for every decision
//! - **Local-First**: No external dependencies for core evaluation logic
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use reflex_engine::{Event, Policy, evaluate_event};
//!
//! fn main() -> anyhow::Result<()> {
//!     let policy = Policy::load("policy.json")?;
//!     let event = Event::load("event.json")?;
//!     let decision = evaluate_event(&event, &policy);
//!     let _ = decision;
//!     Ok(())
//! }
//! ```

pub mod artifact;
pub mod model;
pub mod validator;

pub use artifact::*;
pub use model::*;
pub use validator::*;
