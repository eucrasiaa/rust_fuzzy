// src/lib.rs

/*!

# Will's Fuzzy Algorithm Library
### for linux (?) primarily. not tested on other devices lol
`will_fuzzy` is a quite fast, low runtime allocation high efficienty fuzzy matching engine and
psudo-TUI framework.

This library provides the core search algorithms and session management required to 
build custom fuzzy finders. additionally, by following the trait patterns defined, custom
algorithms could be inserted as long as they match the required functions!
it additionally defines structures to allow for any struct to implement a fuzzy search across
it, allowing weighting of different fields as well!



## Workflow

### 1. Define an entity list to be searched against
*   Either use `GenericStringStruct` + `CandidateGenerator::from_lines(Vec<String>)`
*   Or implement the `FuzzyCandidate` trait for a custom struct:
    *   `fn search_targets(&self) -> Vec<ScoreTarget>;` (use `ScoreTarget::new()`)
    *   `fn display_text(&self) -> &str;`

### 2. Pick an algorithm 
Currently, only `AlgoWillGreedyVer2` is supported.
```rust
let matcher = FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new());
```

### 3. Generate a session
```rust
let session = SearchSession::create(&entities, matcher, String::new());
```

### 4. Interface with the session
You can interact with the session directly via:
*   `.top_results(amt, offset)`
*   `.set_query(str)`
*   `.type_char(c)` — *Primarily for use in interactive forms!*

### 5. Use the TUI Interface
Alternatively, use the built-in TUI framework:
```rust
let mut fuzzy_app = FuzzyApp::new(session);
fuzzy_app.init()?;
```
*/
pub mod app;
pub mod fuzzy;
pub mod entities;
pub use fuzzy::canidate::{FuzzyCandidate, ScoreTarget, ScoredResult};
pub use fuzzy::session::SearchSession;
pub use fuzzy::matcher::FuzzyMatcher;
pub use fuzzy::algorithms::algo_greedy_v2::AlgoWillGreedyVer2;
pub use entities::{DesktopEntity,GenericStringStruct,AnimalEnt};

/// technically usable on non-ascii, i haven't tested it extensively. so. just incase
#[derive(Debug, Clone, Copy, Default)]
pub enum SearchMode{
    // lets use the standard &[u8] algo
    #[default]
    Ascii,  
    //else
    Unicode, 
}

//
// fn strings_to_events(inputs: Vec<&str>) -> Vec<KeyEvent> {
//     inputs.into_iter().flat_map(|s| {
//         let chars = s.chars().map(|c| KeyCode::Char(c));
//         let bksps = (0..s.len()).map(|_| KeyCode::Backspace);
//
//         chars.chain(bksps).map(|code| KeyEvent::new(code, KeyModifiers::NONE))
//     }).collect()
// }
