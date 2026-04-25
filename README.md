refactoring

trying to implement a fuzzyfinder for my launcher app
right now its just a way to index-based its subsequence index checking where i score matched indexes to specific cases (start of string, conseq matches, start after a divider like ' ' - _, etc), then, and fail early constantly thruout, and update the threshold for culling. it stores as a stack too for instant backspaces
objectively theres some loop rolling and pipeline blocking ifs  i could optimize out with bitwise. but i feel the real slowdowns come from the structural bloat i really should break it into files and not just all one big thing ... so unorganized

tui via ratatui, custom algorithm for searching. it works!! but wow. gah. its rough...

still very proud of it
