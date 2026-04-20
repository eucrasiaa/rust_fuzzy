use std::default;
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
    style::{Color, Style, Stylize},
    layout::{Constraint, Direction, Layout},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Block, Paragraph, Widget,ListItem, List, Clear},
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


pub struct FZFGreedy {
    pub bonus_bound: i32,
    pub bonus_consec: i32,
    pub bonus_start: i32,
    pub string: String,
}

pub struct DebugList {
    pub texts: Vec<FZFGreedy>,
    pub last_str: String,
}

impl Default for DebugList{
    fn default() -> Self {
        DebugList{
            texts: Vec::new(),
            last_str: String::new(),
        }
    }
}
//
// let debug_lister = Mutex::new(DebugList);
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
    // Source: original string we search Within
    // target: the user typed string we match with 
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
        // Source: original string we search Within
        // target: the user typed string we match with 
        // 0.0
        let indexies = self.find_match_indices(target, source).unwrap_or_default();
        // let to_pass = match indexies  {
        //     Some(idx) => idx,
        //     None => vec![],
        // }; 
        // println!("{:?}",indexies);

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

    //byte index of matches (the fast fail here!)
    fn find_match_indices(&self, query: &str, target: &str) -> Option<Vec<usize>> {
        let mut matched_indexes:Vec<usize> = Vec::with_capacity(query.len()); // pre known size,
                                                                              // might help?
                                                                              //to fast fail, if first char doesnt match, return instantly
                                                                              // println!("Query: {} + Target: {}",query, target);
        let mut query_chars = query.chars();
        // iter type 
        let mut current_q = match query_chars.next() {
            Some(c) => c,
            None => return None,
            //{
            //     println!("{} - scored NONE?",query);
            //     return None
            // }, 
        };


        for (byte_idx, char) in target.char_indices() {
            //if char.to_ascii_lowercase() == current_q.to_ascii_lowercase() 
            if char.eq_ignore_ascii_case(&current_q) {
                // push match index to vec
                matched_indexes.push(byte_idx);
                //step next &  return when out of char
                if let Some(next_q) = query_chars.next() {
                    current_q = next_q;
                } else {
                    return Some(matched_indexes);
                }
            }
        }
        None
            // todo!()
    }
    
    //string + indexes, score it
    fn calculate_score(&self, target:&str, indices: &[usize]) -> f64 {
        // 0 matches at all? drop it
        if indices.is_empty() { return 0.0; }
        let bytes = target.as_bytes();
        let mut score = 0.0;
        // if is_empty() passed, then we know 0 will always exist, so safe
        if indices[0] == 0 { 
            score += self.bonus_start; 
        }
        // if indices.contains(&0) { score +=self.bonus_start; };
        //TODO this feels like a prime loop unrolling thing bc of 2 distinct  i > 0 cases

        for i in 0..indices.len() {
            let current_idx = indices[i];

            // consec only matters past elem 1 
            if i > 0 && current_idx == indices[i - 1] + 1 {
                score += self.bonus_consec;
            }

            // score += (curr == prev + 1) as i32 * self.bonus_consec;
            if current_idx > 0 {
                let prev_byte = bytes[current_idx - 1];
                // bounds = ' ', -, or _
                // let is_bound = (prev_byte == b' ') | (prev_byte == b'-') | (prev_byte == b'_');
                // score += (is_bound as i32 as f64) * self.bonus_bound;
                if prev_byte == b' ' || prev_byte == b'-' || prev_byte == b'_' {
                    score += self.bonus_bound;
                }
            }
        }
        //
        //
        // for window in indices.windows(2) {
        //     let current = window[0];
        //     let next = window[1];
        //
        //     //consec
        //     if next == current +1 {
        //         score += self.bonus_consec;
        //     }
        //
        //     //bound
        //     if current > 0 {
        //         let prev_byte = bytes[current - 1];
        //         if prev_byte == b' ' || prev_byte == b'-' {
        //             score += self.bonus_bound;
        //         }
        //     }
        // }

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

    //assumes sorted
    // pub fn update_thresh<'a, T>(&mut self, results: &[ScoredResult<'a, T>]) -> f64 {
    pub fn update_thresh<'a, T>(&mut self, results: &[ScoredResult<'a, T>])  {

        let mut max_diff = -1.0;
        let mut thrsh = -1.0;
        // if results.is_empty() { return self.threshold }
        if results.is_empty() { return }
        for (window) in results.windows(2){  
            let prev = &window[0];
            let curr = &window[1];
            let diff = prev.score - curr.score;
            if diff > max_diff {
                max_diff = diff;
                thrsh = curr.score;
                // best_idx = i + 1;
            }
        }
        self.threshold = thrsh-10.0;

        // if let Some(res) = results.get(best_idx) {
        // self.threshold = res.score;
        // res.score
        // } 
        // thrsh
    }

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



