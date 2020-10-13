use clap::{App, Arg, SubCommand};
use mpvipc::{ipc::run_mpv_command, Mpv};

fn main() {
    let matches = App::new("rsmpv")
        .version("0.0.1")
        .about("\nA controller for mpv written in Rust, requires ipc to be enabled in mpv.")
        .subcommand(SubCommand::with_name("clear")
            .about("Clear the current playlist in mpv"))
        .subcommand(SubCommand::with_name("set")
            .about("Set different options for mpv")
            .arg(Arg::with_name("loop")
                .short("l")
                .long("loop")
                .help("Enable or disable playback looping")
                .value_name("y/n")
                .takes_value(true))
            .arg(Arg::with_name("position")
                .short("p")
                .long("position")
                .help("Set the position of media playback progress")
                .value_name("PERCENTAGE")
                .takes_value(true))
            .arg(Arg::with_name("volume")
                .short("v")
                .long("volume")
                .help("Set the volume")
                .value_name("PERCENTAGE")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("toggle")
            .about("Toggle various settings in mpv")
            .arg(Arg::with_name("state")
                .short("s")
                .long("state")
                .help("Pause/play mpv"))
            .arg(Arg::with_name("subtitles")
                .short("S")
                .long("subtitles")
                .help("Enable/disable visibility of subtitles")))
        .get_matches();
    let mpv = Mpv::connect("/tmp/mpvsocket").unwrap();
    if matches.is_present("clear") {
        run_mpv_command(&mpv, "clear-playlist", &[""])
            .expect("Could not play mpv, are you sure that the ipc-server is set to /tmp/mpvsocket?");
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if matches.is_present("loop") {
            let mut arg = matches.value_of("loop").unwrap().replace("n", "no");
            arg = arg.replace("y", "inf");
            run_mpv_command(&mpv, "set_property", &["loop-file", &arg])
                .expect("Could not set the loopback property.");
        } else if matches.is_present("position") {
            run_mpv_command(&mpv, "set_property", &["percent-pos", matches.value_of("position").unwrap()])
                .expect("Could not set the position, are you sure you used a valid percentage?");
        } else if matches.is_present("volume") {
            run_mpv_command(&mpv, "set_property", &["volume", matches.value_of("volume").unwrap()])
                .expect("Could not set the volume, are you sure you used a valid percentage?");
        }
    } else if let Some(matches) = matches.subcommand_matches("toggle") {
        if matches.is_present("state") {
            run_mpv_command(&mpv, "cycle", &["pause"])
                .expect("Could not pause/play mpv, are you sure that the ipc-server is set to /tmp/mpvsocket?");
        } else if matches.is_present("subtitles") {
            run_mpv_command(&mpv, "cycle", &["sub-visibility"])
                .expect("Could not enable/disable subtitle visibility, are you sure that the ipc-server is set to /tmp/mpvsocket?");
        }
    }
}
