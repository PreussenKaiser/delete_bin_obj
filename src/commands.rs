/// Directory commands.
pub mod commands {
    use std::{fs, path::Path};
    use walkdir::WalkDir;

    /// Executes a predicate on the current directory.
    pub fn execute(f: fn(&Path)) {
        let current_directory = WalkDir::new("./");

        current_directory
            .into_iter()
            .filter(|d| d.is_ok())
            .for_each(|d| {
                let path = d
                    .unwrap()
                    .path();

                if (path.ends_with(name)) {
                    f(path);
                }
            });
    }

    /// Prints the found directory to the console.
    pub fn read(dir: &Path) {
        println!("{}", dir.display());
    }

    /// Deletes the given directory,
    pub fn delete(dir: &Path) {
        fs::remove_dir_all(dir).unwrap();
    }

    /// Determines if the current directory has a VS solution or not.
    pub fn is_solution() -> bool {
        return fs::read_dir(Path::new("./"))
            .map(|mut entries| entries.all(|entry| match entry {
                Ok(entry) =>
                    entry
                        .path()
                        .extension()
                        .and_then(|e| Some(e == "sln"))
                        .unwrap(),
                        
                Err(_error) => false
            })).unwrap();
    }
}