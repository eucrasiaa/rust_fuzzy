pub mod algo_basic_greedy_v1;
pub mod algo_greedy_v2;
pub use algo_basic_greedy_v1::AlgoWillBasicGreedyVer1;
pub use algo_greedy_v2::AlgoWillGreedyVer2;

pub trait SimilarityAlgorithm {
    /// Source: original string we search Within
    /// target: the user typed string we match with 
    fn score(&self, source: &str, target: &str) -> i64;
}
