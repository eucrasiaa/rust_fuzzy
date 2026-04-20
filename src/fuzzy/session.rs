use crate::fuzzy::FuzzyMatcher;

use super::algorithm::{SimilarityAlgorithm};
use super::canidate::*;

/// A Session is used for live character by character fuzzy searching.
pub struct SearchSession<'a,T:FuzzyCandidate,S:SimilarityAlgorithm>{
    /// An array of a structure with FuzzyCanidate Implemented. Critical to decide which values of
    /// the struct are used in the searching
    candidate_structs: &'a [T],
    matcher: FuzzyMatcher<S>,
    current_query: String,
    ///History: to enable quick undoing for character based Searching.
    ///ScoredResult: previous results
    ///i64: previous threshhold (optional)  
    ///usize: previous output length(optional) (some small optimizations possible?)
    history: Vec<(
        Vec<ScoredResult<'a, T>>, 
        i64,                      
        usize                     
    )>, 
    pub current_threshold: i64,
    pub num_results: usize,
}

impl<'a,T,S> SearchSession<'a,T,S> 
where 
    T: FuzzyCandidate+ std::fmt::Display,
    S: SimilarityAlgorithm,
{
    pub fn new(candidate_structs: &'a [T], matcher: FuzzyMatcher<S>, current_query: String, history: Vec<(
            Vec<ScoredResult<'a, T>>, 
            i64,                      
            usize                     
        )>, current_threshold: i64, num_results: usize) -> Self {
        Self { candidate_structs, matcher, current_query, history, current_threshold, num_results }
    }

    pub fn type_char<A: SimilarityAlgorithm>(&mut self, c: char) {
        self.current_query.push(c); 
        // if there is a history last (culled list of strings) operate on that (to save effeciency)
        let candidates_to_search: Vec<&T> = if let Some((last_results, _score, _index)) = self.history.last() {
            last_results.iter().map(|res| res.item).collect()
        } else {
            self.candidate_structs.iter().collect()
        };
        //DEBUG_PRINT
        //  let joined = candidates_to_search.iter()
        //     .map(|n| n.to_string())
        //     .collect::<Vec<String>>()
        //     .join("\n ");
        //
        // println!("canidates: {}",joined);
        let new_results = self.matcher.search(&self.current_query, &candidates_to_search,self.current_threshold);
        //DEBUG_PRINT
        // for result in &new_results {
        //     println!("{}", result); 
        // }
        let tmp_thresh = self.matcher.update_thresh(&new_results);
        let tmp_length = new_results.len();

        self.history.push((new_results,tmp_thresh,tmp_length))

    }
    pub fn backspace(&mut self) {
        if !self.current_query.is_empty()  && let Some((_, thresh, length)) = self.history.pop() {
            self.current_threshold = thresh;
            self.num_results = length;
        }
    } 
                        // KeyCode::Backspace => {
                        //     self.keystrokes.pop();
                        //     self.session.backspace(); 
                        //     let thresh_len=self.session.thresh_len_hist.pop().unwrap_or_default();
                        //     self.matcher.threshold = thresh_len.0;
                        //     self.session.num_results = thresh_len.1;
                        //     if self.session.num_results > 0{
                        //         self.session.hover_index =1
                        //     }
                        //     else{
                        //         self.session.hover_index =0;
                        //     }
                        // }
    

    pub fn current_results(&self) -> &[ScoredResult<'a, T>] {
        self.history.last().map(|(v,_past_thresh,_past_length)| v.as_slice()).unwrap_or(&[])
    }


}



// impl<'a> DemoSession<'a> {
//     pub fn type_char<A: SimilarityAlgorithm>(&mut self, c: char, matcher: &mut FuzzyMatcher<A>) {
//         self.current_query.push(c);
//
//         let candidates_to_search: &[AnimalEnt] = if let Some(last_results) = self.history.last() {
//             self.list_strings 
//         } else {
//             self.list_strings
//         };
//         let new_results = matcher.search(&self.current_query, candidates_to_search,self.threshold);
//         self.num_results = new_results.len();
//         matcher.update_thresh(&new_results);
//         self.history.push(new_results);
//
//     }
//     pub fn backspace(&mut self) {
//         if !self.current_query.is_empty() {
//             self.current_query.pop();
//             self.history.pop(); 
//         }
//         // if !self.thresh_hist.is_empty(){
//         //
//         // }
//     }
//
//     pub fn current_results(&self) -> &[ScoredResult<'a, AnimalEnt>] {
//         self.history.last().map(|v| v.as_slice()).unwrap_or(&[])
//     }
// }
//
//
// impl<T> SearchSession<T> 
// where 
//     T: SomeTrait + AnotherTrait 
// {
