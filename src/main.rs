use std::process::Command;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp::Ordering;

// |||||||||||||||||||||||||||||||
// fuzzy finder part
// |||||||||||||||||||||||||||||||

//strings to score, their rules
pub struct ScoreTarget<'a> {
    pub text: &'a str,
    pub weight_multiplier: f64, // the weight
    pub exact_match_only: bool, // for smthn like tags? idk
}
//using a trait to define parts of a struct to score + weights
pub trait FuzzyCandidate {
    // for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    // from use statistics, later include ig
    fn usage_bonus(&self) -> f64;
}

// struct returned off a fzzy
pub struct ScoredResult<'a, T> {
    pub item: &'a T,
    pub score: f64,
}
//
// #[derive(Default)]
// pub enum SimAlgo {
//     #[default]
//     FzfStyle,
//     Other
// }
//
// fn calc_other(source: &str, target: &str) -> f64{ 
//
//  0.0
// }
//
// impl SimAlgo {
//     pub fn score(&self, source: &str, target: &str) -> f64 {
//         match self {
//             SimAlgo::FzfStyle => {
//                 1.0
//             }
//             SimAlgo::Other =>{
//                 calc_other(source,target)
//             }
//         }
// }


pub trait SimilarityAlgorithm {
    fn score(&self, source: &str, target: &str) -> f64;
}

pub struct SimAlgoFzfStyle;
impl SimilarityAlgorithm for SimAlgoFzfStyle {
    fn score(&self, source: &str, target: &str) -> f64 {
        0.0 
    }
}
pub struct SimAlgoOtherAlgo;
impl SimilarityAlgorithm for SimAlgoOtherAlgo {
    fn score(&self, source: &str, target: &str) -> f64 {
        other_calc(source, target) 
    }
}


fn other_calc(a: &str, b: &str) -> f64 {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    if len_a == 0 && len_b == 0 { return 100.0; }
    
    let max_len = std::cmp::max(len_a, len_b) as f64;
    let distance = other_recurse(a, b) as f64;
    (1.0 - (distance / max_len)) * 100.0
}

fn other_recurse(a: &str, b: &str) -> usize {
    if a.is_empty() { return b.chars().count(); }
    if b.is_empty() { return a.chars().count(); }
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();
    let a_head = a_chars.next().unwrap();
    let b_head = b_chars.next().unwrap();
    let a_tail = a_chars.as_str();
    let b_tail = b_chars.as_str();

    if a_head == b_head {
        other_recurse(a_tail, b_tail)
    } else {
        1 + std::cmp::min(
            other_recurse(a_tail, b),    
            std::cmp::min(
                other_recurse(a, b_tail), 
                other_recurse(a_tail, b_tail) 
            )
        )
    }
}

//conf for FuzzyMatcher
#[derive(Default)]
pub struct FuzzyMatcher<A: SimilarityAlgorithm> {
    pub algorithm: A,
    pub threshold: f64,        
    pub case_sensitive: bool,
}


impl FuzzyMatcher<SimAlgoFzfStyle> {
    pub fn new() -> Self {
        Self {
            algorithm: SimAlgoFzfStyle,
            threshold: 0.0,
            case_sensitive: false,
        }
    }
}


impl<A: SimilarityAlgorithm> FuzzyMatcher<A> {
    pub fn with_algo(algorithm: A) -> Self {
        Self {
            algorithm,
            threshold: 0.0,
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
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }

    // main thing?
    pub fn search<'a, T: FuzzyCandidate>(
        &self,
        query: &str,
        candidates: &'a [T],
    ) -> Vec<ScoredResult<'a, T>> {
        let mut results: Vec<ScoredResult<'a, T>> = candidates
            .iter()
            .filter_map(|item| {
                let mut best_score = 0.0;
                // score all provided targets and keep the highest one
                for target in item.search_targets() {
                    let score = if target.exact_match_only {
                        // strict substring check for tags
                        if target.text.to_lowercase().contains(&query.to_lowercase()) { 100.0 } else { 0.0 }
                    } else {
                        // TODO!!!
                        self.algorithm.score(target.text, query)
                        // if target.text.to_lowercase().contains(&query.to_lowercase()) { 50.0 } else { 0.0 }
                        //TOTO!
                    };
                    let weighted_score = score * target.weight_multiplier;
                    if weighted_score > best_score {
                        best_score = weighted_score;
                    }
                }
                let final_score = best_score + item.usage_bonus();
                // thresh filter
                if final_score >= self.threshold {
                    Some(ScoredResult { item, score: final_score })
                } else {
                    None
                }
            })
        .collect();
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
        results
    }

}



// |||||||||||||||||||||||||||||||||
// desktop stuff for testing its work!
// |||||||||||||||||||||||||||||||||
pub struct DesktopEntity{
    // id = filename.desktop
    id: String,
    // name = "Executable Desktop Nice Name" - "Strawberry"
    name: String,
    // generic_name = ""  - "Strawberry Music Player"
    generic_name: String,
    // desc = "description of it"
    desc: String,
    // exec = "exec commands + args" - "strawberry %U"
    exec: String,
    // tags<Vec Str> = []  - "\[AudioVideo\]\[Player\]\[Qt\]\[Audio\];" 
    tags: Vec<String>,

    launch_count: i64
}
impl FuzzyCandidate for DesktopEntity {
    fn search_targets(&self) -> Vec<ScoreTarget> {
        let mut targets = vec![
            ScoreTarget { text: &self.name, weight_multiplier: 1.0, exact_match_only: false },
        ];
        //generic w/ penalty
        if !self.generic_name.is_empty() {
            targets.push(ScoreTarget { 
                text: &self.generic_name, 
                weight_multiplier: 0.5, 
                exact_match_only: false 
            });
        }
        //tags, strict for slightly diff parsing? or do i jsut handle sep?
        for tag in &self.tags {
            targets.push(ScoreTarget { 
                text: tag, 
                weight_multiplier: 0.8, 
                exact_match_only: true
            });
        }
        targets
    }
    fn usage_bonus(&self) -> f64 {
        //tweak this heavy LOL
        self.launch_count as f64 * 1.2 
    }
}


// because backtracking with backspace, we store each step in a stack style thing, and backspace =
// pop!
pub struct SearchSession<'a> {
    all_apps: &'a [DesktopEntity],

    // my lovely stack
    history: Vec<Vec<ScoredResult<'a, DesktopEntity>>>,

    current_query: String,
}

pub struct AnimalEnt{
    name:String,
    freq:f64,
} 


impl FuzzyCandidate for AnimalEnt{
    fn search_targets(&self) -> Vec<ScoreTarget>{
        let targets = vec![
            ScoreTarget { text: &self.name, weight_multiplier: 1.0, exact_match_only: false},
        ];
        targets
    }
    fn usage_bonus(&self) -> f64{
        self.freq * 1.2
    }
}

pub struct DemoSession<'a> {
    list_strings: &'a [AnimalEnt],

    // my lovely stack
    history: Vec<Vec<ScoredResult<'a, AnimalEnt>>>,

    current_query: String,
}


fn main() {


    let file = File::open("animallist.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let animals: Vec<AnimalEnt> = reader
        .lines()
        .map_while(Result::ok) // just incase? yells otherwisie 
        .map(|name| AnimalEnt {
            name: name.trim().to_string(),
            freq: 1.0, // basically ignore 
        })
        .collect();

    let mut test_sess = DemoSession {
        list_strings: &animals, 
        history: Vec::new(),
        current_query: String::new(),
    };

    let matcher = FuzzyMatcher::<SimAlgoOtherAlgo>::new();
    

    
    
    
}


