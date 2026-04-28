
use super::prelude::*;
use ini::Ini;
use std::fs;



/// base parsing of the file. probably should add more safety checks lol
pub fn parse_desktop_inis() -> Vec<DesktopEntity>{
    
    // uh. maybe. idk just wanna be sure
    if !cfg!(target_os = "linux") {
        panic!("This program must be run on Linux!");
    }
    


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
    entities
}


// #[derive(Debug)]
///holder for the Desktop Ini file information!
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

impl FuzzyCandidate for DesktopEntity {
    fn search_targets(&self) -> Vec<ScoreTarget> {
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
