extern crate regex;

fn main() {
    let args = parse_arguments();
    let find_regex = regex::Regex::new(String::from(args.find_string.to_lowercase())
                                       .as_str())
        .unwrap();
    let init_path = std::path::Path::new("./");

    list_dir(init_path,
             &find_regex,
             &args);
}

struct Arguments
{
    color: bool,
    recursive: bool,
    find_string: String,
}

fn parse_arguments() ->Arguments
{
    let mut use_color: bool = false;
    let mut string: String = String::from("");
    let mut count: i32 = 0;
    for argument in std::env::args()
    {
        if count == 0
        {
            count += 1;
        }
        else
        {
            if argument.get(0..1) == Some("-")
            {
                for (index, letter) in argument.chars().enumerate()
                {
                    if letter.to_string() == "c"
                    {
                        use_color = true;
                    }
                }
            }
            else if string == ""
            {
                string = argument;
            }
        }
    }
    Arguments {
        color: use_color,
        recursive: true,
        find_string: string,
    }
}

fn list_dir(dir_name: &std::path::Path, regex: &regex::Regex, args: &Arguments) -> std::io::Result<()>
{

    if dir_name.is_dir() {
        for entry in std::fs::read_dir(dir_name)? {
            let entry = entry?;
            if regex.is_match(&format!("{}", entry
                                       .path()
                                       .display())) {
                println!("{}", entry
                         .path()
                         .display());
            }
            if entry.path().is_dir() {
                list_dir(&entry.path(), &regex, &args);
            }
        }
    }
    Ok(())
}
