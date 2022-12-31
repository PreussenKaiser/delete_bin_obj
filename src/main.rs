use std::env;

mod commands;

/// Processes CLI arguments.
fn main() {
    let args: Vec<String> = env::args().collect();

    const DIRS: [&str; 2] = ["bin", "obj"];

    match args[1]
        .to_lowercase()
        .trim() {

        "read" => { },
        "delete" => { },
        _ => { }
    }
}