use super::algorithm::{SimilarityAlgorithm,AlgoWillBasicGreedyVer1};
use super::canidate::*;

use std::cmp::Ordering;



#[derive(Default)]
pub struct FuzzyMatcher<A: SimilarityAlgorithm> {
    pub algorithm: A,
    pub case_sensitive: bool,
}


// impl FuzzyMatcher<AlgoWillBasicGreedyVer1> {
//     pub fn new() -> Self {
//         Self {
//             algorithm: AlgoWillBasicGreedyVer1,
//             threshold: 0,
//             case_sensitive: false,
//         }
//     }
// }

impl<A: SimilarityAlgorithm> FuzzyMatcher<A> {
    pub fn with_algo(algorithm: A) -> Self {
        Self {
            algorithm,
            case_sensitive: false,
        }
    }
    // pub fn new() -> Self {
    //     Self {
    //         algorithm: SimAlgoFzfStyle,
    //         threshold: 0.0,
    //         case_sensitive: false,
    //     }
    // }

    // pub fn set_thresh(mut self, threshold: f64) -> Self {
    //     self.threshold = threshold;
    //     self
    // }
    // main thing?

    //assumes sorted
    // pub fn update_thresh<'a, T>(&mut self, results: &[ScoredResult<'a, T>]) -> f64 {
    pub fn update_thresh<'a, T>(&mut self, results: &[ScoredResult<'a, T>]) ->i64  {
        if results.is_empty() { return -1; }
        if results.len() < 2 { return 10; }
        let top_score = results[0].score;
        let mut max_diff = -1;
        let mut thrsh = -1;
        // if results.is_empty() { return -1 }
        for window in results.windows(2){  
            let prev = &window[0];
            let curr = &window[1];
            let diff = prev.score - curr.score;
            if diff > max_diff {
                max_diff = diff;
                thrsh = curr.score;
                // best_idx = i + 1;
            }
        }
        // println!("{}",thrsh.max(10));
        // give the results SOME leniency to prevent overagressive?
        let mercy_floor = (top_score as f64 * 0.5) as i64;
    
        thrsh.min(mercy_floor).max(10)
        // thrsh.max(10)

        // if let Some(res) = results.get(best_idx) {
        // self.threshold = res.score;
        // res.score
        // } 
        // thrsh
    }

    pub fn search<'a, T: FuzzyCandidate>(
        &self,
        query: &str,
        candidates: &[&'a T],
        threshold: i64,
    ) -> Vec<ScoredResult<'a, T>> {
        let query_low = query.to_lowercase();
        let mut results: Vec<ScoredResult<'a, T>> = candidates
            .iter()
            .filter_map(|&item| {
                let mut best_score = 0;
                // let mut scoring_breakdown< = vec
                // score all provided targets and keep the highest one
                for target in item.search_targets() {
                    //DEBUG_PRINT
                    //println!("target: {}", target);
                    let score = if target.exact_match_only {
                        // strict substring check for tags
                        // TODO WORK WITH NON ASCII? maybe helper
                        // if target.text.eq_ignore_ascii_case(&query_low) {
                        if contains_ignore_case(&target.text,&query_low){
                            (target.text.len() as i64) * 10
                        } else {
                            0
                        }


                    } else {
                        // TODO!!!
                        self.algorithm.score(target.text, query)
                            // let a = self.algorithm.score(target.text, query);
                            // println!("{a}: {}",target.text);
                            // a
                            // if target.text.to_lowercase().contains(&query.to_lowercase()) { 50.0 } else { 0.0 }
                            //TOTO!
                    };
                    let weighted_score = (score * target.weight_multiplier) >> 10;
                    // let weighted_score = ((score as f64) * target.weight_multiplier) as i64;
                    if weighted_score > best_score {
                        best_score = weighted_score;
                    }
                }
                // we iterate all the search targets, 
                let final_score = best_score + item.usage_bonus();
                // thresh filter
                // -1 = none used
                
                if final_score >= threshold && threshold != -1 {
                    Some(ScoredResult { item, score: final_score })
                } else {
                    None
                }
            })
        .collect();
        //todo this feels odd.. gotta be better way
        // maybe?
        results.sort_unstable_by(|a, b| b.score.cmp(&a.score));
        results
    }

}
fn contains_ignore_case(target: &str, query_low: &str) -> bool {
    if target.is_ascii() {
        target.eq_ignore_ascii_case(query_low)
    }
    else{
        target.to_lowercase().contains(query_low)
    }
}
