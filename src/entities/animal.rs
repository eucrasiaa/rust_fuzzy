use std::io::ErrorKind;

use super::prelude::*;
pub struct AnimalEnt{
    name:String,
    freq:i64,
    precompute_str: String
} 

impl fmt::Display for AnimalEnt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (freq: {:.2})", self.name, self.freq)
    }
}


fn read_animal_file() -> File{
    match File::open("animal_names.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("Error: File not found! Ensure the animal_names.txt is alongside the binary.");
                std::process::exit(1); 
            }
            _ => {
                eprintln!("Error: Failed to open file: {:?}", error);
                std::process::exit(1);
            }
        },
    }
}


pub fn animal_demo() -> Vec<AnimalEnt>{
        let reader = BufReader::new(read_animal_file());
        let animals: Vec<AnimalEnt> = reader
            .lines()
            .map_while(Result::ok) // just incase? yells otherwisie 
            .map(|name| {
                // let trimmed = name.trim().to_string();
                AnimalEnt {
                    name: name.trim().to_string(),
                    freq: 1, // basically ignore 
                    precompute_str: name.trim().to_string(),
                    // precompute_str: format!("{}",name.trim())
                }
            }

            )
            .collect();
    animals
}

impl FuzzyCandidate for AnimalEnt{
    fn search_targets(&self) -> Vec<ScoreTarget>{
        let targets = vec![
            ScoreTarget::new(&self.name, 1.0, false)
           // ScoreTarget::new text: &self.name, weight_multiplier: scale_weight(1.0), exact_match_only: false},
        ];
        targets
    }
    fn usage_bonus(&self) -> i64{
        self.freq + 5
    }
    fn exec(&self) -> String{
        "\0".to_string()
    }
    fn display_text(&self) -> &str{
        &self.precompute_str
    }
}
