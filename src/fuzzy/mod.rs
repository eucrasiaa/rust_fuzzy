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

#[inline(always)]
pub fn scale_weight(f: f64) -> i64 {
    (f * 1024.0).round() as i64
}
// pub use self::algorithm::*;
    // Add other common traits/structs here
// }
// pub use self::SimilarityAlgorithm;