// |||||||||||||||||||||||||||||||||
/// desktop stuff for testing its work!
// |||||||||||||||||||||||||||||||||
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
    thresh_len_hist: Vec<(f64,usize)>,
    num_results: usize,
    hover_index: usize,   

    current_query: String,
}

//Todo genuine slop. fix this LOL
impl<'a> DemoSession<'a> {
    pub fn type_char<A: SimilarityAlgorithm>(&mut self, c: char, matcher: &mut FuzzyMatcher<A>) {
        self.current_query.push(c);

        let candidates_to_search: &[AnimalEnt] = if let Some(last_results) = self.history.last() {
            self.list_strings 
        } else {
            self.list_strings
        };
        let new_results = matcher.search(&self.current_query, candidates_to_search);
        self.thresh_len_hist.push( (matcher.threshold, self.num_results) );
        self.num_results = new_results.len();
        matcher.update_thresh(&new_results);
        self.history.push(new_results);

    }
    pub fn backspace(&mut self) {
        if !self.current_query.is_empty() {
            self.current_query.pop();
            self.history.pop(); 
        }
        // if !self.thresh_hist.is_empty(){
        //
        // }
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
    keystrokes: Vec<String>, 
    toggles: Vec<i32>,
    // debug_list: DebugList,
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
                Constraint::Percentage(15), // left
                Constraint::Percentage(90), // bot
            ]).split(frame.area());

            let bot_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70), // left
                    Constraint::Percentage(30), // bot
                ]).split(chunks[1]);

                let title = Line::from(" Animal Searcher ").bold();
                let total_line = self.session.list_strings.len();
                let active_line = self.session.current_results().len();
                let dropped_line = total_line-active_line;
                // let title_bottom = Line::from(format!("[total:{}] [active:{}] [dropped:{}]",)).yellow();
                let title_bottom = Line::from(vec![
                    Span::styled(format!("[total:{}]",total_line),Style::default().fg(Color::Red)),
                    Span::styled(format!("[active:{}]",active_line), Style::default().fg(Color::Green)),
                    Span::styled(format!("[dropped:{}]",dropped_line), Style::default().fg(Color::Yellow)),
                    // Span::from("!"),
                ]);

                let tui_total_line = ListItem::new(
                    Span::styled(format!("[total: {}]",total_line),Style::default().fg(Color::Red)).into_centered_line());        
                let tui_active_line = ListItem::new(
                    Span::styled(format!("[active: {}]",active_line), Style::default().fg(Color::Green)).into_centered_line());
                let tui_dropped_line = ListItem::new(
                    Span::styled(format!("[dropped: {}]",dropped_line), Style::default().fg(Color::Yellow)).into_centered_line());

                let tui_hovering_index = ListItem::new(
                    Span::styled(format!("[hover index: {}]",self.session.hover_index), Style::default().fg(Color::LightGreen)).into_centered_line());
                let tui_threshold_value = ListItem::new(
                    Span::styled(format!("[treshold: {}]",self.matcher.threshold), Style::default().fg(Color::LightRed)).into_centered_line());


                let tui_str_hovering_value = if self.session.hover_index==0{
                    String::from("No String Hovered")
                } else {
                    match self.session.current_results().get(self.session.hover_index-1){
                        Some(k) => format!("{k}"),
                        None => String::from("Error Iterating String")
                    }
                };
                let tui_hovering_value = ListItem::new(
                    Span::styled(format!("[selected: {}]",tui_str_hovering_value), Style::default().fg(Color::Cyan)).into_centered_line());




                let tui_db_list_vec = vec![
                    tui_total_line,tui_active_line,tui_dropped_line,
                    tui_threshold_value,
                    tui_hovering_index, tui_hovering_value
                ];
                let db_title=Line::from(" Debug Statistics ").bold();

                let tui_db_list =  List::new(tui_db_list_vec)
                    .block(
                        Block::bordered()
                        .title(db_title.centered().bg(Color::White).fg(Color::Black))
                        .border_set(border::THICK)
                    );

                // RENDER RENDER
                frame.render_widget(tui_db_list, bot_chunks[1]);

                // RENDER RENDER

                // self.list_strings.len();
                // let title = Line::from(format!(" Animal Searcher [non-cull: {}, thresh: {}]", self.session.list_strings.len(), self.matcher.threshold).bold());

                let block = Block::bordered()
                    .title(title.centered())
                    // .title_bottom(title_bottom.centered())
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
            .enumerate()
            // .enumerVate() //gives i
            .map(|(i, result)| {
                if i+1 == self.session.hover_index {
                    ListItem::new(format!("{}",result)).bg(Color::White).fg(Color::Black)
                }
                else {
                    ListItem::new(format!("{}",result))
                }
            })
        .collect();

        let event_list = List::new(list_items)
            .block(Block::bordered().title(" Results "));
        // .highlight_symbol(">> "); // Optional: if you add selection logic later
        frame.render_widget(event_list, bot_chunks[0]);
        // frame.render_widget(event_list, chunks[1]);

        if self.toggles[0] == 1{
            let popup_block = Block::bordered().title("Popup");
            let centered_area = frame.area().centered(Constraint::Percentage(60), Constraint::Percentage(20));
            // clears out any background in the area before rendering the popup
            frame.render_widget(Clear, centered_area);
            let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);
            frame.render_widget(paragraph, centered_area); 
        }
    }
    fn manage_result_cursor(&mut self, arrow_dir:KeyCode){
        if self.session.hover_index == 0{ return}
        if arrow_dir==KeyCode::Up{
            if self.session.hover_index > 1 {
                self.session.hover_index-=1;
            }
        } 
        if arrow_dir==KeyCode::Down{
            if self.session.hover_index -1 < self.session.num_results{
                self.session.hover_index+=1;
            }


        }
    }
    fn handle_events(&mut self) -> io::Result<()> { 

        if poll(Duration::from_millis(16))?{
            if let Event::Key(key) = event::read()? {
                // Only handle "Press" events (avoids double-counting on Windows)
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            self.keystrokes.push(c.to_string());
                            self.session.type_char(c,&mut self.matcher); 
                            //self.session.hover_index=0;
                            if self.session.num_results > 0{
                                self.session.hover_index =1
                            }
                            else{
                                self.session.hover_index =0;
                            }

                        }
                        KeyCode::Backspace => {
                            self.keystrokes.pop();
                            self.session.backspace(); 
                            let thresh_len=self.session.thresh_len_hist.pop().unwrap_or_default();
                            self.matcher.threshold = thresh_len.0;
                            self.session.num_results = thresh_len.1;
                            if self.session.num_results > 0{
                                self.session.hover_index =1
                            }
                            else{
                                self.session.hover_index =0;
                            }
                        }
                        KeyCode::Enter => self.toggles.insert(0,self.toggles[0]^1),
                        KeyCode::Up => self.manage_result_cursor(KeyCode::Up),
                        KeyCode::Down => self.manage_result_cursor(KeyCode::Down),


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



pub fn main()-> io::Result<()>  {


    let file = File::open("justnames.txt").expect("Could not open file");
    // let file = File::open("animallist.txt").expect("Could not open file");
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
    let mut show_selected = false;
    let mut app = App{
        session:
            DemoSession{
                list_strings: &animals, 
                history: Vec::new(),
                current_query: String::new(),
                thresh_len_hist: Vec::new(),
                num_results: 0,
                hover_index: 0,
            },

            matcher:FuzzyMatcher::with_algo(SimAlgoGreedyV1::new()),
            // matcher:FuzzyMatcher::with_algo(SimAlgoOtherAlgo),
            exit:false,
            keystrokes: Vec::new(),
            toggles: vec![0,0,0,0,0,0],
            // debug_list: DebugList::default(),
    };
    // app.matcher.set_thresh(10.0);
    app.matcher.threshold=5.0;
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
