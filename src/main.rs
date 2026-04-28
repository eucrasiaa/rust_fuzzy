use will_fuzzy::fuzzy::algorithms::AlgoWillGreedyVer2;
use will_fuzzy::fuzzy::canidate::{FuzzyCandidate, ScoreTarget};
use will_fuzzy::fuzzy::session::SearchSession;
use will_fuzzy::fuzzy::matcher::FuzzyMatcher;
use will_fuzzy::app::FuzzyApp;
use will_fuzzy::entities::*;
use will_fuzzy::fuzzy::canidate::FuzzyBoxExt;
#[derive(Debug)]
enum ActionDemo {
    DesktopFile,
    InputFile,
    ANIMALS,
}



/// WITHIN THIS DEMO, THE BOX< DYN> IS USED FOR CODE CLARITY. 
/// its not super extended and validated, so be cautious using it like this.
use std::io::Result;
fn main() -> Result<()>{
    let mut args = std::env::args().skip(1);

    let mut filename = String::new();

    let mut selected_action = ActionDemo::DesktopFile;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                if let Some(val) = args.next() {
                    filename = val;
                    selected_action = ActionDemo::InputFile;
                } else {
                    eprintln!("Error: --input requires a file path");
                    std::process::exit(1);
                }
            }
            "-h" | "--help" => {
                println!("MY (wills) AWEsome amazing fuzzy finder application");
                println!("/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\");
                println!("./will_fuzzy <flags>");
                println!();
                println!("  -h, --help      shows this ");
                println!("  --animals       reads off the provided animal_names.txt");
                println!("  --input <FILE>  not well tested but should be able to just read in a text file lol");
                println!();
                std::process::exit(0);
            }
            "--animals" => {
                selected_action = ActionDemo::ANIMALS;
            }
            other => {
                eprintln!("Error!: unknown argument {:?}",other);
                std::process::exit(1);
            }
        }
    }
    // let mut entities: Vec<Box<dyn FuzzyCandidate>>;
    let matcher = FuzzyMatcher::with_algo(AlgoWillGreedyVer2::new());
    let entities: Vec<Box<dyn FuzzyCandidate>> = match selected_action {
        ActionDemo::DesktopFile => {
            parse_desktop_inis().into_iter().into_boxed()
        }
        ActionDemo::ANIMALS => {
            animal_demo().into_iter().into_boxed()
        }
        ActionDemo::InputFile => {
            new_generic_from_file(&filename).into_iter().into_boxed()
        }
    };
    let session = SearchSession::create(&entities, matcher, String::new());
    let mut fuzzy_app = FuzzyApp::new(session);
    fuzzy_app.init()?;


    Ok(())
}
