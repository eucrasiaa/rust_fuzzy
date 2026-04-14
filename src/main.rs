use std::process::Command;
use crossterm::event::{self, KeyEventKind};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::io::stdout;

use std::fmt::{self, Display};
use std::{time::Duration, io};
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp::Ordering;




use crossterm::{
    event::{
        DisableFocusChange, DisableMouseCapture,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, poll
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    layout::{Constraint, Direction, Layout},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget,ListItem, List},
    DefaultTerminal, Frame,
};
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
impl<'a, T> fmt::Display for ScoredResult<'a, T> 
where 
    T: fmt::Display 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.2}] {}", self.score, self.item)
    }
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

pub struct SimAlgoGreedyV1{
    pub bonus_bound:f64,
    pub bonus_consec:f64,
    pub bonus_start:f64,
}
impl SimilarityAlgorithm for SimAlgoGreedyV1 {

    fn score(&self, source: &str, target: &str) -> f64 {
        0.0 
    }
}

impl Default for SimAlgoGreedyV1 {
    fn default() -> Self {
        Self {
            bonus_bound: 10.0,
            bonus_consec: 4.0,
            bonus_start: 20.0,
        }
    }
}

impl SimAlgoGreedyV1 {
    pub fn new() -> Self {
        Self::default()
    }

    //byte index of matches (the fast fail here!)
    fn find_match_indices(&self, query: &str, target: &str) -> Option<Vec<usize>> {
        let mut matched_indexes:Vec<usize> = Vec::new();
        //iter over targert search in queue
        for (byte_idx, char) in query.char_indices() {
            if target.contains(char) {
                matched_indexes.push(byte_idx);
            }
        }
        Some(matched_indexes)

        // todo!()
    }
    
    //string + indexes, score it
    fn calculate_score(&self, target:&str, indices: &[usize]) -> f64 {
        if indices.is_empty() { return 0.0; }
        let bytes = target.as_bytes();
        let mut score = 0.0;
        if indices.contains(&0) { score +=self.bonus_start; }
        for window in indices.windows(2) {
            let current = window[0];
            let next = window[1];
            
            //consec
            if next == current +1 {
                score += self.bonus_consec;
            }

            //bound
            if current > 0 {
                let prev_byte = bytes[current - 1];
                if prev_byte == b' ' || prev_byte == b'-' {
                    score += self.bonus_bound;
                }
            }
        }

        score
    }
}
// fn fastFail:






pub struct SimAlgoOtherAlgo;
impl SimilarityAlgorithm for SimAlgoOtherAlgo {
    fn score(&self, source: &str, target: &str) -> f64 {
        self.other_calc(source, target) 
    }
}

impl SimAlgoOtherAlgo{

    fn other_calc(&self, a: &str, b: &str) -> f64 {
        let len_a = a.chars().count();
        let len_b = b.chars().count();

        if len_a == 0 && len_b == 0 { return 100.0; }

        let max_len = std::cmp::max(len_a, len_b) as f64;
        let distance = self.other_recurse(a, b) as f64;
        (1.0 - (distance / max_len)) * 100.0
    }

    fn other_recurse(&self, a: &str, b: &str) -> usize {
        if a.is_empty() { return b.chars().count(); }
        if b.is_empty() { return a.chars().count(); }
        let mut a_chars = a.chars();
        let mut b_chars = b.chars();
        let a_head = a_chars.next().unwrap();
        let b_head = b_chars.next().unwrap();
        let a_tail = a_chars.as_str();
        let b_tail = b_chars.as_str();
        if a_head == b_head {
            self.other_recurse(a_tail, b_tail)
        } else {
            1 + std::cmp::min(
                self.other_recurse(a_tail, b),    
                std::cmp::min(
                    self.other_recurse(a, b_tail), 
                    self.other_recurse(a_tail, b_tail) 
                )
            )
        }
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

    // pub fn set_thresh(mut self, threshold: f64) -> Self {
    //     self.threshold = threshold;
    //     self
    // }
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

impl fmt::Display for DesktopEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) - {}", self.name, self.generic_name, self.desc)
    }
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

pub struct DemoSession<'a> {
    list_strings: &'a [AnimalEnt],

    // my lovely stack
    history: Vec<Vec<ScoredResult<'a, AnimalEnt>>>,

    current_query: String,
}

