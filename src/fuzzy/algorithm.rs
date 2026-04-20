// define a new algorithm by defining a struct, that does the "score" function.
// it can manage itself, store variables, etc.  
pub trait SimilarityAlgorithm {
    /// Source: original string we search Within
    /// target: the user typed string we match with 
    fn score(&self, source: &str, target: &str) -> i64;
}

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
            bonus_consec: 20,
            bonus_start: 20,
        }
    }
}

impl AlgoWillBasicGreedyVer1 {
    pub fn new() -> Self {
        Self::default()
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
 
        for (byte_idx, char) in target.char_indices() {
            if char.eq_ignore_ascii_case(&current_q) {
                // push match index to vec
                matched_indexes.push(byte_idx);
                //step next &  return when out of char
                if let Some(next_q) = query_chars.next() {
                    current_q = next_q;
                } else {
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
        let  mut scores_why = vec![0,0,0];
        // if is_empty() passed, then we know 0 will always exist, so safe
        if indices[0] == 0 { 
            score += self.bonus_start; 
            scores_why[0] = 1;
        }
        // if indices.contains(&0) { score +=self.bonus_start; };
        //TODO this feels like a prime loop unrolling thing bc of 2 distinct  i > 0 cases

        for i in 0..indices.len() {
            let current_idx = indices[i];

            // consec only matters past elem 1 
            if i > 0 && current_idx == indices[i - 1] + 1 {
                score += self.bonus_consec;
                            scores_why[1] += 1;
            }

            // score += (curr == prev + 1) as i32 * self.bonus_consec;
            if current_idx > 0 {
                let prev_byte = bytes[current_idx - 1];
                // bounds = ' ', -, or _
                // let is_bound = (prev_byte == b' ') | (prev_byte == b'-') | (prev_byte == b'_');
                // score += (is_bound as i32 as f64) * self.bonus_bound;
                if prev_byte == b' ' || prev_byte == b'-' || prev_byte == b'_' {
                    score += self.bonus_bound;
                                                scores_why[2] +=1;

                }
            }
        }
        if score != 0{
            let joined = scores_why.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
            println!("scored {} with {}", target, joined);
        }
        score
    }
}


pub fn algo_demo_slop(){
    let new_greedy = AlgoWillBasicGreedyVer1::default();
    let mut results: Vec<(i64, String)> = Vec::new();
    let to_comp_with = String::from("Dyscrasia");
    let combinations = ('a'..='z').flat_map(|a| {
        ('a'..='z').flat_map(move |b| {
            ('a'..='z').map(move |c| (a, b, c)) 
        })
    });
    for (a, b, c) in combinations {
            let tmp_str=format!("{a}{b}{c}");
            let tmp_score = new_greedy.score(&to_comp_with,&tmp_str);
            let new_item=(tmp_score, tmp_str);
            let pos = results.binary_search_by(|(score, _)| tmp_score.cmp(score))
                .unwrap_or_else(|e| e);
            results.insert(pos, new_item);

            print!("\x1B[H"); 

            'printer: for (i, item) in results.iter().enumerate() {
                println!("[{i}]: {} - {}\x1B[K", item.0, item.1);
                if i>20 { break 'printer};
            }
            results.truncate(1000);

            // results.push((tmp_score,tmp_str));
        
    }

}
