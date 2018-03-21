extern crate regex;

fn main() {
    let find_string = std::env::args_os().nth(1);
    if find_string == None {
        println!("Error: no filename specified!");
    }
    else {
        let find_string = String::from(find_string
                                       .unwrap()
                                       .to_str()
                                       .unwrap()
                                       .to_lowercase());

        let find_regex = regex::Regex::new(find_string
                                           .as_str())
            .unwrap();
        let init_path = std::path::Path::new("./");
        list_dir(init_path, &find_regex);
    }
}

fn list_dir(dir_name: &std::path::Path, regex: &regex::Regex) -> std::io::Result<()>
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
                list_dir(&entry
                         .path(), &regex);
            }
        }
    }
    Ok(())
}
