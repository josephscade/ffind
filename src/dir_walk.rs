extern crate regex;
use std;
use arguments;

// function which walk throught folders to look for a named file or directory
pub fn list_dir(dir_name: &std::path::Path, args: &arguments::Arguments) {
    // usefull for error handling (eg: permission denied)
    if std::fs::read_dir(dir_name).is_ok() {
        // iteration for each file or folder inside the actual folder
        for entry in std::fs::read_dir(dir_name).unwrap() {
            // error checking (eg: permission denied)
            if !entry.is_err() {
                let entry = entry.unwrap();
                // check if the first letter of the file or folder is a '.'
                // TODO : disable this check if the hidden_directories is enabled
                let can_walk_trought: bool = entry
                    .path()
                    .components()
                    .last()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .get(0..1) != Some(".");
                if can_walk_trought || args.hidden_directories {
                    // check if it is a match
                    if args.find_regex.is_match(&format!("{}", entry.path().display())) {
                        // we create the string which will be printed
                        let mut print_string;
                        // we put coloured informations if user want it
                        if args.color {
                            print_string = String::from(args.find_regex.replace_all(
                                &format!("{}", entry.path().display()),
                                "\x1B[31m$match\x1B[0m",
                            ));
                        // or we just print it colourless
                        } else {
                            print_string = format!("{}", entry.path().display());
                        }
                        println!("{}", print_string);
                    }
                    // we triger a new exam in the directory if there is a folder
                    if entry.path().is_dir() {
                        list_dir(&entry.path(), &args);
                    }
                }
            }
        }
    }
}
