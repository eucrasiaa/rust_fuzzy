// mod old_main;
// use old_main::*;
use will_fuzzy::fuzzy::algorithms::AlgoWillGreedyVer2;
use will_fuzzy::fuzzy::canidate::{FuzzyCandidate, ScoreTarget};
use will_fuzzy::fuzzy::session::SearchSession;
use will_fuzzy::fuzzy::matcher::FuzzyMatcher;
use will_fuzzy::app::FuzzyApp;
// mod algorithms;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fmt;
use ini::Ini;
use std::io::Result;
use std::path::Path;

use std::fs;


use std::fs::File;
use std::io::{BufRead, BufReader};


fn run_ui(entities: Vec<DesktopEntity>) -> Result<()> {
    let is_ascii = true;
    let session = SearchSession::<DesktopEntity, AlgoWillGreedyVer2>::new(
        &entities,
        FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new()), 
        String::new(),
        Vec::new(),
        0,
        0,
        is_ascii
    );

    let mut my_fuzzy_app = FuzzyApp::new(session);
    my_fuzzy_app.init()
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
    precompute_str: String,
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


impl FuzzyCandidate for DesktopEntity {
    fn search_targets(&self) -> Vec<ScoreTarget> {
        // let mut targets = vec![
        //     ScoreTarget { text: &self.name, weight_multiplier: 1.0, exact_match_only: false },
        // ];
        let mut targets = Vec::with_capacity(1+self.tags.len());
        //generic w/ penalty
        targets.push(
            ScoreTarget::new(&self.name, 1.0, false)
        );
        for tag in &self.tags {
            targets.push(
            ScoreTarget::new(tag, 0.1, true)
            );
        }
        targets
    }
    fn exec(&self) -> String{
        self.exec.to_string()
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
    let mut args = std::env::args().skip(1);
    // old_main::main().unwrap();
    // let new_greedy = AlgoWillBasicGreedyVer1::default();
    // let applications_dir = Path::new("/usr/share/applications/");
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "".into());

    let desktop_paths_as_per_arch_wiki = vec![
        format!("{}/.local/share/applications", home_dir), // User apps
        "/usr/share/applications".to_string(),             // System apps
        "/usr/local/share/applications".to_string(),       // Local system apps
    ];
let entities: Vec<DesktopEntity> = desktop_paths_as_per_arch_wiki.iter()
    .map(Path::new)
    // 1. Turn the list of dirs into a stream of entries
    .flat_map(|path| fs::read_dir(path).ok().into_iter().flatten())
    .flatten() // Flatten the DirEntry results
    // 2. Filter for .desktop files
    .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("desktop"))
    // 3. Try to parse each file
    .filter_map(|entry| DesktopEntity::from_file(&entry.path()))
    // 4. Collect into your final Vec
    .collect();
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
    //     .ok()
    //     .into_iter()
    //     .flat_map(|entries| entries.flatten())
    //     .filter(|entry| entry.path().extension().map_or_else(|| false, |ext| ext == "desktop"))
    //     .filter_map(|entry| DesktopEntity::from_file(&entry.path()))
    //     .collect();
    //
    let mut is_ascii = true;


    let mut filename = String::new();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--input" => {
                    if let Some(val) = args.next() {
                        filename = val;
                    }
            }
            "-h" | "--help" => {
            println!("MY (wills) AWEsome amazing fuzzy finder application");
            println!("/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\");
            println!("./will_fuzzy <flags>");
            println!();
            println!("  -h, --help      shows this ");
            println!("  --animals       reads off the provided animal_names.txt");
            println!("  --input <FILE>  not well tested but should be able to just read in a text file lol");
            println!();
            std::process::exit(0);
        }
            "--animals"
                if filename.is_empty() => {
                    filename = "animal_names.txt".to_string();
                }
            _ => {}
        }
    }
        if !filename.is_empty() {
            let file = match File::open(&filename) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening {}: {}", filename, e);
                    return Err(e);
                }
            };
        
        
        let reader = BufReader::new(file);


        let animals: Vec<AnimalEnt> = reader
            .lines()
            .map_while(Result::ok) // just incase? yells otherwisie 
            .map(|name| {
                let trimmed = name.trim().to_string();
                if !trimmed.is_ascii() {
                    is_ascii = false;
                }
                AnimalEnt {
                    name: name.trim().to_string(),
                    freq: 1, // basically ignore 
                    precompute_str: name.trim().to_string(),
                    // precompute_str: format!("{}",name.trim())
                }
            }

            )
            .collect();

        let session = SearchSession::<AnimalEnt, AlgoWillGreedyVer2>::new(
            &animals,
            FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new()),
            String::new(), Vec::new(), 0, 0, is_ascii
        );
        let mut fuzzy_app = FuzzyApp::new(session);
        fuzzy_app.init()?;
    } else {
        let session = SearchSession::<DesktopEntity, AlgoWillGreedyVer2>::new(
            &entities,
            FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new()),
            String::new(), Vec::new(), 0, 0, is_ascii
        );
        let mut fuzzy_app = FuzzyApp::new(session);
        fuzzy_app.init()?; 
    }
    Ok(())
        // let mut my_fuzzy_app = FuzzyApp::new(session);
        //     my_fuzzy_app.is_profiling= true;
        //     let demo_strings_1 = vec!["eeee","bbbb","blackbird","aaaaaaaa","--aa--aa--","aeiou","abcdefghijk"];
