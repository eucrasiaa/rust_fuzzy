TODO: 

break apart some heavy functs (notibly within session)


OH MY GOODD I FIXED SCROLLING FINALLY THANK GOODDDDDDDDDD 
coworker losingf their damn mind :sob:
better document functions and structures


maybe some loop unrolling in calculatescore?    

Did: 

Weight were adjusted, helped fix culling problems 
ascii properly handled i think?
lotss of runtime optimizations:

Optimized runtime in a few spots!
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
