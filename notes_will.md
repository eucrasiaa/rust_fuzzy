


### Original Structs Layout:
```rust

// the struct containing each string involved in a search. 
// base list populated on init
// needs heavy rework
pub struct ScoreTarget<'a> {
    pub text: &'a str,
    pub weight_multiplier: f64, 
    pub exact_match_only: bool, // where should this be tied to? feels... 
							    // troublesome down the line
}

//using a trait to define parts of a struct to score + weights
pub trait FuzzyCandidate {
    // for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    // from use statistics, later include ig
    fn usage_bonus(&self) -> f64;
}
```


``` rust
// struct returned off a fzzy
pub struct ScoredResult<'a, T> {
    pub item: &'a T,
    pub score: f64,
}
impl<'a, T> fmt::Display for ScoredResult<'a, T> 
where 
    T: fmt::Display 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "[{:.2}] {}", self.score, self.item)
    }
}
```

Algorithms + Traits
```rust
// define a new algorithm by defining a struct, that does the "score" function.
// it can manage itself, store variables, etc.  
pub trait SimilarityAlgorithm {
    // Source: original string we search Within
    // target: the user typed string we match with 
    fn score(&self, source: &str, target: &str) -> f64;
}
// the SimAlgoGreedyV1
pub struct SimAlgoGreedyV1{
    pub bonus_bound:f64,
    pub bonus_consec:f64,
    pub bonus_start:f64,
}
impl SimilarityAlgorithm for SimAlgoGreedyV1 {
    fn score(&self, source: &str, target: &str) -> f64 {
        let indexies = self.find_match_indices(target, source).unwrap_or_default();
        self.calculate_score(source, &indexies)
    }
}
impl Default for SimAlgoGreedyV1 {
    fn default() -> Self {
        Self {
            bonus_bound:  10.0,
            bonus_consec: 20.0,
            bonus_start: 20.0,
        }
    }
}

impl SimAlgoGreedyV1 {
    pub fn new() -> Self {
        Self::default()
    }
    fn find_match_indices(&self, query: &str, target: &str) -> Option<Vec<usize>> { todo!() } 
    fn calculate_score(&self, target:&str, indices: &[usize]) -> f64 { todo!() }
}

```


FuzzyMatcher Type (slop? remake?)
```rust
// part of session, stores 
#[derive(Default)]
pub struct FuzzyMatcher<A: SimilarityAlgorithm> {
    pub algorithm: A,
    pub threshold: f64,        
    pub case_sensitive: bool, // never used, but fix that?
}


impl FuzzyMatcher<SimAlgoFzfStyle> {
    pub fn new() -> Self {todo!()}
}


impl<A: SimilarityAlgorithm> FuzzyMatcher<A> {
    pub fn with_algo(algorithm: A) -> Self {
        Self {
            algorithm,
            threshold: 0.0,
            case_sensitive: false,
        }
    }
    pub fn update_thresh<'a, T>(&mut self, results: &[ScoredResult<'a, T>])  {
        // some sort of threshhold culling algo. currently just
        // an iter + find largest jump and cut there
	        // ex: generally cuts all non first letter matching at 
	        // first b/c jump big there 
        self.threshold = thrsh-10.0;
    }
    //remake this one lol
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
                            // let a = self.algorithm.score(target.text, query);
                            // println!("{a}: {}",target.text);
                            // a
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
```

entities. desktop + animals
```rust 
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
    launch_count: i64
}

impl fmt::Display for DesktopEntity {}
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

pub struct AnimalEnt{
    name:String,
    freq:f64,
} 

impl fmt::Display for AnimalEnt {}
pub struct AnimalEnt{
    name:String,
    freq:f64,
} 

impl fmt::Display for AnimalEnt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (freq: {:.2})", self.name, self.freq)
    }
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


```


