
use super::prelude::*;
use crate::SearchMode;
/// a generic basic struct, for a quick single / multi string implementation.
/// 
/// assumes a lowercase, non-exact match pattern. 
pub struct GenericStringStruct {
    pub visible_line: String,   
    pub precompute_str: String, 
    pub freq: i64,       
    pub mode: SearchMode, 
}


fn generic_from_file(filename:&str) -> File{
    match File::open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("Error: File not found! Ensure the file path is valid! {:?}.",filename);
                std::process::exit(1); 
            }
            _ => {
                eprintln!("Error: Failed to open file: {:?}", error);
                std::process::exit(1);
            }
        },
    }

}


/// default way to handle passing a filename, treating each line as a string
/// returns the Vector of struct, to nicely pass into a session!
pub fn new_generic_from_file(filename:&str) -> Vec<GenericStringStruct>{
    let reader = BufReader::new(generic_from_file(filename));
    let lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok) 
        .collect();
    CandidateGenerator::from_lines(lines)
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
        self.freq
    }

    fn exec(&self) -> String {
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
            .map(|line | {
                let mut candidate = GenericStringStruct::new(&line, 0);
                if !line.is_ascii() {
                    candidate.mode = SearchMode::Unicode;
                }
                candidate
            })
        .collect()
    }
}
