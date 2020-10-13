## Configuration
Requires either the flag `--input-ipc-server=/tmp/mpvsocket` to be passed at mpv runtime,<br>
or for this line `input-ipc-server=/tmp/mpvsocket` to be present in your `$HOME/.config/mpv/mpv.conf`.

## Help
`$ rsmpv -h`
```
rsmpv 0.0.1

A controller for mpv written in Rust, requires ipc to be enabled in mpv.

USAGE:
    rsmpv [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clear     Clear the current playlist in mpv
    help      Prints this message or the help of the given subcommand(s)
    set       Set different options for mpv
    toggle    Toggle various settings in mpv
```

`$ rsmpv set -h`
```
rsmpv-set 
Set different options for mpv

USAGE:
    rsmpv set [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --loop <y/n>               Enable or disable playback looping
    -p, --position <PERCENTAGE>    Set the position of media playback progress
    -v, --volume <PERCENTAGE>      Set the volume
```

`$ rsmpv toggle -h`
```
rsmpv-toggle
Toggle various settings in mpv

USAGE:
    rsmpv toggle [FLAGS]

FLAGS:
    -h, --help         Prints help information
    -s, --state        Pause/play mpv
    -S, --subtitles    Enable/disable visibility of subtitles
    -V, --version      Prints version information
```

## Roadmap
Nothing really, just adding features as I want. If anyone wants anything,<br>
let me know and I'll probably be more than happy to implement whatever.
