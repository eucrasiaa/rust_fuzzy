/*!
# Entities Module

the `entities` module does some pre-defined implementations of structs
with the FuzzyCanidate traits. custom ones can be done via the trait
pre-defined:
 - **`DesktopEntity`**: parses/handles Linux `.desktop` files for application launching (theoredically).
    - parse_desktop_inis()
 - **`AnimalEnt`**: a simple entity model used for demonstration and scale testing, but left in for fun (yay!)
 - **`GenericStringStruct`**: A versatile standard struct for basic line strings, with a helper to pass it any file and reading by lines

 - **`CandidateGenerator`**: nice way to convert a Vec<String> into a basic GenericStringStruct type.
 - **`animal_demo`**: for animals 
 - **`new_generic_from_file`**: lets you pass a file. see the main demo
 - **`parse_desktop_inis`**: reads ini files. it kinda works.

*/
// use std::path::Path;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

pub mod prelude {
    use crate::fuzzy;
    pub use crate::fuzzy::canidate::{FuzzyCandidate, ScoreTarget};
    pub use crate::fuzzy::algorithms::*;
    pub use crate::SearchSession;
    pub use std::fmt;
    pub use std::path::Path;
    pub use std::io::{BufRead, BufReader};
    pub use std::{fs::File, io::ErrorKind};

}
pub mod animal;   
pub mod desktop;  
pub mod generic;  

pub use animal::AnimalEnt;
pub use animal::animal_demo;

pub use desktop::DesktopEntity;
pub use desktop::parse_desktop_inis;

pub use generic::GenericStringStruct;
pub use generic::new_generic_from_file;
pub use generic::CandidateGenerator;
