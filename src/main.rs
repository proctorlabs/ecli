mod error;

pub use error::{AppError, Result};

mod config;
mod screen;

fn main() -> Result<()> {
    use std::env::args;
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Expected target file!")
    } else {
        let path = std::path::PathBuf::from(args.last().unwrap_or(&String::default()).to_owned());
        let menu = config::AppConfig::load_file(path)?;
        screen::enter(menu)?;
    }
    Ok(())
}
