extern crate regex;
use std;
use arguments;

pub struct DiskElement {
    root: std::path::PathBuf,
}

impl DiskElement {
    pub fn new(dir_name: std::fs::DirEntry) -> DiskElement {
        DiskElement {
            root: dir_name.path(),
        }
    }
    pub fn is_dir(&self) -> bool {
        self.root.is_dir()
    }
    pub fn is_hidden(&self) -> bool {
        format!(
            "{}",
            std::path::Path::new(self.root.file_name().unwrap()).display()
        ).get(0..1) == Some(".")
    }
    pub fn get_path(&self) -> std::string::String {
        format!("{}", self.root.display())
    }
}

// function which walk throught folders to look for a named file or directory
pub fn list_dir(dir_name: &std::path::Path, args: &arguments::Arguments) {
    // check for the rights in the directory
    if let Ok(entries) = std::fs::read_dir(dir_name) {
        for wraped_entry in entries {
            // we check for errors (eg: permission errors)
            if let Ok(entry) = wraped_entry {
                let entry = DiskElement::new(entry);
                // if the entry is a folder, then we try to schedule a search
                // therefore we have to know if the directory is "hidden"
                // but we first check if we have to search in hidden
                // directories or not
                // if we haven't, then we look for the directory's first letter
                // if this is a ".", then we don't dive in it, otherwise, we do
                if entry.is_dir() {
                    if !entry.is_hidden() || args.hidden_directories {
                        list_dir(entry.root.as_path(), &args);
                    }
                }
                // we look if the entry matches the regex provided by the user
                if (!entry.is_hidden() || args.hidden_directories)
                    && args.find_regex.is_match(entry.get_path().as_str())
                {
                    // if it is, we create a string which contains either the
                    // coloured match or the uncoloured one
                    let printed_string: String = match args.color {
                        false => entry.get_path(),
                        true => String::from(
                            args.find_regex
                                .replace_all(entry.get_path().as_str(), "\x1B[31m$match\x1B[0m"),
                        ),
                    };
                    // then we print it
                    println!("{}", printed_string);
                }
            }
        }
    }
}
