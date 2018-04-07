extern crate regex;
extern crate clap;

fn main() {
    let matches = clap::App::new("ffind")
        .version("0.1.0")
        .author("Leo Pourcelot leo.pourcelot@protonmail.com")
        .about("A tool to recursively find files and folders in disk")
        .arg(clap::Arg::with_name("FILENAME")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("name to find in current working directory"))
        .arg(clap::Arg::with_name("deep search")
             .short("d")
             .long("deep")
             .required(false))
        .arg(clap::Arg::with_name("uncolored output")
             .long("no-color")
             .required(false))
        .get_matches();

    let no_color_enabled: bool = match std::env::var_os("NO_COLOR") {
        None => false,
        _ => true,
    };
    let args = Arguments {
        hidden_directories: matches.is_present("deep search"),
        color: !(matches.is_present("uncolored output") || no_color_enabled),
        find_string: String::from(matches.value_of("FILENAME").unwrap()),
    };

    let find_regex = regex::Regex::new(&format!(r"(?i)(?P<match>{})", args.find_string))
        .unwrap();
    let init_path = std::path::Path::new("./");
    if init_path.is_dir() {
        list_dir(init_path,
                 &find_regex,
                 &args);
    }
}

struct Arguments {
    hidden_directories: bool,
    color: bool,
    find_string: String,
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
}
