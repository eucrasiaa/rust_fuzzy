
desktop file locations:

/usr/share/applications/
/usr/local/share/applications/
~/.local/share/applications/


using std command spawn to just let ownership leave scope? 

https://doc.rust-lang.org/std/process/index.html


matching and weights:
Desktop struct:
  id = filename.desktop
  name = "Executable Desktop Nice Name" - "Strawberry"
  generic_name = ""  - "Strawberry Music Player"
  desc = "description of it"
  exec = "exec commands + args" - "strawberry %U"
  tags<Vec Str> = []  - "\[AudioVideo\]\[Player\]\[Qt\]\[Audio\];"
  use-weight = f64 - more launched more often pushed up? store somewhere 

maybe store freq count of launched times and add to weight?


weight ent:
pub struct ScoredApp<'a> {
    pub app: &'a DesktopEntry,
    pub score: f64, 
}

assumedly id have some sort of stack of ScoredAppVectors to do the pop on backspace
```
[Desktop Entry]
Version=1.0
Type=Application
Name=Strawberry
GenericName=Strawberry Music Player
GenericName[fr]=Lecteur de musique Strawberry
GenericName[ru]=Музыкальный проигрыватель Strawberry
Comment=Plays music
Comment[fr]=Joue de la musique
Comment[ru]=Прослушивание музыки
Exec=strawberry %U
TryExec=strawberry
Icon=strawberry
Terminal=false
Categories=AudioVideo;Player;Qt;Audio;
Keywords=Audio;Player;Clementine;
MimeType=x-content/audio-player;application/ogg;application/x-ogg;application/x-ogm-audio;audio/flac;audio/ogg;audio/vorbis;audio/aac;audio/mp4;audio/mpeg;audio/mpegurl;audio/vnd.rn-realaudio;audio/x-flac;audio/x-oggflac;audio/x-vorbis;audio/x-vorbis+ogg;audio/x-speex;audio/x-wav;audio/x-wavpack;audio/x-ape;audio/x-mp3;audio/x-mpeg;audio/x-mpegurl;audio/x-ms-wma;audio/x-musepack;audio/x-pn-realaudio;audio/x-scpls;video/x-ms-asf;x-scheme-handler/tidal;
StartupWMClass=strawberry
Actions=Play-Pause;Stop;StopAfterCurrent;Previous;Next;
```


engine


look into traits + impl on struct?

struct holds config (algo, case_sensitive, etc)
passed a list? make a resulting score values?
return sorted?


subset caching prob

algo prob look more at like Greedy Matcher + Smith-Waterman for shorthand vs error checking in things like Levenshtein Distance or Sørensen-Dice for similar



``` rust
pub struct ScoreTarget<'a> {
    pub text: &'a str,
    pub weight_multiplier: f64, // the weight
    pub exact_match_only: bool, // for smthn like tags? idk
}
pub trait FuzzyCandidate {
    // for structs, define which strings are included in scoring?
    fn search_targets(&self) -> Vec<ScoreTarget>;
    // from use statistics, later include ig
    fn usage_bonus(&self) -> f64;
}
```
so like for desktop ent


Algorithm:

looking into the FZF v1 greedy:

step 1
fast fail:
query = "abuc"
target = "a buzy camel"
we hit all 4 in string, so keep it
else? drop it
store indexes of match

step 2
then scoring thing:
first match at index 0? massive bonus
index after a space or dash? bonus
consec indexes?

