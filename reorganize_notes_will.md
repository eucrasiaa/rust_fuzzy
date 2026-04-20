

Critically need to break up and generic a lot of stuff.
Session is the fundamental object, stores:
```

pub struct SearchSession<'a,T:FuzzyCandidate,S:SimilarityAlgorithm>{
	candidate_structs: &'a [T],
	matcher: S,
    current_query: String,
    history: Vec<(
        Vec<ScoredResult<'a, T>>, 
        i64,                      
        usize                     
    )>, 
    current_threshold: i64,
    num_results: usize,
}

		impl<T> SearchSession<T> 
		where 
		    T: SomeTrait + AnotherTrait 
		{

pub struct SearchSession<'a,T,A>
candidates: &'a [T]
history: Vec<Vec<ScoredResult>>
threshold_stack: Vec<f64>
current_threshold: f64
pub current_results()
pub type_char() / backspace()


```

maybe?
![[Pasted image 20260420101805.png]]
src/
- fuzzy/
	- mod rs
	- algorithm.rs   <- SimilarityAlgo trait + impls
	- canidate.rs  <- FuzzyCanidate, ScoreTarget, ScoredResult
	- matcher.rs  <- FuzzyMatcher (all config, no mutable session state)
	- session.rs  <- SearchSession \<T> SHOULD BE GENERIC. owns history + thresh stack
- app/
	- mod.rs   <- exports pub API
	- state.rs    <- App struct, Ui-only state (cursor, toggles, exit)
	- draw.rs     <- duh
	- events.rs  <- handle_events()
- entities/
	- desktop.rs   <- DesktopEntity+FuzzyCanidate ipl
	- animals.rs  <- AnimalEnt





