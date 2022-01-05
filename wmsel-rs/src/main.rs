use std::path::Path;
use std::{fs, env, io};

fn init() {

}

fn determine_config_path() -> Path {
    for entry in fs::read_dir("~/.config/")? {
        let entry = entry.expect("Error");
        let path = entry.path();
        if path.is_dir() && (entry.to_str()? == "wmsel") {
            return entry.to_str()?;
        }
    }
    fs::create_dir("~/.config/wmsel").unwrap();
    *Path::new("~/.config/wmsel")
}

fn front_end(){}





fn main() {
    println!("Hello, world!");
    let config_path = Path::new("~/.config");

}
