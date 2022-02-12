use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt)]
/// Deletes the bin and obj folders in a VS solution.
struct Cli {
    /// help, delete and read.
    command: String,
}

fn main() {
    let args = Cli::from_args();

    match args.command.as_str() {
        "help" => print_commands(),

        "delete" => {
            delete_directories("bin");
            delete_directories("obj");
        },

        "read" => {
            read_directories("bin");
            read_directories("obj");
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

// Deletes all directories with the name parameter.
fn delete_directories(name: &str) {
    if is_solution().is_some() {
        for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
            try_delete_dir(&name, entry.path());
        }
    }
}

// Reads all directories with the name parameter.
fn read_directories(name: &str) {
    for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
        try_read_dir(&name, entry.path());
    }
}

// Deletes a directory.
// name: Name of the directory.
// dir: The directory's path.
fn try_delete_dir(name: &str, dir: &Path) {
    if dir.ends_with(Path::new(&name)) {
        fs::remove_dir_all(dir).unwrap();
        println!("Deleted {}", dir.display());
    }
}

// Prints a directory.
// name: Name of the directory.
// dir: The directory's path.
fn try_read_dir(name: &str, dir: &Path) {
    if is_solution().is_some() {
        if dir.ends_with(Path::new(&name)) {
            println!("Found {}", dir.display());
        }
    }
}

// Determines if the current directory has a solution file.
fn is_solution() -> Option<bool> {
    for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(OsStr::to_str) == Some("sln") {
            return Some(true)
        }
    }

    None
}