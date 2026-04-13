use std::process::Command;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

use std::cmp::Ordering;

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

    launch_count: f64
}


pub struct ScoreTarget<'a> {
    pub text: &'a str,
    pub weight_multiplier: f64, // the weight
    pub exact_match_only: bool, // for smthn like tags? idk
}
pub trait FuzzyCandidate {
    // for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    // from use statistics, later include ig
    fn usage_bonus(&self) -> f64;
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
        // e.g., Every time it's launched, add 5 points to the final score
        self.launch_count as f64 * 1.2 
    }
}

fn main() {
    println!("Hello, world!");
}
