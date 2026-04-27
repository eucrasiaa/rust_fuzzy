use super::{MatchReporter, VerboseReporter};

pub use super::SimilarityAlgorithm;


/// Remade Algorithm, might actually do what it says
/// we traverse down the query, and see if the letter maps to somewhere within the target.
/// if so, check for 1 of three conditonals it could be
/// we define 3 core weights mappins to three conditions:
/// 1. boundary: on match, if the previous is a deliminator `-`, `_`, or `' '`
/// 2. consecutive: if the previous matched index also was found, so multiple in a row
/// 3. start: if the first letter of query matches the first of target
pub struct AlgoWillGreedyVer2{
    pub bonus_bound:i64,
    pub bonus_consec:i64,
    pub bonus_start:i64,
}
// impl SimilarityAlgorithm for AlgoWillGreedyVer2 {
//     fn score(&self, target: &[u8], query: &[u8]) -> i64{
//         // let mut reporter = DebugReporter { steps: Vec::new() };
//         // let score = self.one_step_calc(target, query, &mut reporter);
//         //
//         // for step in reporter.steps {
//         //     println!("{}", step);
//         // }
//         // score
//         self.one_step_calc(target,query)
//         // let indexies = self.find_match_indices(target, source).unwrap_or_default();
//         // self.calculate_score(source, &indexies)
//     }
// }
impl SimilarityAlgorithm for AlgoWillGreedyVer2 {
    fn score(&self, target: &[u8], query: &[u8]) -> i64 {
        #[cfg(debug_assertions)] // Only run debug logic in dev builds
        if std::env::var("DEBUG_ALGO").is_ok() {
            let mut reporter = VerboseReporter { steps: Vec::new() };
            let score = self.one_step_calc(target, query, &mut reporter);
            for step in reporter.steps { println!("{}", step); }
            return score;
        }

        // Production: Pass the empty unit reporter
        self.one_step_calc(target, query, &mut ())
    }
}
impl Default for AlgoWillGreedyVer2 {
    fn default() -> Self {
        Self {
            bonus_bound:  10,
            bonus_consec: 10,
            bonus_start: 20,
        }
    }
}

impl AlgoWillGreedyVer2 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_weights(custom_bonus_bound:i64, custom_bonus_consecutive:i64, custom_bonus_start:i64) -> Self { 
        Self{
            bonus_bound:custom_bonus_bound,
            bonus_consec:custom_bonus_consecutive,
            bonus_start:custom_bonus_start
        }
    }
    pub fn modify_weights(&mut self, custom_bonus_bound:i64, custom_bonus_consecutive:i64, custom_bonus_start:i64) {
        self.bonus_bound = custom_bonus_bound;
        self.bonus_consec = custom_bonus_consecutive;
        self.bonus_start = custom_bonus_start;
    }

    /// core philosopy is fast fail, fast iter, minimal allocations
    ///
    // fn one_step_calc<R:MatchReporter>(&self, target: &[u8], query: &[u8], reporter: &mut R) -> i64{
    // pub fn one_step_calc(&self, target: &[u8], query: &[u8]) -> i64{
    pub fn one_step_calc<R: MatchReporter>(&self, target: &[u8], query: &[u8], reporter: &mut R) -> i64 {
        // let s = String::from_utf8((&target).to_vec()).expect("Found invalid UTF-8");
        // refactor that actually works
        // process psudo:
        // if either empty, fail 
        // ideally, fail if we dont match any bounds/start
        // maybe? fail off first char not match. but more the any bounds
        // prev iter matched usize var
        // var score
        // while over both,  
        // check if identical
        //    if match{
        //    if start of string (index 1?) 
        //              start bonus!
        //    else if, check if prev matched. 
        //        add consec bonus
        //    else if prev is delimin
        //         add delim bonus
        //    update last matched index
        //    advance query
        //   }
        //  regardless of match, advance target:
        //}
        // if we reach end of query, return score. (want to ensure target didnt end early
        // i think? ran odd without)
        if target.is_empty() || query.is_empty() { return 0 }
        let mut prev_match_index=0;
        let mut amt_consec = 0;
        let mut score:i64 = 0;
        let mut target_index=0;
        let mut query_index=0;
        while target_index < target.len() && query_index < query.len(){
            let t_byte = target[target_index];
            let q_byte = query[query_index];

            let is_match = t_byte == q_byte || (is_sep(t_byte) && is_sep(q_byte));
            reporter.on_step(target, target_index, query, query_index, is_match);
            //match, orr if either is a seperator(match)
            if t_byte == q_byte || (is_sep(t_byte) && is_sep(q_byte)) {
                // b/c we advance both together, or target alone, any target_index ==0 means its on
                // first. so, apply bonus start
                if target_index == 0 {
                    score+=self.bonus_start;
                    reporter.on_bonus("START", self.bonus_start, score);
                }// if its not, its at least past 1. so we can safely check for -1 without extra
                 // conditonal:
                else{
                    // check if past is a delim, if so, give delim bonus
                    // such a finicky if, might be faster this way?
                    // DELIM BONUS!
                    // unsafe { std::arch::asm!(";# LLVM-MCA-BEGIN"); }
                    let mask = SEPARATOR_MAP[target[target_index - 1] as usize];
                    // score += mask & self.bonus_bound;
                    let b_amt = mask & self.bonus_bound;
                    if b_amt > 0 {
                        
                        score += b_amt;
                        reporter.on_bonus("BOUNDARY", b_amt, score);
                    }
                    else{
                    }
                    // score += -(is_sep(target[target_index - 1]) as i64) & self.bonus_bound;
                    // unsafe { std::arch::asm!(";# LLVM-MCA-END"); }
                     
                    // CONSEC BONUS
                    // if target_index == prev_match_index + 1 {
                    //     score += self.bonus_consec;
                    // }
                    if target_index == prev_match_index + 1 {
                    score += self.bonus_consec + amt_consec;
                    amt_consec <<= 1;
                    reporter.on_bonus("CONSECUTIVE", self.bonus_consec+amt_consec, score);
                    }
                    else{
                        amt_consec = 1;

                    }
                }
                prev_match_index = target_index;

                // advance query if match expr
                query_index +=1;   
            }
            target_index +=1;
        }
        // let mask = ((query_index == query.len()) as i64).wrapping_neg();
        // mask & score
        let fully_matched = query_index == query.len();
    let mask = (fully_matched as i64).wrapping_neg();
    let final_score = mask & score;
    
    reporter.on_complete(final_score, fully_matched);
    final_score
    }
    
}
static SEPARATOR_MAP: [i64; 256] = {
    let mut map = [0i64; 256];
    map[b' ' as usize] = -1; // -1 is all 1s in bits
    map[b'-' as usize] = -1;
    map[b'_' as usize] = -1;
    map[b'/' as usize] = -1;
    map[b'\\' as usize] = -1;
    map[b'.' as usize] = -1;
    map
};
#[inline(always)]
fn is_sep(b: u8) -> bool {
    b == b' ' || b == b'-' || b == b'_' || b == b'/' || b == b'\\' || b == b'.'
}

