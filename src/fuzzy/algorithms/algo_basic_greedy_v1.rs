pub use super::SimilarityAlgorithm;
pub use super::{Trace, DebugAlgo};
// the SimAlgoGreedyV1
/// DO NOT USE: BAD AND WRONG
/// the first attempt at a scoring algorithm, poorly implemented but functional
/// and likely not doing EXACTLY what it claims to do
/// we traverse down the query, and see if the letter maps to somewhere within the target.
/// if so, check for 1 of three conditonals it could be
/// we define 3 core weights mappins to three conditions:
/// 1. boundary: on match, if the previous is a deliminator `-`, `_`, or `' '`
/// 2. consecutive: if the previous matched index also was found, so multiple in a row
/// 3. start: if the first letter of query matches the first of target
pub struct AlgoWillBasicGreedyVer1{
    pub bonus_bound:i64,
    pub bonus_consec:i64,
    pub bonus_start:i64,
}
impl SimilarityAlgorithm for AlgoWillBasicGreedyVer1 {
    /// Basic Greedy V1 implements scoring by finding all indexes,
    /// then scoring from those indexes. 
    fn score(&self, source: &str, target: &[u8]) -> i64 {
        let indexies = self.find_match_indices(target, source).unwrap_or_default();
        self.calculate_score(source, &indexies)
    }
}
impl Default for AlgoWillBasicGreedyVer1 {
    fn default() -> Self {
        Self {
            bonus_bound:  10,
            bonus_consec: 10,
            bonus_start: 20,
        }
    }
}

impl DebugAlgo for AlgoWillBasicGreedyVer1{


    fn multi_score(&self, target:&str, queries:Vec<String>) -> Vec<Trace>{
        let mut to_ret:Vec<Trace> = Vec::new();
        for qu in queries.iter(){
            to_ret.push(self.debug_score(target,qu));         
        }
        to_ret.sort_unstable_by_key(|s| s.final_score);
        to_ret
    }
    fn debug_score(&self, target: &str, query: &str) -> Trace{
        let mut steps = Vec::new();
        let mut matched_indices = Vec::new();
        let mut current_score = 0;

        let mut query_chars = query.chars().peekable();
        let bytes = target.as_bytes();
        for (byte_idx, t_char) in target.char_indices() {
            let current_q = match query_chars.peek() {
                Some(&c) => c,
                None => break,
            };

            let is_match = t_char.eq_ignore_ascii_case(&current_q) 
                        || (self.is_separator(t_char) && self.is_separator(current_q));

            let mut line = String::new();
            for (i, c) in target.chars().enumerate() {
                let pos = target.char_indices().nth(i).unwrap().0;
                if pos == byte_idx {
                    line.push_str(&format!("\x1b[1;33m[{}]\x1b[0m", c));
                } else if matched_indices.contains(&pos) {
                    line.push_str(&format!("\x1b[32m{}\x1b[0m", c)); 
                } else {
                    line.push(c);
                }
            }

            if is_match {
                matched_indices.push(byte_idx);
                query_chars.next(); 
                
                let mut bonus_msg = String::new();
                if byte_idx == 0 {
                    current_score += self.bonus_start;
                    bonus_msg = format!(" (+{} Start)", self.bonus_start);
                } else {
                    let prev_byte = bytes[byte_idx - 1];
                    if prev_byte == b' ' || prev_byte == b'-' || prev_byte == b'_' {
                        current_score += self.bonus_bound;
                        bonus_msg = format!(" (+{} Boundary)", self.bonus_bound);
                    }
                }
                
                steps.push(format!("{} Match '{}'! Score: {}{}", line, t_char, current_score, bonus_msg));
            } else {
                //log skips?
            }
        }

        Trace {
            target: target.to_string(),
            query: query.to_string(),
            steps,
            final_score: current_score,
        }
    
    

    }
}


impl AlgoWillBasicGreedyVer1 {
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
    fn is_separator(&self, c: char) -> bool {
        matches!(c, ' ' | '-' | '_')
    }

    //byte index of matches (the fast fail here!)
    fn find_match_indices(&self, query: &str, target: &str) -> Option<Vec<usize>> {
        let mut matched_indexes:Vec<usize> = Vec::with_capacity(query.len()); 
        // pre known size, might help?
        // to fast fail, if first char doesnt match, return instantly
        let mut query_chars = query.chars();
        // iter type 
        let mut current_q = query_chars.next()?;
        // let mut current_q = match query_chars.next() 
        //     Some(c) => c,
        //     None => return None,
 
        for (byte_idx, t_char) in target.char_indices() {
            let is_match = t_char.eq_ignore_ascii_case(&current_q) 
                   || (self.is_separator(t_char) && self.is_separator(current_q));
            if is_match {
                matched_indexes.push(byte_idx);
        
            if let Some(next_q) = query_chars.next() {
                current_q = next_q;
            } else {
                // Found all query chars in sequence!
                return Some(matched_indexes);
            }
        }
        }
        None
    }
    
    //string + indexes, score it
    fn calculate_score(&self, target:&str, indices: &[usize]) -> i64 {
        // 0 matches at all? drop it
        if indices.is_empty() { return 0; }
        let bytes = target.as_bytes();
        let mut score = 0;
        // if is_empty() passed, then we know 0 will always exist, so safe
        if indices[0] == 0 { 
            score += self.bonus_start; 
        }
        // if indices.contains(&0) { score +=self.bonus_start; };
        //TODO this feels like a prime loop unrolling thing bc of 2 distinct  i > 0 cases
        // no thats stupid... trust the compiler  Will cmonn
        for i in 0..indices.len() {
            let current_idx = indices[i];

            // consec only matters past elem 1 
            // if i > 0 && current_idx == indices[i - 1] + 1 {
            //     score += self.bonus_consec;
            // }
            if current_idx > 0 {
                let prev_byte = bytes[current_idx - 1];
                // bounds = ' ', -, or _
                // let is_bound = (prev_byte == b' ') | (prev_byte == b'-') | (prev_byte == b'_');
                // score += (is_bound as i32 as f64) * self.bonus_bound;
                if prev_byte == b' ' || prev_byte == b'-' || prev_byte == b'_' {
                    score += self.bonus_bound;

                }
            }
        }
        //DEBUG_PRINT
        // if score != 0{
        //     let joined = scores_why.iter()
        //             .map(|n| n.to_string())
        //             .collect::<Vec<String>>()
        //             .join(", ");
        //     println!("scored {} with {}", target, joined);
        // }
        score
    }
}
