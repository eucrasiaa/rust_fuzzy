pub mod algorithm;
pub mod session;
pub mod canidate;
pub mod matcher;
// pub mod prelude {
// pub use self::algorithm::*;
pub use self::algorithm::*;
pub use self::canidate::*;
pub use self::session::*;
pub use self::matcher::*;

pub use self::algorithm::SimilarityAlgorithm;
pub use self::algorithm::AlgoWillBasicGreedyVer1;



// pub use self::algorithm::*;
    // Add other common traits/structs here
// }
// pub use self::SimilarityAlgorithm;
