#![allow(unused_must_use)]

use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::from_args();

    match args.command.as_str() {
        "help" => print_commands(),

        "delete" => {
            delete_directories(String::from("bin"));
            delete_directories(String::from("obj"));
        },

        "read" => {
            read_directories(String::from("bin"));
            read_directories(String::from("obj"));
        },

        _ => println!("{} is an unknown command!", args.command),
    };

    // Prints available commands.
    fn print_commands() {
        println!("HELP: Returns a list of commands.");
        println!("DELETE: Deletes bin and obj folders.");
        println!("READ: Lists all detected bin and obj folders.");
    }

    // Deletes bin and obj directories.
    fn delete_directories(name: String) {
        println!("Deleted {}", name);
        fs::remove_dir_all(name);
    }

    // Reades all detected bin and obj directories.
    fn read_directories(name: String) {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            let dir = format!("./{}", name);
            let cur_dir = format!("{}", path.unwrap().path().display());

            if cur_dir == dir {
                println!("{}", cur_dir);
            }
        }
    }
}