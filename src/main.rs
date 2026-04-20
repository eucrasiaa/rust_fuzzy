mod old_main;
// use old_main::*;
mod fuzzy;
// use fuzzy::{AlgoWillBasicGreedyVer1, SimilarityAlgorithm};
use fuzzy::*;

fn main(){
    // old_main::main().unwrap();
    let new_greedy = AlgoWillBasicGreedyVer1::default();
    println!("hello world");
}
