extern crate regex;

pub struct Arguments {
    pub hidden_directories: bool,
    pub color: bool,
    pub find_regex: regex::Regex,
}
