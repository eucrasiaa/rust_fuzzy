// define a new algorithm by defining a struct, that does the "score" function.
// it can manage itself, store variables, etc.  




// pub fn algo_demo_slop(){
//     let new_greedy = AlgoWillBasicGreedyVer1::default();
//     let mut results: Vec<(i64, String)> = Vec::new();
//     let to_comp_with = String::from("Dyscrasia");
//     let combinations = ('a'..='z').flat_map(|a| {
//         ('a'..='z').flat_map(move |b| {
//             ('a'..='z').map(move |c| (a, b, c)) 
//         })
//     });
//     for (a, b, c) in combinations {
//             let tmp_str=format!("{a}{b}{c}");
//             let tmp_score = new_greedy.score(&to_comp_with,&tmp_str);
//             let new_item=(tmp_score, tmp_str);
//             let pos = results.binary_search_by(|(score, _)| tmp_score.cmp(score))
//                 .unwrap_or_else(|e| e);
//             results.insert(pos, new_item);
//
//             print!("\x1B[H"); 
//
//             'printer: for (i, item) in results.iter().enumerate() {
//                 println!("[{i}]: {} - {}\x1B[K", item.0, item.1);
//                 if i>20 { break 'printer};
//             }
//             results.truncate(1000);
//
//             // results.push((tmp_score,tmp_str));
//
//     }
//
// }
