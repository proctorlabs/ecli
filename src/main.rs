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
pub use templar::Document;
pub use templates::{EcliExpression, EcliTemplate};

fn main() -> Result<()> {
    match args::parse() {
        Command::Open { file } => {
            let menu = config::AppConfig::load_file(file)?;
            templates::context_set_value(&menu.vars)?;
            screen::enter(menu)?;
        }
        Command::Generate { file, add_shebang } => {
            commands::generate(file, add_shebang)?;
        }
    }
    Ok(())
}
