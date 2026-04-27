# Wills-Fuzzy-Searcher

## FOR LINUX!!!!!!!!!! i love linux
a shockingly fast, terminal based fuzzy-search tool! A custom written, technically standalone fuzzy-search algorithm and rust library powers a tui visual interface.

the release here is an example use case of the algorithm + session manager, which defaults to a psudo linux application launcher . well technically it does everything BUT launch but thats for version 2 where its gonna exist as a socket and store in memory

while totally usable. it mostly  was a fun practice for me to learn rust! i really wanted to write smthn to force me to proper interact with rust tools, especially iterators, lifespans, modules, etc. my notes app has become so full with rust notes. but man is the documentation just lovely.

## Core modes!!!
- no args: desktop file mode. reads from folders
- demo/animal names: --animals, reads from the demo file provided animal_names.txt
- or custom input mode --input <filename>
- this can also be seen in ./ --help/-h


## loose architecture & implementation details
i tried to preallocate, minimize odd edge cases and trying to reduce floating point math n other stuff. enjoyed watching demo runtimes and performance stuff
### Minimal allocation in hotloops!!!

i made a very barebones kidna chopped arena bump allocator(?) for the debug strings to avoid formatting constantly on draw loop, and just pre allocating and Write!() ing!! 


### arethmathic ?
i was initially doing scalings via floating point values, but swapped to a bit manip thingy i read once abt on some article or maybe a book ages back, so i used a scale of   and bit shifted  to make my calc just a >> 10 ! who knows if its acutally any faster, fighting compiler optimization was strange.
i wanna learn to read llvm and asm better!

### state management references 
instant backspacing and full sync hisory via reference slices, which was sooo annoying with lifetimes but finally realized :3



### single file to modules
the original file was a massive evil 900 line file. but i got to break it up into various modules and library setup. its technically standalone! all the parts. if you wanna. i guess.

### heavily trait driven!!
one of my favorite parts of rust has been structs, impls, and traits and deriving traits for generics. i love it so much.  so its nice to break things apart 


## upcoming changes or goals
- more nicer rust-doc style comments, its not very comprensive
- clean out junk comments that are old code blocks
- make it actually launch files? 



optimized runtime in a few spots!
1. Debug String pooling via a preallocated buffer, to prevent repetitive format!() calls
2. render loop changes to only iter thru the visible elements
3. precomputation of visual strings, rather than construction every call
4. some itoa  uses to help go faster
5. fixed a possible bug in the ignore_case thing
6. swapped the sort to sort_unstable_by_key 
7. a mock keystroke for profiling and such perhaps

maybe toggle an ascii or non acii mode with #features for optimization? idk brah

trying to implement a fuzzyfinder for my launcher app
right now its just a way to index-based its subsequence index checking where i score matched indexes to specific cases (start of string, conseq matches, start after a divider like ' ' - _, etc), then, and fail early constantly thruout, and update the threshold for culling. it stores as a stack too for instant backspaces

tui via ratatui, custom algorithm for searching. it works!! but wow. gah. its rough...

still very proud of it
