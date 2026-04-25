// pub mod state;
// pub use self::state::*;
mod draw;   
mod events;
use events::*;
use draw::*;
use crate::fuzzy::{SearchSession, SimilarityAlgorithm, FuzzyCandidate};
use crossterm::{
    event::{DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::DefaultTerminal;

use std::io::{Result,stdout};

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
