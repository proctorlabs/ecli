#[macro_use]
extern crate clap;

mod error;
#[macro_use]
mod macros;

pub use error::{AppError, Result};

mod args;
mod commands;
mod config;
mod screen;

pub use args::Command;

fn main() -> Result<()> {
    match args::get_args() {
        Command::Open { file } => {
            let menu = config::AppConfig::load_file(file)?;
            screen::enter(menu)?;
        }
        Command::Generate { file } => {
            commands::generate(file)?;
        }
    }
    Ok(())
}
