// src/lib.rs

//! # Will's Fuzzy Algorithm Library
//! 
//! `will_fuzzy` is a quite fast, minimal allocation high efficienty fuzzy matching engine and
//! psudo-TUI framework.
//! This library provides the core search algorithms and session management required to 
//! build custom fuzzy finders. additionally, by following the trait patterns defined, custom
//! algorithms could be inserted as long as they match the required functions!
//! it additionally defines structures to allow for any struct to implement a fuzzy search across
//! it, allowing weighting of different fields as well!
//!
//!
//! flow is as follows:
//!  
pub mod app;
pub mod fuzzy;
pub use fuzzy::canidate::{FuzzyCandidate, ScoreTarget, ScoredResult};
pub use fuzzy::session::SearchSession;
pub use fuzzy::matcher::FuzzyMatcher;
pub use fuzzy::algorithms::algo_greedy_v2::AlgoWillGreedyVer2;


