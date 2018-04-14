extern crate regex;
use std;
use arguments;

// function which walk throught folders to look for a named file or directory
pub fn list_dir(dir_name: &std::path::Path, args: &arguments::Arguments) {
    match std::fs::read_dir(dir_name) {
        Err(_) => {}
        Ok(entries) => for wraped_entry in entries {
            let entry = wraped_entry.unwrap();
            let can_walk_trought: bool = match args.hidden_directories {
                true => true,
                false => {
                    match entry
                        .path()
                        .components()
                        .last()
                        .unwrap()
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .get(0..1)
                    {
                        Some(i) => match i {
                            "." => false,
                            _ => true,
                        },
                        None => false,
                    }
                }
            };
            if can_walk_trought && entry.metadata().unwrap().is_dir() {
                list_dir(&entry.path(), &args);
            }
            if args.find_regex
                .is_match(entry.path().as_path().to_str().unwrap())
            {
                let printed_string: String = match args.color {
                    false => entry.path().as_path().to_str().unwrap().to_string(),
                    true => String::from(args.find_regex.replace_all(
                        entry.path().as_path().to_str().unwrap(),
                        "\x1B[31m$match\x1B[0m",
                    )),
                };
                println!("{}", printed_string);
            }
        },
    }
}
