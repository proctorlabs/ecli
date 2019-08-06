use crate::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{collections::BTreeMap, fmt, path::PathBuf};
pub use termion::color::{self, Color as TermColor};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StyleConfig {
    pub default: Style,
    pub selected: Style,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppConfig {
    pub styles: StyleConfig,
    pub menus: BTreeMap<String, Menu>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Menu {
    pub title: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub text: String,
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Action {
    Script {
        script: String,
        #[serde(default = "default_shell")]
        shell: String,
    },
    Command {
        command: String,
        #[serde(default)]
        args: Vec<String>,
    },
    Goto {
        goto: String,
    },
    Return {
        r#return: (),
    },
    None {
        none: (),
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    None,
    Red,
    Green,
    Blue,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl AppConfig {
    pub fn load_file(file: PathBuf) -> Result<Self> {
        let f = std::fs::File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

fn default_shell() -> String {
    "bash".into()
}
