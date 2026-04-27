// pub mod algo_basic_greedy_v1;
pub mod algo_greedy_v2;
// pub use algo_basic_greedy_v1::AlgoWillBasicGreedyVer1;
pub use algo_greedy_v2::AlgoWillGreedyVer2;
use std::fmt;

/// all an algo has to implement is a scoring function
/// it will be passed the pre-modified strings from session
/// (eg: session handles exact vs just lowercase match)
pub trait SimilarityAlgorithm {
    /// Target: original static strings we search against, the canidates 
    /// Query: the user typed string we match with 
    // fn score(&self, target: &str, query: &str) -> i64;
    fn score(&self, target: &[u8], query: &[u8]) -> i64;

}

pub struct Trace {
    pub target: String,
    pub query: String,
    pub steps: Vec<String>,
    pub final_score: i64,
}
impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- Debug Trace: '{}' vs '{}' ---", self.query, self.target)?;
        for (i, step) in self.steps.iter().enumerate() {
            writeln!(f, "[{:02}] {}", i, step)?;
        }
        writeln!(f, "Final Score: \x1b[1;32m{}\x1b[0m", self.final_score)
    }
}
// debug allows for a step thru of a matching execution, where the display 
pub trait DebugAlgo:SimilarityAlgorithm{
    fn debug_score(&self, target: &str, query: &str) -> Trace;
    fn multi_score(&self, target:&str, queries:Vec<String>) -> Vec<Trace>;
}

pub trait MatchReporter {
    /// Called every iteration of the while loop
    fn on_step(&mut self, target: &[u8], t_idx: usize, query: &[u8], q_idx: usize, matched: bool);
    
    /// Called when a match is found to report specific bonuses
    fn on_bonus(&mut self, name: &'static str, amount: i64, current_total: i64);
    
    /// Called at the very end
    fn on_complete(&mut self, final_score: i64, fully_matched: bool);
}

impl MatchReporter for () {
    #[inline(always)] fn on_step(&mut self, _: &[u8], _: usize, _: &[u8], _: usize, _: bool) {}
    #[inline(always)] fn on_bonus(&mut self, _: &'static str, _: i64, _: i64) {}
    #[inline(always)] fn on_complete(&mut self, _: i64, _: bool) {}
}
pub struct VerboseReporter {
    pub steps: Vec<String>,
}

impl MatchReporter for VerboseReporter {
    fn on_step(&mut self, target: &[u8], t_idx: usize, query: &[u8], q_idx: usize, matched: bool) {
        let mut line = String::new();
        for (i, &byte) in target.iter().enumerate() {
            let c = byte as char;
            if i == t_idx {
                // Highlight current cursor in Yellow
                line.push_str(&format!("\x1b[1;33m[{}]\x1b[0m", c));
            } else {
                line.push(c);
            }
        }
        
        let status = if matched { "\x1b[32mMATCH\x1b[0m" } else { "skip " };
        let q_char = query[q_idx] as char;
        
        self.steps.push(format!("{} | Query: '{}' -> {}", line, q_char, status));
    }

    fn on_bonus(&mut self, name: &'static str, amount: i64, current: i64) {
        if let Some(last) = self.steps.last_mut() {
            last.push_str(&format!(" \x1b[1;34m+{}{} ({})\x1b[0m", amount, name, current));
        }
    }

    fn on_complete(&mut self, final_score: i64, fully_matched: bool) {
        let color = if fully_matched { "\x1b[1;32m" } else { "\x1b[1;31m" };
        self.steps.push(format!("{}--- Result: {} ---", color, final_score));
    }
}
