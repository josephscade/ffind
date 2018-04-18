extern crate clap;
extern crate regex;

// function used to walk throught folders
mod dir_walk;
// struct used to pass parameters and infos about ffind's behavour
mod arguments;

fn main() {
    // creation of an application (usefull for argument parsing and error handling)
    let matches = clap::App::new("ffind")
        .setting(clap::AppSettings::ColorNever)
        .version("0.1.0")
        .author("Leo Pourcelot leo.pourcelot@protonmail.com")
        .about("A tool to recursively find files and folders in disk")
        .arg(
            clap::Arg::with_name("FILENAME")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("name to find in current working directory"),
        )
        .arg(
            clap::Arg::with_name("deep search")
                .short("d")
                .long("deep")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("uncolored output")
                .long("no-color")
                .required(false),
        )
        .get_matches();
    // checking for the "NO_COLOR" environment variable:
    // if it's present, then content must be printed colorless
    let no_color_enabled: bool = match std::env::var_os("NO_COLOR") {
        None => false,
        _ => true,
    };
    // creation of the regex which will be used for searching
    let regex = regex::Regex::new(&format!(r"(?i)(?P<match>{})", matches.value_of("FILENAME").unwrap())).unwrap();


    // creation argument struct which let us carry informations about ffind's behavour
    let args = arguments::Arguments {
        hidden_directories: matches.is_present("deep search"),
        color: !(matches.is_present("uncolored output") || no_color_enabled),
        find_regex: regex,
    };
    // creation of the initial searching path
    let init_path = std::path::Path::new("./");
    dir_walk::list_dir(init_path, &args);
}
