use clap::{App, Arg, ArgMatches};
use std::io::Write;

pub fn parse_args<'a>(name: &str, about: &str) -> ArgMatches<'a> {
    App::new(name)
        .about(about)
        .arg(
            Arg::with_name("n")
                .long("matrix-size")
                .short("n")
                .takes_value(true)
                .number_of_values(1)
                .help("Size of square matrices (default 100)"),
        )
        .get_matches()
}

pub fn main_inner<'a, F: Fn(usize)>(opts: ArgMatches<'a>, f: F) {
    if let Ok(n) = opts
        .value_of("n")
        .map(|n| n.parse())
        .unwrap_or_else(|| Ok(100))
    {
        f(n)
    } else {
        writeln!(std::io::stderr(), "Could not parse 'n' as usize")
            .expect("Could not write to stderr");
    }
}
