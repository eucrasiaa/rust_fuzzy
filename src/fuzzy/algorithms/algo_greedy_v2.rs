pub use super::SimilarityAlgorithm;
#[cfg(feature = "logging")]
use crate::fuzzy::StepSnapshot;
#[cfg(feature = "logging")]
pub use crate::fuzzy::TraceReporter;

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

impl SimilarityAlgorithm for AlgoWillGreedyVer2 {
    // #[cfg(not(feature = "logging"))] 
    fn score<T: AsRef<[u8]>, Q: AsRef<[u8]>>(&self, target: T, query: Q) -> i64{
        // let mut reporter = DebugReporter { steps: Vec::new() };
        // let score = self.one_step_calc(target, query, &mut reporter);
        //
        // for step in reporter.steps {
        //     println!("{}", step);
        // }
        let t = target.as_ref();
        let q = query.as_ref();
        self.one_step_calc(t,q)
            // let indexies = self.find_match_indices(target, source).unwrap_or_default();
            // self.calculate_score(source, &indexies)
    }

    // let indexies = self.find_match_indices(target, source).unwrap_or_default();
    // self.calculate_score(source, &indexies)

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

    // #[cfg(feature = "logging")]
    // pub fn one_step_calc_rep(&self, target: &[u8], query: &[u8], mut reporter: &TraceReporter) -> (i64, mTraceReporter){
    //
    //     (0,reporter)
    // }
    /// core philosopy is fast fail, fast iter, minimal allocations
    //
    // fn one_step_calc<R:MatchReporter>(&self, target: &[u8], query: &[u8], reporter: &mut R) -> i64{
    // #[cfg(not(feature = "logging"))]
    // #[cfg(feature = "logging")]
    pub fn one_step_calc(&self, target: &[u8], query: &[u8],
        // #[cfg(feature = "logging")] reporter: &mut TraceReporter
    ) -> i64{

        // eprintln!("{}",String::from_utf8(target.to_vec()).unwrap());
        // eprintln!("target: {:?}  -  query: {:?}",target,query);
        // pub fn one_step_calc<R: MatchReporter>(&self, target: &[u8], query: &[u8], reporter: &mut R) -> i64 {
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
        let mut amt_consec = 1;
        let mut score:i64 = 0;
        let mut target_index=0;
        let mut query_index=0;
        #[cfg(feature = "logging")]
        let mut reporter = TraceReporter::new(target, query); 
        while target_index < target.len() && query_index < query.len(){
            let t_byte = target[target_index];
            let q_byte = query[query_index];
            //match, orr if either is a seperator(match)
            #[cfg(feature = "logging")]
            let is_match = t_byte == q_byte || (is_sep(t_byte) && is_sep(q_byte)); 
            if t_byte == q_byte || (is_sep(t_byte) && is_sep(q_byte)) {
                // b/c we advance both together, or target alone, any target_index ==0 means its on
                // first. so, apply bonus start
                if target_index == 0 {
                    score+=self.bonus_start;
                }// if its not, its at least past 1. so we can safely check for -1 without extra
                 // conditonal:
                else{
                    // check if past is a delim, if so, give delim bonus
                    // such a finicky if, might be faster this way?
                    // DELIM BONUS!
                    // unsafe { std::arch::asm!(";# LLVM-MCA-BEGIN"); }
                    let mask = SEPARATOR_MAP[target[target_index - 1] as usize];
                    score += mask & self.bonus_bound;
                    // score += -(is_sep(target[target_index - 1]) as i64) & self.bonus_bound;
                    // unsafe { std::arch::asm!(";# LLVM-MCA-END"); }

                    // CONSEC BONUS
                    if target_index == prev_match_index + 1 {
                        amt_consec+=5;
                        score += self.bonus_consec+amt_consec;
                    }
                    else{ amt_consec = 0}
                }
                prev_match_index = target_index;

                // advance query if match expr
                #[cfg(feature = "logging")]
                reporter.record(StepSnapshot {
                    target_index, query_index,
                    t_byte, q_byte,
                    prev_match_index, amt_consec, score,
                    is_match
                });

                query_index +=1;  


            }

            #[cfg(feature = "logging")]
            if !is_match{
                reporter.record(StepSnapshot {
                    target_index, query_index,
                    t_byte, q_byte,
                    prev_match_index, amt_consec, score,
                    is_match
                });
            }
            target_index +=1;
        }
        let mask = ((query_index == query.len()) as i64).wrapping_neg();

        let ret_score = mask & score;
        #[cfg(feature = "logging")]
        reporter.print_trace(ret_score);
        ret_score
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

