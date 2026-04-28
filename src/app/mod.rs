/*!
# mostly self contained. just create via new, and init it with a session!

*/

// pub mod state;
// pub use self::state::*;
mod draw;   
mod events;
use events::*;
use draw::*;
use crate::fuzzy::{SearchSession, SimilarityAlgorithm, FuzzyCandidate,ScoredResult};
use crossterm::{
    event::{Event, DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture,KeyEvent},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{DefaultTerminal};
use ratatui::widgets::ListState;
use std::fmt::{Write, Arguments};
use std::io::{Result,stdout};



/// DebugStrings is a psudo bump allocator primarily used for the output tui interface. i guess.
/// could be used for other things.  
pub struct DebugStrings{
    num_debugs: usize,
    char_capacity: usize,
    debug_total_bufs: Vec<String>,
    open_str:usize,
}

impl DebugStrings {
    pub fn new() -> Self {
        let num_debugs = 10;
        let char_capacity = 40;

        // 10 strings
        let mut debug_total_bufs = Vec::with_capacity(num_debugs);
        // preallcoate each to length 40
        for _ in 0..num_debugs {
            debug_total_bufs.push(String::with_capacity(char_capacity));
        }

        Self {
            num_debugs,
            char_capacity,
            debug_total_bufs,
            open_str:0
        }
    }
    pub fn reset(&mut self) {
        for s in &mut self.debug_total_bufs {
            //keeps the alloc but sets all to empty
            s.clear();
        }
        self.open_str = 0;
    }
    
    pub fn next_buf(&mut self) -> &mut String {
        let idx = self.open_str;
        //wrap to prevent panic? also clear indicator you need to add more
        self.open_str = (self.open_str + 1) % self.num_debugs; 
        &mut self.debug_total_bufs[idx]
    }
    // maybe?
    #[inline]
    pub fn push_debug(&mut self, args: Arguments) {
        let buf = self.next_buf();
        buf.clear();
        buf.write_fmt(args).expect("String write failed");
    }
    pub fn valid_strings(&self) -> &[String] {
        &self.debug_total_bufs[0..self.open_str]
    }
}

pub struct FuzzyApp<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    pub session: SearchSession<'a, T, A>,

    // tui state stuff
    /// 0 = nothing selected, 1-based otherwise
    pub hover_index: usize, 
    pub exit: bool,
    /// 0 = nothing, 1+ = shifted down # outputs. reset on any keystroke
    pub scroll_index: usize, 
    pub toggles: [bool; 10],
    pub list_state: ratatui::widgets::ListState,
    pub debug_strs: DebugStrings,

    // pub current_results: Vec<ScoredResult<'a, T>>,
    pub mock_keys: Vec<KeyEvent>,
    pub is_profiling: bool,
}

impl<'a, T, A> FuzzyApp<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    pub fn new(session: SearchSession<'a, T, A>) -> Self {
        Self {
            session,
            hover_index: 0,
            exit: false,
            scroll_index:0,
            toggles: [false; 10],
            list_state: ListState::default(),
            debug_strs: DebugStrings::new(),
            is_profiling: false,
            mock_keys: Vec::new(),
        }
    }
    pub fn init(&mut self) -> Result<()>{

        enable_raw_mode()?; // from cooked -> raw 
        let mut terminal_out = stdout();
        execute!(terminal_out, EnterAlternateScreen)?;
        let result = ratatui::run(|terminal| self.run(terminal));
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        result
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        execute!(stdout(), EnableMouseCapture, EnableFocusChange)?;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;  // defined in draw.rs
            self.handle_events()?;                      // defined in events.rs
        }
        execute!(stdout(), DisableMouseCapture, DisableFocusChange)?;
        Ok(())
    }
}
