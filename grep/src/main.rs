use std::fs::read_to_string;

struct GrapeArgs {
    path: String,
    pattern: String,
}

impl GrapeArgs {
    fn new(pattern: String, path: String) -> GrapeArgs {
        GrapeArgs { path, pattern }
    }
}

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(state: GrapeArgs) {
    match read_to_string(state.path) {
        Ok(content) => grep(content, state.pattern),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    let pattern = std::env::args().nth(1);
    let path = std::env::args().nth(2);

    match (pattern, path) {
        (Some(pattern), Some(path)) => run(GrapeArgs::new(pattern, path)),
        _ => println!("pattern or path is not specified!"),
    }
}
