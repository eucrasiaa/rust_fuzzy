// mod old_main;
// use old_main::*;
mod fuzzy;
mod app;
use app::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
// use fuzzy::{AlgoWillBasicGreedyVer1, SimilarityAlgorithm};
use fuzzy::*;
use std::fmt;
use crate::fuzzy::session::SearchSession;
use ini::Ini;
use std::io::Result;
use std::fs;
use std::path::Path;


use std::fs::File;
use std::io::{BufRead, BufReader};


#[inline(always)]
pub fn scale_weight(f: f64) -> i64 {
    (f * 1024.0).round() as i64
}



#[derive(Debug)]
pub struct DesktopEntity{
    /// id = filename.desktop
    id: String,
    /// name = "Executable Desktop Nice Name" - "Strawberry"
    name: String,
    /// generic_name = ""  - "Strawberry Music Player"
    generic_name: String,
    /// desc = "description of it"
    desc: String,
    /// exec = "exec commands + args" - "strawberry %U"
    exec: String,
    /// tags<Vec Str> = []  - "\[AudioVideo\]\[Player\]\[Qt\]\[Audio\];" 
    tags: Vec<String>,
    /// freq? todo
    launch_count: i64,
    precompute_str: String
}

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


impl FuzzyCandidate for AnimalEnt{
    fn search_targets(&self) -> Vec<ScoreTarget>{
        let targets = vec![
            ScoreTarget { text: &self.name, weight_multiplier: scale_weight(1.0), exact_match_only: false},
        ];
        targets
    }
    fn usage_bonus(&self) -> i64{
        self.freq + 5
    }
    fn display_text(&self) -> &str{
        &self.precompute_str
    }
}


impl FuzzyCandidate for DesktopEntity {
    fn search_targets(&self) -> Vec<ScoreTarget> {
        // let mut targets = vec![
        //     ScoreTarget { text: &self.name, weight_multiplier: 1.0, exact_match_only: false },
        // ];
        let mut targets = Vec::with_capacity(1+self.tags.len());
        //generic w/ penalty
        targets.push(ScoreTarget {
            text: &self.name,
            weight_multiplier: scale_weight(1.0),
            exact_match_only: false,
        });
        for tag in &self.tags {
            targets.push(ScoreTarget { 
                text: tag, 
                weight_multiplier: scale_weight(0.1), 
                exact_match_only: true
            });
        }
        targets
    }
    fn usage_bonus(&self) -> i64 {
        //tweak this heavy LOL
        (self.launch_count as f64 * 1.2) as i64 
    }
    fn display_candidate(&self) -> String {
        format!("{} ({})", self.name, self.exec)
    }
    fn display_text(&self) -> &str{
        &self.precompute_str
    }
}



impl DesktopEntity {
    pub fn from_file(path: &Path) -> Option<Self> {
        // Load the INI file
        let conf = Ini::load_from_file(path).ok()?;
        let section = conf.section(Some("Desktop Entry"))?;
        let get_str = |key: &str| -> String {
            section.get(key).unwrap_or("").to_string()
        };

        let name = get_str("Name");
        let generic_name = get_str("GenericName");
        let precompute_str = format!("{} - {}", name, generic_name);
        let tags: Vec<String> = section
            .get("Categories")
            .map(|s| {
                s.split(';')
                    .filter(|tag| !tag.is_empty())
                    .map(|tag| tag.to_string())
                    .collect()
            })
            .unwrap_or_default();

        Some(DesktopEntity {
            id: path.file_name()?.to_string_lossy().to_string(),
            name,
            generic_name, 
            desc: get_str("Comment"),
            exec: get_str("Exec"),
            tags,
            launch_count: 0,
            precompute_str,
        })
    }
}

impl fmt::Display for DesktopEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - [{}]", self.name, self.tags.join(", "))
    }
}

fn main() -> Result<()>{
    // old_main::main().unwrap();
    // let new_greedy = AlgoWillBasicGreedyVer1::default();
     // let applications_dir = Path::new("/usr/share/applications/");

    // // Iterate over the directory
    // if let Ok(entries) = fs::read_dir(applications_dir) {
    //     for entry in entries.flatten() {
    //         let path = entry.path();
    //         if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
    //             if let Some(entity) = DesktopEntity::from_file(&path) {
    //                 println!("Loaded: {} ({})", entity.name, entity.id);
    //                 println!("Tags: {:?}", entity.tags);
    //             }
    //         }
    //     }
    // }
    // let entities: Vec<DesktopEntity> = fs::read_dir(applications_dir)
    // .ok()
    // .into_iter()
    // .flat_map(|entries| entries.flatten())
    // .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "desktop"))
    // .filter_map(|entry| DesktopEntity::from_file(&entry.path()))
    // .collect();
    //

    let file = File::open("justnames.txt").expect("Could not open file");
    // let file = File::open("animallist.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let animals: Vec<AnimalEnt> = reader
        .lines()
        .map_while(Result::ok) // just incase? yells otherwisie 
        .map(|name| AnimalEnt {
            name: name.trim().to_string(),
            freq: 1, // basically ignore 
            precompute_str: format!("{} - slop",name.trim())
        })
    .collect();


    let mut session:SearchSession::<AnimalEnt, AlgoWillBasicGreedyVer1>= SearchSession::new(
        &animals,
        FuzzyMatcher::with_algo(AlgoWillBasicGreedyVer1::new()), 
        String::new(),
        Vec::new(),
        0,
        0
    );

    let mut my_fuzzy_app = FuzzyApp::new(session);
    my_fuzzy_app.is_profiling= true;
    let demo_strings_1 = vec!["eeee","bbbb","blackbird","aaaaaaaa","--aa--aa--","aeiou","abcdefghijk"];
    let demo_strings_2 = vec![
    "Axolotl",            
    "Gnu",                
    "Hummingbird",
    "aaeeaaee",
    " Buffalo",      
    "Crow\n",             
    "BlackCat",         
    "C0nd0r",             
    "fiissh...",           
    "adeeeeal",      
];
    my_fuzzy_app.mock_keys = [
        strings_to_events(demo_strings_1),
        strings_to_events(demo_strings_2),
    ].into_iter().flatten().collect();
    // my_fuzzy_app.mock_keys.push(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    my_fuzzy_app.init()
}
fn strings_to_events(inputs: Vec<&str>) -> Vec<KeyEvent> {
    inputs.into_iter().flat_map(|s| {
        let chars = s.chars().map(|c| KeyCode::Char(c));
        let bksps = (0..s.len()).map(|_| KeyCode::Backspace);
        
        chars.chain(bksps).map(|code| KeyEvent::new(code, KeyModifiers::NONE))
    }).collect()
}
