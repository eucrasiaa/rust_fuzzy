
mod draw;   
mod event;
use event::*;
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




pub struct DebugTui<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    pub session: SearchSession<'a, T, A>,
    pub scroll_index: usize,
    pub exit: bool,
    pub toggles: [bool; 10],
    pub list_state: ratatui::widgets::ListState,
    pub mock_keys: Vec<KeyEvent>,
    pub bonus_bound:i64,
    pub bonus_consec:i64,
    pub bonus_start:i64,
    pub output_buff_1: String,
    pub output_buff_2: String,
    pub query: String,
    
        
}

impl<'a, T, A> DebugTui<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    pub fn new(session: SearchSession<'a, T, A>) -> Self {
        Self {
            session,
            scroll_index:0,
            exit:false,
            toggles: [false; 10],
            list_state: ListState::default(),
            mock_keys: Vec::new(),
            bonus_bound:10,
            bonus_consec:10,
            bonus_start:20,
            output_buff_1: String::with_capacity(100),
            output_buff_2: String::with_capacity(100),
            query: String::new(),
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
