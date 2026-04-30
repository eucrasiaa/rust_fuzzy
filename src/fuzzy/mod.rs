//! # Fuzzy Algorithms, Session Engine
//!
//! The `fuzzy` module is the heart & soul of the library's search capabilities, 
//! - algoritms!
//! - matching logic!
//! - session state stuff!
//!
//! - **`FuzzyCandidate`**: if implemented, allows for passing to a session!
//! - **`SearchSession`**: state controller(?) manager(?) the thing handling the algorithm.
//!     - history and current query management
//!     - letter by letter parsing
//!     - flat string querying
//!     - results polling
//!     - interfaces for the tui or other handlers if designed!
//!     - threshold for multi character type culling for optimization
pub mod algorithm;
pub mod session;
pub mod canidate;
pub mod matcher;
pub mod algorithms;
// pub mod prelude {
// pub use self::algorithm::*;
pub use self::canidate::*;
pub use self::session::*;
pub use self::matcher::*;
pub use self::algorithms::SimilarityAlgorithm;
pub use self::algorithms::*;
pub use self::canidate::ScoreTarget;

/// the way weights are handled, to optimize calcs, is that its ints scaled as a 1024 and bit
/// shifted. so be sure to use handlers! 
#[inline(always)]
pub fn scale_weight(f: f64) -> i64 {
    (f * 1024.0).round() as i64
}
// pub use self::algorithm::*;
    // Add other common traits/structs here
// }
// pub use self::SimilarityAlgorithm;
//

pub trait MatchReporter {
    fn on_step(&mut self, target: &str, current_idx: usize, query_char: char, matched: bool, score_diff: i64);
}

#[cfg(feature = "logging")]
#[derive(Debug, Clone)]
pub struct StepSnapshot {
    pub target_index: usize,
    pub query_index: usize,
    pub t_byte: u8,
    pub q_byte: u8,
    pub prev_match_index: usize,
    pub amt_consec: i64,
    pub score: i64,
    pub is_match: bool,
}
#[cfg(feature = "logging")]
pub struct TraceReporter<'a> {
    pub target: &'a [u8],
    pub query: &'a [u8],
    pub steps: Vec<StepSnapshot>,
}

#[cfg(feature = "logging")]
impl<'a> TraceReporter<'a> {
    pub fn new(target: &'a [u8], query: &'a [u8]) -> Self {
        Self { target, query, steps: Vec::new() }
    }

    pub fn record(&mut self, snap: StepSnapshot) {
        self.steps.push(snap);
    }

    pub fn print_trace(&self, final_score: i64) {
        let t_str = String::from_utf8_lossy(self.target);
        let q_str = String::from_utf8_lossy(self.query);
        
        println!("\n\x1b[1;36m=== Match Trace ===\x1b[0m");
        println!("Target: '{}'", t_str);
        println!("Query:  '{}'", q_str);
        println!("-------------------");

        for (i, step) in self.steps.iter().enumerate() {
            let match_color = if step.is_match { "\x1b[1;32m" } else { "\x1b[1;31m" };
            println!("Step {:02}: prev_idx={}, consec={}, score={}, t_idx={}, q_idx={}", 
                i, step.prev_match_index, step.amt_consec, step.score, step.target_index, step.query_index);
            
            let mut q_vis = String::new();
            for (qi, &b) in self.query.iter().enumerate() {
                if qi == step.query_index {
                    q_vis.push_str(&format!("{}\x1b[4m[{}]\x1b[0m", match_color, b as char));
                } else {
                    q_vis.push(b as char);
                }
            }
            let mut t_vis = String::new();
            for (ti, &b) in self.target.iter().enumerate() {
                if ti == step.target_index {
                    t_vis.push_str(&format!("{}\x1b[1m[{}]\x1b[0m", match_color, b as char));
                } else {
                    t_vis.push(b as char);
                }
            }

            println!("   Q: {}", q_vis);
            println!("   T: {}\n", t_vis);
        }
        println!("\x1b[1;32mFinal Score: {}\x1b[0m\n", final_score);
    }
}
