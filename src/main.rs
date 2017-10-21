use std::env;
use std::io::{self, Write};

fn main() {
    let mut output = Vec::new();
    let mut args = env::args();
    args.next();                // toss executable
    for arg in args {
        for path in arg.split(":") {
            if output.iter().all(|p| p != &path) {
                output.push(path.to_owned());
            }
        }
    }
    io::stdout().write(output.join(":").as_bytes()).unwrap();
}