//Todo genuine slop. fix this LOL
impl<'a> DemoSession<'a> {
    pub fn type_char<A: SimilarityAlgorithm>(&mut self, c: char, matcher: &FuzzyMatcher<A>) {
        self.current_query.push(c);

        let candidates_to_search: &[AnimalEnt] = if let Some(last_results) = self.history.last() {
            self.list_strings 
        } else {
            self.list_strings
        };
        let new_results = matcher.search(&self.current_query, candidates_to_search);
        self.history.push(new_results);
    }
    pub fn backspace(&mut self) {
        if !self.current_query.is_empty() {
            self.current_query.pop();
            self.history.pop(); 
        }
    }

    pub fn current_results(&self) -> &[ScoredResult<'a, AnimalEnt>] {
        self.history.last().map(|v| v.as_slice()).unwrap_or(&[])
    }
}

// dead basic ratatui thing
//

pub struct App<'a, S: SimilarityAlgorithm> {
    // data_list: Vec<AnimalEnt>,
    session: DemoSession<'a>,
    matcher: FuzzyMatcher<S>,
    exit: bool,
    keystrokes: Vec<String> 

}

impl<'a, S> App<'a, S> 
where 
    S: SimilarityAlgorithm 
{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        execute!(io::stdout(), EnableMouseCapture, EnableFocusChange)?;
        // when run called, loop until the app's exit varible is set!
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        execute!(io::stdout(), DisableMouseCapture, DisableFocusChange)?;
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {

        // frame.render_widget(self, frame.area());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10), // left
                Constraint::Percentage(90), // bot
            ]).split(frame.area());

            let title = Line::from(" Animal Searcher ".bold());

            let block = Block::bordered()
                .title(title.centered())
                .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
                "Search: ".into(),
                self.session.current_query.to_string().yellow(),
        ])]);

        let para = Paragraph::new(counter_text)
            .centered()
            .block(block);
        frame.render_widget(para,chunks[0]);

        let list_items: Vec<ListItem> = self.session.current_results()
            .iter()
            // .enumerVate() //gives i
            .map(|result| {
                ListItem::new(format!("{}",result))
            })
        .collect();

        let event_list = List::new(list_items)
            .block(Block::bordered().title(" Results "));
        // .highlight_symbol(">> "); // Optional: if you add selection logic later

        frame.render_widget(event_list, chunks[1]);

    }
    fn handle_events(&mut self) -> io::Result<()> { 

        if poll(Duration::from_millis(16))?{
            if let Event::Key(key) = event::read()? {
                // Only handle "Press" events (avoids double-counting on Windows)
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            self.keystrokes.push(c.to_string());
                            self.session.type_char(c,&self.matcher); 
                        }
                        KeyCode::Backspace => {
                            self.keystrokes.pop();
                            self.session.backspace(); 
                        }
                        KeyCode::Esc => {
                            self.exit = true;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
            // print_events(&mut self.keystrokes)
            //todo!()
    }
}



fn main()-> io::Result<()>  {


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

    // let mut test_sess = DemoSession {
    //     list_strings: &animals, 
        //     history: Vec::new(),
        //     current_query: String::new(),
        // };
        //
    // let matcher = FuzzyMatcher::with_algo(SimAlgoOtherAlgo);

    let mut app = App{
        session:
            DemoSession{
                list_strings: &animals, 
                history: Vec::new(),
                current_query: String::new()
            },
            matcher:FuzzyMatcher::with_algo(SimAlgoOtherAlgo),
            exit:false,
            keystrokes: Vec::new(),
    };
    // app.matcher.set_thresh(10.0);
    app.matcher.threshold=10.0;
    enable_raw_mode()?; // from cooked -> raw 
    let mut terminal_out = stdout();
    execute!(terminal_out, EnterAlternateScreen)?;
    // println!("{:?}", App::default());
    let result = ratatui::run(|terminal| app.run(terminal));
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    result
        // test_sess.type_char('e', &matcher);
        // print_results(&test_sess);    

}

// fn print_results(sess: &DemoSession) {
//     println!("Query: '{}'", sess.current_query);
//     for res in sess.current_results() {
//         println!("  {}: {:.2}", res.item.name, res.score);
//     }
// }
