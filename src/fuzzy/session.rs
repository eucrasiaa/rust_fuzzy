use super::FuzzyMatcher;
use super::SimilarityAlgorithm;
use super::canidate::*;



/// A Session is used for live character by character fuzzy searching.
/// sessions exist to enable character by character typing searching wrapper for an algorithm


struct InternalSerSesStats{
    len_canidates: usize,
    hist_length: usize,
    query_prefix: char,
    len_query: usize,
    // len_results:usize,

}
impl Default for InternalSerSesStats{
    fn default() -> Self {
        Self{len_canidates:0,hist_length:0, query_prefix: '\0', len_query: 0}
    }
}


#[derive(Debug)]
enum KeyStrokeUpdate {
    STANDARDCHAR,
    BACKSPACE,
    COMMAND,
    UNKNOWN,
}
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

    /// current statusu
    pub current_results: Vec<ScoredResult<'a, T>>,
    pub current_threshold: i64,
    pub num_results: usize,
    internal_state: InternalSerSesStats,
}

impl<'a,T,S> SearchSession<'a,T,S> 
where 
    T: FuzzyCandidate,
    S: SimilarityAlgorithm,
{
    /// initializer. standard
    pub fn new(candidate_structs: &'a [T], matcher: FuzzyMatcher<S>, current_query: String, history: Vec<(
            Vec<ScoredResult<'a, T>>, 
            i64,                      
            usize                     
    )>, current_threshold: i64, num_results: usize) -> Self {
        Self { candidate_structs, matcher, current_query, history, current_threshold, 
            num_results, 
            current_results: Vec::new(),
            internal_state: InternalSerSesStats { 
                len_canidates: candidate_structs.len(), 
                hist_length: 0, 
                query_prefix: '\0',
                len_query: 0,
                // len_results: 0,
            },
        }
    }

    /// TODID that !!!
    pub fn type_char(&mut self, c: char) {
        self.current_query.push(c); 


        // let candidates_to_search: Vec<&T> = if let Some((last_results, _score, _index)) = self.history.last() {
        //     // self.candidate_structs.iter().collect()
        //     last_results.iter().map(|res| res.item).collect()
        // } else {
        //     self.candidate_structs.iter().collect()
        // };
        // if history = empty, its the first press so  
        let candidates_to_search: Vec<&T> = if self.history.is_empty() {
            self.candidate_structs.iter().collect()
        } else {
            self.current_results.iter().map(|res| res.item).collect()
        };


        // !! generate updated list
        let new_results = self.matcher.search(&self.current_query, &candidates_to_search, self.current_threshold);
        let tmp_thresh = self.matcher.update_thresh(&new_results);
        let new_thresh = if tmp_thresh == -1 { self.current_threshold } else { tmp_thresh };
        let new_length = new_results.len();


        // swapp?
        let old_results = std::mem::replace(&mut self.current_results, new_results);
        self.history.push((old_results, self.current_threshold, self.num_results));
        
        // update current! 
        self.current_threshold = new_thresh;
        self.num_results = new_length;
    }


    fn append_internal(&mut self, type_update:KeyStrokeUpdate){
        match type_update {
            KeyStrokeUpdate::STANDARDCHAR => {
                self.internal_state.len_query += 1;
                    if self.internal_state.len_query == 1 {
                        self.internal_state.query_prefix = self.current_query.chars().next().unwrap_or('\0');
                    }
                    self.internal_state.hist_length = self.internal_state.len_query;
            },
            KeyStrokeUpdate::BACKSPACE => {
                if self.internal_state.len_query > 0 {
                    self.internal_state.len_query -= 1;
                    if self.internal_state.len_query == 0 {
                        self.internal_state.query_prefix = '\0';
                    }
                }
                self.internal_state.hist_length = self.internal_state.len_query;
            },
            KeyStrokeUpdate::COMMAND =>{
                todo!()
            }
            KeyStrokeUpdate::UNKNOWN => {
                self.update_internal();
            }
        }
    }
    /// if full sync needed. generally will be updated off a backpace or type_char call
    fn update_internal(&mut self){
        self.internal_state.len_query = self.current_query.len();
        self.internal_state.query_prefix = if self.internal_state.len_query > 0{
            // validate length before, only chars can be pushed to this will always be safe
            self.current_query.chars().nth(0).unwrap_or('\0')
        }
        else{
            '\0'
        };
        self.internal_state.hist_length = self.current_query.len();

    }
    pub fn backspace(&mut self) {
    // try to pop from query
    if self.current_query.pop().is_some() {
        
        // yo wtf was i DOING heree LMFAO??? no wonder it was bugging out crazy
        if let Some((past_results, past_thresh, past_length)) = self.history.pop(){
            // load past
            self.current_results = past_results;
            self.current_threshold = past_thresh;
            self.num_results = past_length;
        } else {
            // fallback error case?
            //  is there a need to make a helper probably not 
            self.history.clear();
            self.current_threshold = 0;
            self.num_results = self.internal_state.len_canidates;
            self.current_results.clear();
        }
    }

    // cant pop, so we nuke history and reset to initial state?
    // memoization... theres gotta be somethjing here 
    if self.current_query.is_empty() {
        self.history.clear();
        self.current_threshold = 0;
        self.num_results = self.internal_state.len_canidates;
        self.current_results.clear();
        // self.current_results = self.candidate_structs
        //     .iter()
        //     .map(|item| ScoredResult { item, score: 0 })
        //     .collect();
        
    }
    self.append_internal(KeyStrokeUpdate::BACKSPACE);

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

    
    pub fn curr_thresh(&self) -> i64{
        self.current_threshold
    }
    pub fn current_results(&self) -> &[ScoredResult<'a, T>] {
         self.current_results.iter().as_slice()
         // self.history.last().map(|(v,_past_thresh,_past_length)| v.as_slice()).unwrap_or(&[])
    }

    // pub fn current_results_range(&self, start:usize, limit:usize) -> &[ScoredResult<'a, T>] {
    //     self.history.last().iter()
    //         .skip(start)
    //         .take(limit)
    //         .for_each(|(v,_past_thresh,_past_length)| v.as_slice()).unwrap_or(&[])
    //
    // }

    // mostly debug stuff at this point
    pub fn len_canidates(&self)->usize{
        self.internal_state.len_canidates

    }
    pub fn len_query(&self)->usize {
        self.internal_state.len_query
    }
    pub fn query_prefix(&self)->char {
        self.internal_state.query_prefix
    }
    pub fn history_length(&self)->usize{
        self.internal_state.hist_length
    }
    pub fn current_query(&self)->&str{
        &self.current_query
    }
    // pub fn len_results(&self)->usize{
    //     self.internal_state.len_results
    // }
    pub fn display_history(&self) -> Vec<String> {
        let mut to_ret: Vec<String> = Vec::new();
        
        for hist_item in self.history.iter() {
            let (results_vec, b, c) = hist_item; 

            let formatted_results = results_vec
                .iter()
                .map(|res| format!("{}", res)) 
                .collect::<Vec<String>>()
                .join(", "); 
            to_ret.push(format!("{}, {}, [{}]", b, c, formatted_results));
        }
        
        to_ret
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
