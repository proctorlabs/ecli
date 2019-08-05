use serde::Deserialize;
use serde_yaml;
use std::fmt;
use std::path::PathBuf;
pub use termion::color::{self, Color as TermColor};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Menu {
    pub default: Style,
    pub selected: Style,
    pub menu: Vec<MenuItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum MenuItem {
    Script { text: String, cmd: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    None,
    Red,
    Green,
    Blue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Style {
    pub alignment: Alignment,
    pub fg: Color,
    pub bg: Color,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::None => color::Reset.write_fg(f),
            Color::Red => color::Red.write_fg(f),
            Color::Green => color::Green.write_fg(f),
            Color::Blue => color::Blue.write_fg(f),
        }
    }
}

impl Menu {
    pub fn load_file(file: PathBuf) -> Self {
        let f = std::fs::File::open(file).unwrap();
        serde_yaml::from_reader(f).unwrap()
    }
}
