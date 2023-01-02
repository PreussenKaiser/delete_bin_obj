use std::env;

use crate::commands::commands::{self, for_each_child, execute};

mod commands;

/// Processes CLI arguments.
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args[1]
        .to_lowercase()
        .trim();

    const DIRS: [&str; 2] = ["bin", "obj"];

    match arg {
        "read" => DIRS
            .into_iter()
            .for_each(|dir| execute(read(dir))),

        "delete" => DIRS
            .into_iter()
            .for_each(|d| execute(delete(dir))),

        _ => println!("{} is an unknown command.", arg)
    }
}