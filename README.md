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
    help     Prints this message or the help of the given subcommand(s)
    pause    Pause the current media in mpv
    play     Play the current media in mpv
    set      Set different options for mpv
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

## Roadmap
Nothing really, just adding features as I want. If anyone wants anything,<br>
let me know and I'll probably be more than happy to implement whatever.
