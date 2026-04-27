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
    fn on_step(&mut self, target: &str, current_idx: usize, query_char: char, matched: bool, score_diff: i64);
}
// no op for telemetry? idk i saw on a forum
impl MatchReporter for () {
    #[inline(always)]
    fn on_step(&mut self, _: &str, _: usize, _: char, _: bool, _: i64) {}
}
pub struct DebugReporter {
    pub steps: Vec<String>,
}
