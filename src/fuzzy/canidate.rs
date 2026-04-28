use std::fmt;
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

impl<'a> fmt::Display for ScoreTarget<'a>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text: {}, weight: {}, exact? {}", self.text, self.weight_multiplier,self.exact_match_only)

    }
}

/// technically usable on non-ascii, i haven't tested it extensively. so. just incase
#[derive(Debug, Clone, Copy, Default)]
pub enum SearchMode{
    // lets use the standard &[u8] algo
    #[default]
    Ascii,  
    //else
    Unicode, 
}
/// a generic basic struct, for a quick single / multi string implementation.
/// 
/// assumes a lowercase, non-exact match pattern. 
pub struct GenericStringStruct {
    pub visible_line: String,   // What the user sees
    pub precompute_str: String, // The actual searchable (likely lowercased) string
    pub freq: i64,              // Popularity score
    pub mode: SearchMode,       // ASCII vs Unicode toggle
}

impl GenericStringStruct {
    /// Helper to build a new candidate quickly
    pub fn new(text: &str, freq: i64) -> Self {
        Self {
            visible_line: text.to_string(),
            precompute_str: text.to_lowercase(),
            freq,
            mode: SearchMode::Ascii,
        }
    }
}

impl Default for GenericStringStruct {
    fn default() -> Self {
        Self {
            visible_line: String::new(),
            precompute_str: String::new(),
            freq: 0,
            mode: SearchMode::Ascii,
        }
    }
}

impl FuzzyCandidate for GenericStringStruct {
    fn search_targets(&self) -> Vec<ScoreTarget> {
        vec![
            ScoreTarget { 
                text: &self.precompute_str, 
                weight_multiplier: 1, 
                exact_match_only: false 
            },
        ]
    }

    fn usage_bonus(&self) -> i64 {
        // Boost score based on frequency
        self.freq
    }

    fn exec(&self) -> String {
        // Return the raw text or a command to run
        self.visible_line.clone()
    }

    fn display_text(&self) -> &str {
        &self.visible_line
    }
}



pub struct CandidateGenerator;

impl CandidateGenerator {
    /// Creates a batch of candidates from raw strings
    pub fn from_lines(lines: Vec<String>) -> Vec<GenericStringStruct> {
        lines.into_iter()
            .enumerate()
            .map(|(idx, line)| {
                let mut candidate = GenericStringStruct::new(&line, 0);
                
                // Heuristic: If it contains non-ASCII, set to Unicode mode
                if !line.is_ascii() {
                    candidate.mode = SearchMode::Unicode;
                }
                
                candidate
            })
            .collect()
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
pub trait FuzzyCandidate {
    /// for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    fn exec(&self) -> String;
    /// from use statistics, later include ig
    fn usage_bonus(&self) -> i64;
    fn display_text(&self) -> &str;
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
        write!(f, "[{:.2}] {}", self.score, self.item.display_text())
    }
}

