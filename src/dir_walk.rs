extern crate regex;
use std;
use arguments;

// function which walk throught folders to look for a named file or directory
pub fn list_dir(dir_name: &std::path::Path, args: &arguments::Arguments) {
    // check for the rights in the directory
    if let Ok(entries) = std::fs::read_dir(dir_name) {
        for wraped_entry in entries {
            // we check for errors (eg: permission errors)
            if let Ok(entry) = wraped_entry {
                // if the entry is a folder, then we try to schedule a search
                // therefore we have to know if the directory is "hidden"
                // but we first check if we have to search in hidden
                // directories or not
                // if we haven't, then we look for the directory's first letter
                // if this is a ".", then we don't dive in it, otherwise, we do
                if entry.metadata().unwrap().is_dir() {
                    if match args.hidden_directories {
                        true => true,
                        false => match entry
                            .path()
                            .components()
                            .last()
                            .unwrap()
                            .as_os_str()
                            .to_str()
                            .unwrap()
                            .get(0..1)
                        {
                            // although this is a bit tricky, it works: if we
                            // can't unwrap the value, then it is a non-ASCII
                            // value, so it is not a ".".
                            // consequently we are able to dive in it
                            Some(i) => match i {
                                "." => false,
                                _ => true,
                            },
                            None => false,
                        },
                    } {
                        list_dir(&entry.path(), &args);
                    }
                }
                // we look if the entry matches the regex provided by the user
                if args.find_regex
                    .is_match(entry.path().as_path().to_str().unwrap())
                {
                    // if it is, we create a string which contains either the
                    // coloured match or the uncoloured one
                    let printed_string: String = match args.color {
                        false => entry.path().as_path().to_str().unwrap().to_string(),
                        true => String::from(args.find_regex.replace_all(
                            entry.path().as_path().to_str().unwrap(),
                            "\x1B[31m$match\x1B[0m",
                        )),
                    };
                    // then we print it
                    println!("{}", printed_string);
                }
            }
        }
    }
}
