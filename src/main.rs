#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

mod error;
#[macro_use]
mod macros;

pub use error::{AppError, Result};

mod args;
mod commands;
mod config;
mod screen;
mod templates;

pub use args::Command;

fn main() -> Result<()> {
    match args::get_args() {
        Command::Open { file } => {
            let menu = config::AppConfig::load_file(file)?;
            screen::enter(menu)?;
        }
        Command::Generate { file, add_shebang } => {
            commands::generate(file, add_shebang)?;
        }
    }
    Ok(())
}
