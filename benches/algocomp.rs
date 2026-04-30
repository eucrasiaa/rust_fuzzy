use will_fuzzy::fuzzy::algorithms::AlgoWillGreedyVer2;
use will_fuzzy::fuzzy::algorithms::AlgoGreedyOptimized;
use will_fuzzy::fuzzy::canidate::{FuzzyCandidate, ScoreTarget};
use will_fuzzy::fuzzy::session::SearchSession;
use will_fuzzy::fuzzy::matcher::FuzzyMatcher;
use will_fuzzy::app::FuzzyApp;
use will_fuzzy::entities::*;
use will_fuzzy::fuzzy::canidate::FuzzyBoxExt;
use std::io::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};


/// WITHIN THIS DEMO, THE BOX< DYN> IS USED FOR CODE CLARITY. 
/// its not super extended and validated, so be cautious using it like this.
fn bench_fuzzy_algos(c: &mut Criterion) {
    let entities = animal_demo();
    let queries = vec![
        "dolphin", "frog", "bird", "a b d c e f g", "phant", 
        "monk", "aligator", "chimpaznee", "bear black", "sea lion",
        "eeeeeeeeaaaaaaaaaiiiiiiiiiiii", "   cat   ", "tree", "greater", "lesser"
    ];

    let mut group = c.benchmark_group("Fuzzy Matching");

    // --- TEST ALGO 1 ---
    group.bench_function("Greedy_V2", |b| {
        let matcher = FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new());
        let mut session = SearchSession::create(&entities, matcher, String::new());
        
        b.iter(|| {
            for query in &queries {
                for c in query.to_ascii_lowercase().as_bytes() {
                    session.type_char(black_box(*c));
                }
                black_box(session.top_results(1, 0)); 
                session.clear();
            }
        });
    });

    // --- TEST ALGO 2 ---
    group.bench_function("Optimized", |b| {
        let matcher = FuzzyMatcher::with_algo(AlgoGreedyOptimized::new());
        let mut session = SearchSession::create(&entities, matcher, String::new());

        b.iter(|| {
            for query in &queries {
                for c in query.to_ascii_lowercase().as_bytes() {
                    session.type_char(black_box(*c));
                }
                black_box(session.top_results(1, 0));
                session.clear();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_fuzzy_algos);
criterion_main!(benches);
