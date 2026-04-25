use std::fmt;
// |||||||||||||||||||||||||||||||
// fuzzy finder part
// |||||||||||||||||||||||||||||||

///strings to score, their rules
pub struct ScoreTarget<'a> {
    pub text: &'a str,
    /// the weight
    pub weight_multiplier: f64,
    /// for smthn like tags? idk
    pub exact_match_only: bool, 
}

impl<'a> fmt::Display for ScoreTarget<'a>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text: {}, weight: {}, exact? {}", self.text, self.weight_multiplier,self.exact_match_only)

    }
}

///using a trait to define parts of a struct to score + weights
pub trait FuzzyCandidate {
    /// for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    /// from use statistics, later include ig
    fn usage_bonus(&self) -> i64;
    fn display_candidate(&self) -> String {
        self.search_targets()
            .iter()
            .map(|t| t.to_string()) 
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

pub struct ScoredResult<'a, T> {
    pub item: &'a T,
    pub score: i64,
}

impl<'a, T> fmt::Display for ScoredResult<'a, T> 
where 
    T: FuzzyCandidate
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "[{:.2}] {}", self.score, self.item.display_candidate())
    }
}

