use rayon::prelude::*;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

// impl GrepArgs {
//     fn new(pattern: String, path: String) -> GrepArgs {
//         GrepArgs { path, pattern }
//     }
// }

fn grep(content: &str, pattern: &str, path: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}: {}", line, path);
        }
    }
}

fn run(state: GrepArgs) {
    state
        .path
        .par_iter()
        .for_each(|file| match read_to_string(file) {
            Ok(content) => grep(&content, &state.pattern, file),
            Err(reason) => println!("{}", reason),
        });
}

fn main() {
    // let pattern = std::env::args().nth(1);
    // let path = std::env::args().nth(2);

    // match (pattern, path) {
    //     (Some(pattern), Some(path)) => run(GrepArgs::new(pattern, path)),
    //     _ => println!("pattern or path is not specified!"),
    // }
    run(GrepArgs::from_args());
}
