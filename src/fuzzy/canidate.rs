use std::fmt;
use super::scale_weight;
// |||||||||||||||||||||||||||||||
// fuzzy finder part
// |||||||||||||||||||||||||||||||

///strings to score, their rules
pub struct ScoreTarget<'a> {
    pub text: &'a str,
    /// the weight. scale of 1024 and bit shifted in math! 1024 = 1.0, 512 = 0.5, etc.
    pub weight_multiplier: i64, // NEW!!
    /// for smthn like tags? idk
    pub exact_match_only: bool, 
}

impl<'a> ScoreTarget<'a> {
    /// Creates a new target with a float weight (e.g., 1.0) and handles the bit-shifting.
    pub fn new(text: &'a str, weight: f64, exact_match_only:bool) -> Self {
        Self {
            text,
            weight_multiplier: scale_weight(weight), 
            exact_match_only,
        }
    }
}
impl<'a> fmt::Display for ScoreTarget<'a>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text: {}, weight: {}, exact? {}", self.text, self.weight_multiplier,self.exact_match_only)

    }
}


// impl Default for GenericStringStruct{
//     fn default() -> Self {
//
//     }
// }
// impl FuzzyCandidate for GenericStringStruct{
//
// }
//
//
//
// impl FuzzyCandidate for AnimalEnt{
//     fn search_targets(&self) -> Vec<ScoreTarget>{
//         let targets = vec![
//             ScoreTarget { text: &self.name, weight_multiplier: scale_weight(1.0), exact_match_only: false},
//         ];
//         targets
//     }
//     fn usage_bonus(&self) -> i64{
//         self.freq + 5
//     }
//     fn exec(&self) -> String{
//         "!".to_string()
//     }
//     fn display_text(&self) -> &str{
//         &self.precompute_str
//     }
// }



///using a trait to define parts of a struct to score + weights
/// hardcode weight 1?
pub trait FuzzyCandidate {
    /// for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    fn exec(&self) -> String { "\0".to_string() }
    /// from use statistics, later include ig
    fn usage_bonus(&self) -> i64 {1}
    fn display_text(&self) -> &str;
    fn display_candidate(&self) -> String {
        self.search_targets()
            .iter()
            .map(|t| t.to_string()) 
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

impl FuzzyCandidate for Box<dyn FuzzyCandidate> {
    fn search_targets(&self) -> Vec<ScoreTarget> {
        (**self).search_targets()
    }
    fn usage_bonus(&self) -> i64 { (**self).usage_bonus() }
    fn exec(&self) -> String { (**self).exec() }
    fn display_text(&self) -> &str { (**self).display_text() }
}

pub trait FuzzyBoxExt {
    fn into_boxed(self) -> Vec<Box<dyn FuzzyCandidate>>;
}
impl<I, T> FuzzyBoxExt for I 
where 
    I: Iterator<Item = T>,
    T: FuzzyCandidate + 'static, 
{
    fn into_boxed(self) -> Vec<Box<dyn FuzzyCandidate>> {
        self.map(|x| Box::new(x) as Box<dyn FuzzyCandidate>)
            .collect()
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
        write!(f, "[{:.2}] {}", self.score, self.item.display_text())
    }
}

