#![allow(dead_code)]

extern crate clap;

use std::io::{self, Write};

use clap::{App, Arg};

#[derive(Debug)]
struct Config<'a> {
    pathvars: Vec<&'a str>,
    prefix_rules: Vec<&'a str>,
}

fn main() {
    let matches = App::new("Pathcomp")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Matti Hänninen <matti@mjhanninen.com>")
        .about(
            "Concatenates, reorders, and removes duplication from PATH-like environment variables",
        )
        .arg(
            Arg::with_name("PREFIX_RULE")
                .short("p")
                .long("prefix")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true)
                .help("Bring elements matching this prefix rule forth"),
        )
        .arg(Arg::with_name("PATHVARS").multiple(true))
        .get_matches();
    let pathvars = matches
        .values_of("PATHVARS")
        .map_or(vec![], |v| v.into_iter().collect::<Vec<_>>());
    let prefix_rules = matches
        .values_of("PREFIX_RULE")
        .map_or(vec![], |v| v.into_iter().collect::<Vec<_>>());
    let config = Config {
        pathvars,
        prefix_rules,
    };
    run(config);
}

fn run(config: Config) {
    let paths = config
        .pathvars
        .into_iter()
        .flat_map(|pathvar| pathvar.split(":"))
        .fold(Vec::new(), |mut compressed, path| {
            if compressed.iter().all(|p| p != &path) {
                compressed.push(path);
            }
            compressed
        });
    io::stdout().write(paths.join(":").as_bytes()).unwrap();
}
