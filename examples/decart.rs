use clap::{crate_version, App, Arg};
use colored_json::prelude::*;
use decart::*;

pub fn main() {
    let matches = App::new("decart")
        .version(crate_version!())
        .author("Tobias V. Langhoff <tobias@langhoff.no>")
        .about("Octocart encoder/decoder")
        .arg(Arg::with_name("tickrate")
                .short("t")
                .long("tickrate")
                .takes_value(true)
                .value_name("TICKRATE")
                .help("Instructions to execute per 60Hz frame")
                .default_value("40")
        )
        .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("CONFIG_FILE")
                .help("Configuration file, compatible with C-Octo\nIf not supplied, we will attempt to find a file with the same name and in the same location as the current ROM, but with an '.octo.rc' file extension, for easy per-game configuration.\nIf that doesn't exist, the default is ~/.octo.rc")
                .default_value("~/.octo.rc")
        )
        .arg(Arg::with_name("quirks")
                .short("q")
                .long("quirks")
                .takes_value(true)
                .value_name("COMPATIBILITY_PROFILE")
                .help("Force quirky behavior for platform compatibility.\n(For fine-tuned quirks configuration, you can toggle individual settings in a configuration file; see --config)\nPossible values: vip, schip, octo")
                .default_value("octo")
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Starts execution in interrupted mode, for easier debugging")
        )
        .arg(
            Arg::with_name("")
                .help("CHIP-8 ROM file")
                .required(true) // for the time being
                //.index(1),
        )
        .get_matches();

    let rom = std::fs::read(matches.value_of("ROM").unwrap()).expect("Couldn't load ROM");
}
