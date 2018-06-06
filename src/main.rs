extern crate clap;

use std::io::{self, Write};

use clap::{App, Arg};

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
    let mut output = Vec::new();
    for pathvar in pathvars {
        for path in pathvar.split(":") {
            if output.iter().all(|p| p != &path) {
                output.push(path.to_owned());
            }
        }
    }
    io::stdout().write(output.join(":").as_bytes()).unwrap();
}
