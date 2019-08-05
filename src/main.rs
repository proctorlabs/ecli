mod config;
mod screen;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected target file!")
    } else {
        let path = std::path::PathBuf::from(args[1].to_owned());
        let menu = config::Menu::load_file(path);
        screen::enter(menu).unwrap()
    }
}
