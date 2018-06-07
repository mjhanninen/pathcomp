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

#[derive(Clone, Debug, PartialOrd, PartialEq)]
enum Match {
    Exact,
    Partial(usize),
}

fn match_prefix(prefix: &str, path: &str) -> Option<Match> {
    if prefix == path {
        Some(Match::Exact)
    } else if path.starts_with(prefix) {
        Some(Match::Partial(path.len() - prefix.len()))
    } else {
        None
    }
}

fn match_rule(rules: &[&str], path: &str) -> usize {
    rules
        .into_iter()
        .enumerate()
        .fold(None, |old: Option<(usize, Match)>, (i, r)| {
            match (old, match_prefix(r, path).map(|m| (i, m))) {
                (Some(old), Some(new)) => {
                    if new.1 < old.1 {
                        Some(new)
                    } else {
                        Some(old)
                    }
                }
                (None, new) => new,
                (old, None) => old,
            }
        })
        .map(|(i, _)| i)
        .unwrap_or(rules.len())
}

fn run(config: Config) {
    let mut paths: Vec<(usize, &str)> = config
        .pathvars
        .iter()
        .flat_map(|pathvar| pathvar.split(":"))
        .fold(Vec::new(), |mut compressed, path| {
            if compressed.iter().all(|p| p != &path) {
                compressed.push(path);
            }
            compressed
        })
        .into_iter()
        .map(|path| (match_rule(&config.prefix_rules, path), path))
        .collect();
    paths.sort_by_key(|x| x.0);
    let paths: Vec<&str> = paths.into_iter().map(|(_, path)| path).collect();
    io::stdout().write(paths.join(":").as_bytes()).unwrap();
}

#[cfg(test)]
mod test {

    #[test]
    fn test_match_ordering() {
        use super::Match;
        assert!(Match::Exact < Match::Partial);
        assert!(Match::Exact < Match::NoMatch);
        assert!(Match::Partial < Match::NoMatch);
        assert!(Match::Exact == Match::Exact);
        assert!(Match::Partial == Match::Partial);
        assert!(Match::NoMatch == Match::NoMatch);
    }
}
