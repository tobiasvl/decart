use std::ffi::OsStr;
use std::path::Path;

use atty::Stream;
use clap::{crate_version, App, Arg};
use colored_json::prelude::*;
use decart::*;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::{SyntaxDefinition, SyntaxSetBuilder};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn main() {
    let matches = App::new("decart")
        .version(crate_version!())
        .author("Tobias V. Langhoff <tobias@langhoff.no>")
        .about("Octocart encoder/decoder")
        .subcommand(
            App::new("decode")
            .about("Decode an Octocart. If no options are supplied, the full JSON payload will be printed to stdout.")
            .arg(Arg::with_name("print program")
                .short("p")
                .long("print-program")
                .help("Instead of printing the entire JSON payload to stdout, print just the Octo program source code.")
            )
            .arg(Arg::with_name("to files")
                .short("t")
                .long("to-files")
                .takes_value(true)
                .value_name("output-file")
                .help("Instead of printing to stdout, create two files with the contents from the Octocart: An .8o file with the program source code, and an .octo.rc file with the runtime options. If you supply a value here, it will be used as the filenames (plus extensions); if not, the filename of the Octocart will be used.")
                .min_values(0)
                .max_values(1)
            )
            .arg(
                Arg::with_name("OCTOCART")
                .help("Octo cartridge file (GIF)")
                .required(true)
                .value_name("OCTOCART")
            )
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("decode") {
        let filename = Path::new(matches.value_of("OCTOCART").unwrap());
        let cart: OctoCart = from_file(filename).unwrap();

        if matches.is_present("to files") {
            let path = filename.parent().unwrap();
            let stem = filename.file_stem().unwrap();
            let base =
                matches
                    .value_of("output-file")
                    .unwrap_or(&format!("{}{:?}", path.display(), stem));
        } else if matches.is_present("print program") {
            if atty::is(Stream::Stdout) {
                let mut ssb = SyntaxSetBuilder::new();
                ssb.add(
                    SyntaxDefinition::load_from_str(
                        include_str!("octo-sublime/Octo.sublime-syntax"),
                        false,
                        None,
                    )
                    .unwrap(),
                );
                let ps = ssb.build();

                let syntax = ps.find_syntax_by_extension("8o").unwrap();

                let mut theme_cursor = std::io::Cursor::new(include_bytes!("Monokai.tmTheme"));
                let theme = ThemeSet::load_from_reader(&mut theme_cursor).unwrap();

                let mut h = HighlightLines::new(syntax, &theme);
                let mut s = Vec::new();
                for line in LinesWithEndings::from(&cart.program) {
                    let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                    s.push(escaped);
                }
                println!("{}", s.join(""));
            } else {
                println!("{}", &cart.program);
            }
        } else {
            println!("{}", cart.to_string().to_colored_json_auto().unwrap());
        }
    }
}
