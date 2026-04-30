// pub mod algo_basic_greedy_v1;
pub mod algo_greedy_v2;
pub mod algo_greedy_opti;
// pub use algo_basic_greedy_v1::AlgoWillBasicGreedyVer1;
pub use algo_greedy_v2::AlgoWillGreedyVer2;
pub use algo_greedy_opti::AlgoGreedyOptimized;

#[cfg(feature = "logging")]
pub use crate::fuzzy::TraceReporter;

/// all an algo has to implement is a scoring function
/// it will be passed the pre-modified strings from session
/// (eg: session handles exact vs just lowercase match)
pub trait SimilarityAlgorithm {
    /// Target: original static strings we search against, the canidates 
    /// Query: the user typed string we match with 
    // fn score(&self, target: &str, query: &str) -> i64;
    // fn score(&self, target: &[u8], query: &[u8]) -> i64;
    // fn score<T: AsRef<[u8]>, Q: AsRef<[u8]>>(&self, target: T, query: Q) -> i64;
    // #[cfg(feature = "logging")]
    // fn score<T: AsRef<[u8]>, Q: AsRef<[u8]>>(
    //     &self, 
    //     target: T, 
    //     query: Q, 
    //     reporter: &mut TraceReporter
    // ) -> i64; 
    // #[cfg(not(feature = "logging"))]
    fn score<T: AsRef<[u8]>, Q: AsRef<[u8]>>(
        &self, 
        target: T, 
        query: Q
    ) -> i64;

}

