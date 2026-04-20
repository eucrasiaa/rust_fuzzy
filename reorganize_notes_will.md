

Critically need to break up and generic a lot of stuff.
Session is the fundamental object, stores:
```

Session{
	canidate_list
}

```

maybe?

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





