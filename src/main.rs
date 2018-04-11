extern crate clap;
extern crate regex;

mod dir_walk;
mod arguments;

fn main() {
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

    let no_color_enabled: bool = match std::env::var_os("NO_COLOR") {
        None => false,
        _ => true,
    };
    let args = arguments::Arguments {
        hidden_directories: matches.is_present("deep search"),
        color: !(matches.is_present("uncolored output") || no_color_enabled),
        find_string: String::from(matches.value_of("FILENAME").unwrap()),
    };

    let find_regex = regex::Regex::new(&format!(r"(?i)(?P<match>{})", args.find_string)).unwrap();
    let init_path = std::path::Path::new("./");
    if init_path.is_dir() {
        dir_walk::list_dir(init_path, &find_regex, &args);
    }
}
