extern crate regex;
use std;
use arguments;

pub fn list_dir(dir_name: &std::path::Path, regex: &regex::Regex, args: &arguments::Arguments) {
    if std::fs::read_dir(dir_name).is_ok() {
        for entry in std::fs::read_dir(dir_name).unwrap() {
            if !entry.is_err() {
                let entry = entry.unwrap();

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
                    if regex.is_match(&format!("{}", entry.path().display())) {
                        let mut print_string;
                        if args.color {
                            print_string = String::from(regex.replace_all(
                                &format!("{}", entry.path().display()),
                                "\x1B[31m$match\x1B[0m",
                            ));
                        } else {
                            print_string = format!("{}", entry.path().display());
                        }
                        println!("{}", print_string);
                    }
                    if entry.path().is_dir() {
                        list_dir(&entry.path(), &regex, &args);
                    }
                }
            }
        }
    }
}