//     let demo_strings_2 = vec![
//     "Axolotl",            
//     "Gnu",                
//     "Hummingbird",
//     "aaeeaaee",
//     " Buffalo",      
//     "Crow\n",             
//     "BlackCat",         
//     "C0nd0r",             
//     "fiissh...",           
//     "adeeeeal",      
// ];
//     my_fuzzy_app.mock_keys = [
//         strings_to_events(demo_strings_1),
//         strings_to_events(demo_strings_2),
//     ].into_iter().flatten().collect();
//
    // let ffox = "Firefox".to_string();
    // let ffox2 = ffox.as_bytes();
    // let q = "fi".to_string();
    // let q2 = q.as_bytes();
    // println!("{}",session.matcher.algorithm.one_step_calc(ffox2,q2));
    // Ok(())
//     my_fuzzy_app.mock_keys.push(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    // my_fuzzy_app.init()


    // //
    // let target = "123 Generic Fuzzy Tester Street, House #4, Maryland, USA".to_ascii_lowercase();
    // let target_bytes = target.as_bytes();
    // let queries = vec![
    //     "123 Ge Fuzz Testr St Hs 4 MD USA".to_ascii_lowercase(),
    //     "3 eneric uzzy ester treet ouse 4 aryland usa".to_ascii_lowercase(),
    //     "123 gfts h4 md usa".to_ascii_lowercase(),     
    //     "maryland usa".to_ascii_lowercase(),           
    //     "999 non-existent".to_ascii_lowercase(),       
    //     "123-generic_fuzzy".to_ascii_lowercase(),      
    // ];

    // for q_str in queries {
    //     let query_bytes = q_str.as_bytes();
    //
    //     // If you want the debug trace we built:
    //     let mut reporter = VerboseReporter { steps: Vec::new() };
    //     let score = session.matcher.algorithm.one_step_calc(target_bytes, query_bytes, &mut reporter);
    //
    //     println!("\n--- Testing Query: '{}' ---", q_str);
    //     for step in reporter.steps {
    //         println!("{}", step);
    //     }
    //     println!("Final Calculated Score: {}", score);
    // }
    // Ok(())
}
fn strings_to_events(inputs: Vec<&str>) -> Vec<KeyEvent> {
    inputs.into_iter().flat_map(|s| {
        let chars = s.chars().map(|c| KeyCode::Char(c));
        let bksps = (0..s.len()).map(|_| KeyCode::Backspace);

        chars.chain(bksps).map(|code| KeyEvent::new(code, KeyModifiers::NONE))
    }).collect()
}
