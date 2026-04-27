use super::canidate::*;
use super::SimilarityAlgorithm;



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
        is_ascii:&bool,
    ) -> Vec<ScoredResult<'a, T>> {
        if query.is_empty() {return vec![]} // eh?
        
        // preprocess query here not every call
        //                     let t_bytes = target.text.as_bytes();

        let q_bytes = query.as_bytes();
        let q_lower_vec = match is_ascii {
            true => query.to_ascii_lowercase().into_bytes(),
            false => query.to_lowercase().into_bytes(),
        };
        let q_lower = &q_lower_vec;
        // if its not ascii, we can use the buff 
        let mut target_lower_buffer = String::with_capacity(64);
        // let target_byte_buff = Vec prealloc?? maybe. 
        // let query_low = query.to_lowercase();
        let mut results: Vec<ScoredResult<'a, T>> = candidates
            .iter()
            .filter_map(|&item| {
                let mut best_score = 0;
                // let mut scoring_breakdown< = vec
                // score all provided targets and keep the highest one
                for target in item.search_targets() {
                    let score = if target.exact_match_only {
                        q_bytes.iter()
                            .zip(target.text.as_bytes().iter())
                            .take_while(|&(a,b)| a==b)
                            .count() as i64 * 15
                    } else {
                        // 2 cases. ascii, and non ascii
                        // ascii and not
                        match is_ascii {
                            true => self.algorithm.score(target.text.as_bytes(), q_lower),
                            false => {
                                // juuuust incase. for debugging
                                if target.text.is_ascii() {
                                    dbg!("wait we defined wrong.. somewhere.");
                                    self.algorithm.score(target.text.as_bytes(), q_lower)
                                }else {
                                    target_lower_buffer.clear();
                                for c in target.text.chars() {
                                    for lc in c.to_lowercase() {
                                        target_lower_buffer.push(lc);
                                    }
                                }
                                self.algorithm.score(target_lower_buffer.as_bytes(), q_lower)
                                }
                            }
                        }
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
                // swap thresh to go first for faster fail
                if threshold != -1 && final_score >= threshold {
                    Some(ScoredResult { item, score: final_score })
                } else {
                    None
                }
            })
        .collect();
        //todo this feels odd.. gotta be better way
        // maybe?
        // results.sort_unstable_by(|a, b| b.score.cmp(&a.score));
        results.sort_unstable_by_key(|b| std::cmp::Reverse(b.score));
        results
    }

}
// fn contains_ignore_case(target: &str, query_low: &str) -> bool {
//     if target.is_ascii() {
//         target.eq_ignore_ascii_case(query_low)
//     }
//     else{
//         target.to_lowercase().contains(query_low)
//     }
// }
// fn contains_ignore_case(target: &str, query_low: &[u8]) -> bool {
//     if target.is_ascii() && query_low.is_ascii() {
//         target.as_bytes()
//             .windows(query_low.len())
//             .any(|window| window.eq_ignore_ascii_case(query_low.as_bytes()))
//     } else {
//         target.to_lowercase().contains(query_low)
//     }
// }
