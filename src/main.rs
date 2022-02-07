#![allow(unused_must_use)]

use std::io;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::from_args();

    let bin = String::from("bin");
    let obj = String::from("obj");

    match args.command.as_str() {
        "help" => print_commands(),

        "delete" => {
            delete_directories(&bin, &PathBuf::from("./"));
            delete_directories(&obj, &PathBuf::from("./"));
        },

        "read" => {
            read_directories(&bin);
            read_directories(&obj);
        },

        _ => println!("{} is an unknown command!", args.command),
    };

    // Prints available commands.
    fn print_commands() {
        println!("HELP: Returns a list of commands.");
        println!("DELETE: Deletes bin and obj folders.");
        println!("READ: Lists all detected bin and obj folders.");
    }
}

// Deletes bin and obj directories.
fn delete_directories(name: &String, dir: &Path) -> io::Result<()> {      
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path == PathBuf::from(&name) {
            fs::remove_dir_all(path);
        } else {
            delete_directories(&name, &path);
        }
    }

    Ok(())
}

// Reads all detected bin and obj directories.
fn read_directories(name: &String) {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let dir = format!("./{}", &name);
        let cur_dir = format!("{}", &path.unwrap().path().display());

        if &cur_dir == &dir {
            println!("{}", &cur_dir);
        }
    }
}