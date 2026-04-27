pub use super::SimilarityAlgorithm;

// the SimAlgoGreedyV1
pub struct AlgoWillBasicGreedyVer1{
    pub bonus_bound:i64,
    pub bonus_consec:i64,
    pub bonus_start:i64,
}
impl SimilarityAlgorithm for AlgoWillBasicGreedyVer1 {
    fn score(&self, source: &str, target: &str) -> i64 {
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

impl AlgoWillBasicGreedyVer1 {
    pub fn new() -> Self {
        Self::default()
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