REMAKE SESSION TO BE MORE GENERIC!!!!
```rust
pub struct SearchSession<'a> {
    all_apps: &'a [DesktopEntity],
    history: Vec<Vec<ScoredResult<'a, DesktopEntity>>>,
    current_query: String,
}
pub struct DemoSession<'a> {
    list_strings: &'a [AnimalEnt],
    // my lovely stack
    history: Vec<Vec<ScoredResult<'a, AnimalEnt>>>,
    thresh_len_hist: Vec<(f64,usize)>,  //this is foul.
    num_results: usize,
    hover_index: usize,   
    current_query: String,
}
impl<'a> DemoSession<'a> {
    pub fn type_char<A: SimilarityAlgorithm>(&mut self, c: char, matcher: &mut FuzzyMatcher<A>) {}
    pub fn backspace(&mut self) {}
    pub fn current_results(&self) -> &[ScoredResult<'a, AnimalEnt>] {}
}
```
app
```rust
pub struct App<'a, S: SimilarityAlgorithm> {
    session: DemoSession<'a>,
    matcher: FuzzyMatcher<S>,
    exit: bool,
    keystrokes: Vec<String>, 
    toggles: Vec<i32>,
}

impl<'a, S> App<'a, S> 
where 
    S: SimilarityAlgorithm 
{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {}
    fn draw(&self, frame: &mut Frame) {}
    // lowk rework keystroke thing!
	fn manage_result_cursor(&mut self, arrow_dir:KeyCode){}
    fn handle_events(&mut self) -> io::Result<()> {}
}


```

desktop file locations:

/usr/share/applications/
-- system wide
/usr/local/share/applications/
-- smallest list here
~/.local/share/applications/
-- user apps

using std command spawn to just let ownership leave scope? 

https://doc.rust-lang.org/std/process/index.html


matching and weights:
Desktop struct:
  id = filename.desktop
  name = "Executable Desktop Nice Name" - "Strawberry"
  generic_name = ""  - "Strawberry Music Player"
  desc = "description of it"
  exec = "exec commands + args" - "strawberry %U"
  tags<Vec Str> = []  - "\[AudioVideo\]\[Player\]\[Qt\]\[Audio\];"
  use-weight = f64 - more launched more often pushed up? store somewhere 

maybe store freq count of launched times and add to weight?


weight ent:
pub struct ScoredApp<'a> {
    pub app: &'a DesktopEntry,
    pub score: f64, 
}

assumedly id have some sort of stack of ScoredAppVectors to do the pop on backspace
```
[Desktop Entry]
Version=1.0
Type=Application
Name=Strawberry
GenericName=Strawberry Music Player
GenericName[fr]=Lecteur de musique Strawberry
GenericName[ru]=Музыкальный проигрыватель Strawberry
Comment=Plays music
Comment[fr]=Joue de la musique
Comment[ru]=Прослушивание музыки
Exec=strawberry %U
TryExec=strawberry
Icon=strawberry
Terminal=false
Categories=AudioVideo;Player;Qt;Audio;
Keywords=Audio;Player;Clementine;
MimeType=x-content/audio-player;application/ogg;application/x-ogg;application/x-ogm-audio;audio/flac;audio/ogg;audio/vorbis;audio/aac;audio/mp4;audio/mpeg;audio/mpegurl;audio/vnd.rn-realaudio;audio/x-flac;audio/x-oggflac;audio/x-vorbis;audio/x-vorbis+ogg;audio/x-speex;audio/x-wav;audio/x-wavpack;audio/x-ape;audio/x-mp3;audio/x-mpeg;audio/x-mpegurl;audio/x-ms-wma;audio/x-musepack;audio/x-pn-realaudio;audio/x-scpls;video/x-ms-asf;x-scheme-handler/tidal;
StartupWMClass=strawberry
Actions=Play-Pause;Stop;StopAfterCurrent;Previous;Next;
```


engine


look into traits + impl on struct?

struct holds config (algo, case_sensitive, etc)
passed a list? make a resulting score values?
return sorted?


subset caching prob

algo prob look more at like Greedy Matcher + Smith-Waterman for shorthand vs error checking in things like Levenshtein Distance or Sørensen-Dice for similar



``` rust
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
```
so like for desktop ent


Algorithm:

looking into the FZF v1 greedy:

step 1
fast fail:
query = "abuc"
target = "a buzy camel"
we hit all 4 in string, so keep it
else? drop it
store indexes of match

step 2
then scoring thing:
first match at index 0? massive bonus
index after a space or dash? bonus
consec indexes?

