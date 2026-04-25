use super::FuzzyApp;
use crate::fuzzy::{SimilarityAlgorithm,FuzzyCandidate};
use std::time::Duration;
use std::io::Result;

use crossterm::{
    event::{
        self, KeyEventKind,
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

impl<'a, T, A> FuzzyApp<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    /// keystroke events run every frame in run()
    pub(crate) fn handle_events(&mut self) ->Result<()> { 

        if poll(Duration::from_millis(16))?{
            if let Event::Key(key) = event::read()? {
                // Only handle "Press" events (avoids double-counting on Windows)
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c)   =>  self.kp_basic_char(c),
                        KeyCode::Backspace =>  self.kp_backspace(),
                        KeyCode::Enter     =>  self.toggles[0]= !self.toggles[0], //toggle
                        KeyCode::Up        =>  self.kp_arrow_up(),
                        KeyCode::Down      =>  self.kp_arrow_down(),
                        KeyCode::Esc       =>  self.exit = true,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
            // print_events(&mut self.keystrokes)
            //todo!()
    }
    /// checks if results = 0, and if so, set the cursor to 0 = not visible/hovering
    /// run every change to results 
    fn sync_cursor(&mut self){
        self.hover_index = if self.session.num_results > 0 { 1 } else { 0 };
    }
    /// sync displayed list of outputs (scrolling)
    fn sync_scroll(&mut self){
        todo!();
    }
    fn kp_basic_char(&mut self, c:char){
        self.session.type_char(c); 
        self.sync_cursor();
    }
    fn kp_backspace(&mut self){
        // println!("backsopaced");
        self.session.backspace(); 
    }
    fn kp_arrow_up(&mut self){
        if self.hover_index > 1 {
            self.hover_index -= 1;
        }
        
    }
    fn kp_arrow_down(&mut self){
       if self.hover_index < self.session.num_results {
            self.hover_index += 1;
       }
    }
}
