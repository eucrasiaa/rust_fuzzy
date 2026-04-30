use will_fuzzy::fuzzy::algorithms::AlgoWillGreedyVer2;
use will_fuzzy::fuzzy::canidate::{FuzzyCandidate, ScoreTarget};
use will_fuzzy::fuzzy::session::SearchSession;
use will_fuzzy::fuzzy::matcher::FuzzyMatcher;
use will_fuzzy::app::FuzzyApp;
use will_fuzzy::entities::*;
use will_fuzzy::fuzzy::canidate::FuzzyBoxExt;
use std::io::Result;



/// WITHIN THIS DEMO, THE BOX< DYN> IS USED FOR CODE CLARITY. 
/// its not super extended and validated, so be cautious using it like this.
fn main() -> Result<()>{

    #[cfg(feature = "logging")]
    println!("logging mode enabled");
    let matcher = FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new());
    // let matcher = FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new());
    let entities: Vec<AnimalEnt> = animal_demo();
    // let entities_2 = entities_1.iter().clone();

    // let session_1 = SearchSession::create(&entities_1, matcher_original, String::new());
    // let session_2 = SearchSession::create(&entities_1, matcher_new, String::new());
    //
    let mut session = SearchSession::create(&entities, matcher, String::new());
    
    let comp_str = "olphin".to_string();
    session.set_query("olphin");
    Ok(())
}
