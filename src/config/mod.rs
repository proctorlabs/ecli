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

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            default: Style {
                fg: Color::Red,
                bg: Color::None,
            },
            selected: Style {
                fg: Color::Green,
                bg: Color::None,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppConfig {
    #[serde(default)]
    pub options: Options,
    pub styles: StyleConfig,
    pub menus: BTreeMap<String, Menu>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct Options {
    #[serde(default)]
    pub debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut menus = BTreeMap::new();
        menus.insert(
            "main".into(),
            Menu::Choice(ChoiceMenu {
                title: "Default Menu".into(),
                entries: vec![Entry {
                    text: "Exit".into(),
                    actions: vec![Action::Pop { pop: () }],
                }],
            }),
        );
        AppConfig {
            options: Default::default(),
            styles: Default::default(),
            menus,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Menu {
    Choice(ChoiceMenu),
    Prompt(PromptMenu),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ChoiceMenu {
    pub title: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct PromptMenu {
    pub prompt: String,
    pub then: Vec<Action>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub text: String,
    pub actions: Vec<Action>,
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
    Set {
        set: String,
    },
    Pop {
        pop: (),
    },
    Validate {
        validate: String,
        #[serde(default = "default_shell")]
        shell: String,
        #[serde(default)]
        on_fail: Vec<Action>,
    },
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
