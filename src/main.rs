use clap::{App, Arg, SubCommand};
use mpvipc::{ipc::run_mpv_command, Mpv};

fn main() {
    let matches = App::new("rsmpv")
        .version("0.0.1")
        .about("\nA controller for mpv written in Rust, requires ipc to be enabled in mpv.")
        .subcommand(SubCommand::with_name("play")
            .about("Play the current media in mpv"))
        .subcommand(SubCommand::with_name("pause")
            .about("Pause the current media in mpv"))
        .subcommand(SubCommand::with_name("set")
            .about("Set different options for mpv.")
            .arg(Arg::with_name("position")
                .long("position")
                .help("Set the percentage of the media playback progress.")
                .value_name("PERCENTAGE")
                .takes_value(true))
            .arg(Arg::with_name("volume")
                .long("volume")
                .help("Set the volume.")
                .value_name("PERCENTAGE")
                .takes_value(true)))
        .get_matches();
    let mut mpv = Mpv::connect("/tmp/mpvsocket").unwrap();
    if matches.is_present("play") {
        run_mpv_command(&mpv, "set_property", &["pause", "no"]);
    } else if matches.is_present("pause") {
        run_mpv_command(&mpv, "set_property", &["pause", "yes"]);
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if matches.is_present("position") {
            run_mpv_command(&mpv, "set_property", &["percent-pos", matches.value_of("position").unwrap()]);
        } else if matches.is_present("volume") {
            run_mpv_command(&mpv, "set_property", &["volume", matches.value_of("volume").unwrap()]);
        }
    }
}