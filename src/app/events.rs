use super::FuzzyApp;
use crate::fuzzy::{SimilarityAlgorithm,FuzzyCandidate};
use std::process::Stdio;
use std::{process::Command, time::Duration};
use std::io::Result;
use fork::{daemon, Fork};
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
            // KeyCode::Enter     => self.toggles[0] = !self.toggles[0],
            KeyCode::Enter     => self.kp_enter(),
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

    fn kp_enter(&mut self) {
        if let Some(res) = self.session.current_results().get(self.hover_index.saturating_sub(1)) {
            let exec_str = res.item.exec();
            // non exec, toggle [2] which is just display text
            if exec_str == "!" {
                self.toggles[2] = !self.toggles[2];
            }
            else{
            if self.toggles[1] || exec_str == "!" {
                self.toggles[1] = false;
            }
            else{
                self.toggles[1] = !self.toggles[1];
            }
            }

            
        }
    }
    /// sync displayed list of outputs (scrolling)
    // this function is so evilllll
    #[inline]
    pub (crate) fn sync_scroll(&mut self, num_line:usize){
        // core logic:
        // if no results, dont do anything early return 
        if self.session.num_results == 0 {
            self.scroll_index = 0;
            self.list_state.select(None);
            return;
        }
        // results? start by tracking current hovered index
        // sub 1 b/c hover is one indexed for 0 marking none
        let new_index = self.hover_index.saturating_sub(1);
        // Ensure we don't select past the actual results
        // set the furthest down to the length -1 (b/c len isnt 0 indexed) 
        let max_possible_index = self.session.num_results.saturating_sub(1);
        let clamped_index = new_index.min(max_possible_index);

        // actually scroll
        // upwards: cursor goes above top line, move up. straightforward
        if clamped_index < self.scroll_index {
            self.scroll_index = clamped_index;

        //downward: if we hit the number visible + scroll index = bottom line of the code
        } else if clamped_index >= self.scroll_index + num_line {
            // we do the clampted + 1 to go down, and - num_line b/c 
            // of the weird range scrlling thing we do
            self.scroll_index = clamped_index + 1 - num_line;
        }
        // b/c we only are "rendering" a slice for optimization sake, we need to pretend for calcs
        // that the whole list is all there, needs to be manually handled as opposed to just the
        // basic select 
        self.list_state.select(Some(clamped_index-self.scroll_index));

    }
    fn kp_basic_char(&mut self, c:char){
        self.list_state.select_first();
        self.toggles[1]=false;
        self.toggles[2]=false;
        self.session.type_char(c); 
        self.sync_cursor();
    }
    fn kp_backspace(&mut self){
        self.list_state.select_first();
        // println!("backsopaced");
                self.toggles[1]=false;
                self.toggles[2]=false;

        // if no results, 0. else, 1
        self.session.backspace(); 
        self.hover_index = (self.session.num_results > 0) as usize;

    }
    fn kp_arrow_up(&mut self){
        // self.list_state.select_previous();
        self.hover_index = self.hover_index.saturating_sub(1).max(1);
        
    }
    fn kp_arrow_down(&mut self){
        // self.list_state.select_next();
        self.hover_index = (self.hover_index + 1).min(self.session.num_results);
    }
}

