
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

maybe store freq count of launched times and add to weight?


weight ent:
pub struct ScoredApp<'a> {
    pub app: &'a DesktopEntry,
    pub score: f64, 
}


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
