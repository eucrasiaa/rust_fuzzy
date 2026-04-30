use will_fuzzy::fuzzy::algorithms::AlgoWillGreedyVer2;
use will_fuzzy::fuzzy::algorithms::AlgoGreedyOptimized;
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
    let matcher_2 = FuzzyMatcher::with_algo(AlgoGreedyOptimized::new());
    let entities: Vec<AnimalEnt> = animal_demo();
    // let entities_2 = entities_1.iter().clone();

    // let session_1 = SearchSession::create(&entities_1, matcher_original, String::new());
    // let session_2 = SearchSession::create(&entities_1, matcher_new, String::new());
    //
    let mut session = SearchSession::create(&entities, matcher, String::new());
    let mut session_2 = SearchSession::create(&entities, matcher_2, String::new());

    let querries = vec![
        "dolphin".to_string(),
        "frog".to_string(),
        "bird".to_string(),
        "a b d c e f g".to_string(),
        "phant".to_string(),
        "monk".to_string(),
        "aligator".to_string(),     
        "chimpaznee".to_string(),
        "bear black".to_string(),
        "sea lion".to_string(),
        "eeeeeeeeaaaaaaaaaiiiiiiiiiiii".to_string(),
        "   cat   ".to_string(),
        "tree".to_string(),  
        "greater".to_string(),  
        "lesser".to_string(),
    ];
    // for _ in 1..=100{
    //     for query in querries.iter() {
    //         for c in query.to_ascii_lowercase().as_bytes() {
    //             session.type_char(*c);
    //         } 
    //         // println!("{}",session.current_query());
    //         // will_fuzzy::fuzzy::print_matches(session.top_results(1,0));
    //         session.clear();
    //     }
    // }
    for _ in 1..=100{
        for query in querries.iter() {
            for c in query.to_ascii_lowercase().as_bytes() {
                session_2.type_char(*c);

            println!("{}",session_2.current_query());
            } 
            will_fuzzy::fuzzy::print_matches(session_2.top_results(1,0));
            session_2.clear();
        }
    }
    //     session_2.set_query("red-winged blackbird");
    // println!("{}",session_2.current_query());
    // will_fuzzy::fuzzy::print_matches(session_2.top_results(5,0));
    //
    // session.set_query("red-winged blackbird");
    // println!("{}",session.current_query());
    // will_fuzzy::fuzzy::print_matches(session.top_results(5,0));
    //
    Ok(())
}
