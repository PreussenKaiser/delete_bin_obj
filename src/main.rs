use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt)]
/// Deletes the bin and obj folders in a VS solution.
struct Cli {
    /// help, delete and read
    command: String,
}

/// Processes CLI arguments.
fn main() {
    let args = Cli::from_args();
    const BIN: &str = "bin";
    const OBJ: &str = "obj";

    match args.command.as_str() {
        "help" => print_commands(),

        "delete" => {
            act_on_directories(BIN, delete_dir);
            act_on_directories(OBJ, delete_dir);
        },

        "read" => {
            act_on_directories(BIN, read_dir);
            act_on_directories(OBJ, read_dir);
        },

        _ => println!("{} is an unknown command!", args.command),
    };
}

/// Prints available commands.
fn print_commands() {
    println!("HELP: Returns a list of commands.");
    println!("DELETE: Deletes bin and obj folders.");
    println!("READ: Lists all detected bin and obj folders.");
}

/// Acts on directories with a given action.
fn act_on_directories(name: &str, action: fn(&str, &Path)) {
    if is_solution().is_some() {
        for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
            action(name, entry.path());
        }
    }
    else {
        println!("Not in a VS Solution!");
    }
}

/// Deletes a directory.
fn delete_dir(name: &str, dir: &Path) {
    if dir.ends_with(Path::new(name)) {
        fs::remove_dir_all(dir).unwrap();
        println!("Deleted {}", dir.display());
    }
}

/// Prints a directory to the console.
fn read_dir(name: &str, dir: &Path) {
    if dir.ends_with(Path::new(name)) {
        println!("Found {}", dir.display());
    }
}

/// Determines if the current directory has a solution file.
fn is_solution() -> Option<bool> {
    for entry in fs::read_dir(Path::new("./")).ok()? {
        if entry.ok()?.path().extension().and_then(OsStr::to_str) == Some("sln") {
            return Some(true)
        }
    }

    None
}