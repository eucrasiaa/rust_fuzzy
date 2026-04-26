use super::FuzzyApp;
use crate::fuzzy::{SimilarityAlgorithm,FuzzyCandidate};
use std::time::Duration;
use std::io::Result;

use crossterm::
    event::{
        self, KeyEventKind,KeyEvent,
        Event, KeyCode, poll
    }
;

impl<'a, T, A> FuzzyApp<'a, T, A>
where
    T: FuzzyCandidate,
    A: SimilarityAlgorithm,
{
    /// keystroke events run every frame in run()
    // pub(crate) fn handle_events(&mut self) ->Result<()> { 
    //
    //     if poll(Duration::from_millis(16))?{
    //         if let Event::Key(key) = event::read()? {
    //             // Only handle "Press" events (avoids double-counting on Windows)
    //             if key.kind == KeyEventKind::Press {
    //                 match key.code {
    //                     KeyCode::Char(c)   =>  self.kp_basic_char(c),
    //                     KeyCode::Backspace =>  self.kp_backspace(),
    //                     KeyCode::Enter     =>  self.toggles[0]= !self.toggles[0], //toggle
    //                     KeyCode::Up        =>  self.kp_arrow_up(),
    //                     KeyCode::Down      =>  self.kp_arrow_down(),
    //                     KeyCode::Esc       =>  self.exit = true,
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    //         // print_events(&mut self.keystrokes)
    //         //todo!()
    // }
    // TODO TMP FOR DEBUGGING PROFILING!
    pub(crate) fn handle_events(&mut self) -> Result<()> {
        if self.is_profiling && !self.mock_keys.is_empty() {
            let key = self.mock_keys.remove(0); 
            self.process_key(key);
            // sleep(Duration::from_millis(16));
            return Ok(());
        }
        // TODO consider just allowing for blocking? any reason not to?
        if poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    self.process_key(key);
                }
                _ => (),
            }
        }
        Ok(())

    }
    fn process_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c)   => self.kp_basic_char(c),
            KeyCode::Backspace => self.kp_backspace(),
            KeyCode::Enter     => self.toggles[0] = !self.toggles[0],
            KeyCode::Up        => self.kp_arrow_up(),
            KeyCode::Down      => self.kp_arrow_down(),
            KeyCode::Esc       => self.exit = true,
            _ => {}
        }
    }
        
    /// checks if results = 0, and if so, set the cursor to 0 = not visible/hovering
    /// run every change to results 
    fn sync_cursor(&mut self){
        self.hover_index = if self.session.num_results > 0 { 1 } else { 0 };
    }
    /// sync displayed list of outputs (scrolling)
    #[inline]
    pub (crate) fn sync_scroll(&mut self, num_line:usize){
        // let new_index = self.hover_index-1;
        // //up
        // if new_index < self.scroll_index {
        //     self.scroll_index = new_index;
        // } 
        // //down
        // // dont let us scroll off page?
        // else if (self.session.num_results <=  self.scroll_index + num_line) && new_index >= self.scroll_index + num_line {
        //     self.scroll_index = new_index+ 1 - num_line;
        // }
        // self.list_state.select(Some(new_index));
        // //todo!();
        if self.session.num_results == 0 {
            self.scroll_index = 0;
            self.list_state.select(None);
            return;
        }

        let new_index = self.hover_index.saturating_sub(1);
        //TODO a lil chopped? fix
        // new_index + 1 - num_line <= scroll_index <= new_index (top bound)
        self.scroll_index = self.scroll_index
            .min(new_index)
            .max(new_index.saturating_add(5).saturating_sub(num_line));
        self.list_state.select(Some(new_index));

    }
    fn kp_basic_char(&mut self, c:char){
        self.list_state.select_first();
        self.session.type_char(c); 
        self.sync_cursor();
    }
    fn kp_backspace(&mut self){
        self.list_state.select_first();
        // println!("backsopaced");
        // if no results, 0. else, 1
        self.session.backspace(); 
        self.hover_index = (self.session.num_results > 0) as usize;

    }
    fn kp_arrow_up(&mut self){
        self.list_state.select_previous();
        self.hover_index = self.hover_index.saturating_sub(1).max(1);
        
    }
    fn kp_arrow_down(&mut self){
        self.list_state.select_next();
        self.hover_index = (self.hover_index + 1).min(self.session.num_results);
    }
}
