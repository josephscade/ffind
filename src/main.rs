extern crate regex;

fn main() {
    let args = parse_arguments();
    if args.find_string != "" {
        let find_regex = regex::Regex::new(&format!(r"(?i)(?P<match>{})", args.find_string))
            .unwrap();
        let init_path = std::path::Path::new("./");
        if init_path.is_dir() {
            list_dir(init_path,
                     &find_regex,
                     &args);
        }
        else {
            println!("The specified file is a path. Sorry");
        }
    } else {
        println!("No pattern specified! Please specify a pattern.");
    }
}

struct Arguments {
    hidden_directories: bool,
    color: bool,
    find_string: String,
    debug: bool,
}

fn parse_arguments() -> Arguments {
    let mut use_color: bool = {
        match std::env::var_os("NO_COLOR") {
            None => true,
            _ => false,
        }
    };

    let mut string: String = String::from("");
    let mut count: i32 = 0;
    let mut all_directories: bool = false;
    let mut debug_output: bool = false;
    for argument in std::env::args() {
        if count == 0 {
            count += 1;
        } else {
            if argument.get(0..1) == Some("-") {
                if argument.get(1..2) == Some("-") {
                    match argument.as_ref() {
                        "--no-color" => use_color = false,
                        "--debug" => debug_output = true,
                        "--deep" => all_directories = true,
                        _ => println!("Argument \"{}\" not recognized", argument),
                    }
                } else {
                    for letter in argument.chars() {
                        match letter {
                            'a' => all_directories = true,
                            'd' => debug_output = true,
                            _ => println!("The argument {} is not recognized. Processing without it", letter),
                        }
                    }
                }
            } else if string == "" {
                string = argument;
            }
        }
    }
    Arguments {
        color: use_color,
        find_string: string,
        hidden_directories: all_directories,
        debug: debug_output,
    }
}

fn list_dir(
    dir_name: &std::path::Path,
    regex: &regex::Regex,
    args: &Arguments,
) {
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
                        list_dir(
                            &entry.path(),
                            &regex,
                            &args);
                    }
                }
            }
        }
    }
    else if args.debug {
        println!("Skipping \"{}\": no permission", dir_name.display());
    }
}
