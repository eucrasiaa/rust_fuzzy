
use super::DebugTui;
use crate::fuzzy::{SimilarityAlgorithm,FuzzyCandidate};
use std::{thread::sleep, time::Duration};
use std::io::Result;

use crossterm::
event::{
    self, KeyEventKind,KeyEvent,
    Event, KeyCode, poll
}
;

impl<'a, T, A> DebugTui<'a, T, A>
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
        if self.mock_keys.is_empty() {
            let key = self.mock_keys.remove(0); 
            self.process_key(key);
            sleep(Duration::from_millis(60));
            return Ok(());
        }

        Ok(())
    }
    fn process_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c)   => self.kp_basic_char(c),
            KeyCode::Backspace => self.kp_backspace(),
            KeyCode::Esc       => self.exit = true,
            _ => {}
        }
    }

    fn kp_basic_char(&mut self, c:char){
        self.type_char(c); 
    }

    pub fn type_char(&mut self, c: char) {
        self.query.push(c);
 

    }
    fn kp_backspace(&mut self){
        // println!("backsopaced");
        // if no results, 0. else, 1
        self.backspace(); 

    }

    fn is_separator(&self, c: char) -> bool {
        matches!(c, ' ' | '-' | '_')
    }

    fn find_match_indices(&self, query: &str, target: &str) -> Option<Vec<usize>> {
        let mut matched_indexes:Vec<usize> = Vec::with_capacity(query.len()); 
        // pre known size, might help?
        // to fast fail, if first char doesnt match, return instantly
        let mut query_chars = query.chars();
        // iter type 
        let mut current_q = query_chars.next()?;
        // let mut current_q = match query_chars.next() 
        //     Some(c) => c,
        //     None => return None,

        for (byte_idx, t_char) in target.char_indices() {
            let is_match = t_char.eq_ignore_ascii_case(&current_q) 
                || (self.is_separator(t_char) && self.is_separator(current_q));
            if is_match {
                matched_indexes.push(byte_idx);

                if let Some(next_q) = query_chars.next() {
                    current_q = next_q;
                } else {
                    // Found all query chars in sequence!
                    return Some(matched_indexes);
                }
            }
        }
        None
    }

    //string + indexes, score it
    fn calculate_score(&self, target:&str, indices: &[usize]) -> i64 {
        // 0 matches at all? drop it
        if indices.is_empty() { return 0; }
        let bytes = target.as_bytes();
        let mut score = 0;
        // if is_empty() passed, then we know 0 will always exist, so safe
        if indices[0] == 0 { 
            score += self.bonus_start; 
        }
        // if indices.contains(&0) { score +=self.bonus_start; };
        //TODO this feels like a prime loop unrolling thing bc of 2 distinct  i > 0 cases
        // no thats stupid... trust the compiler  Will cmonn
        for i in 0..indices.len() {
            let current_idx = indices[i];

            // consec only matters past elem 1 
            // if i > 0 && current_idx == indices[i - 1] + 1 {
            //     score += self.bonus_consec;
            // }
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
        //DEBUG_PRINT
        // if score != 0{
        //     let joined = scores_why.iter()
        //             .map(|n| n.to_string())
        //             .collect::<Vec<String>>()
        //             .join(", ");
        //     println!("scored {} with {}", target, joined);
        // }
        score
    }

}
