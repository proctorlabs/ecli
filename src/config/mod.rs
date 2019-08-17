use crate::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_yaml::Value;
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
    #[serde(default)]
    pub styles: StyleConfig,
    pub actions: BTreeMap<String, OneOrMany<Action>>,
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
                    actions: OneOrMany::One("exit".into()),
                }],
            }),
        );
        AppConfig {
            options: Default::default(),
            styles: Default::default(),
            actions: Default::default(),
            menus,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Menu {
    Choice(ChoiceMenu),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T: Clone> OneOrMany<T> {
    pub fn get(&self) -> Vec<T> {
        match self {
            OneOrMany::One(t) => vec![t.clone()],
            OneOrMany::Many(t) => t.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ChoiceMenu {
    pub title: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub text: String,
    pub actions: OneOrMany<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Action {
    Nav(Nav), // These are navigation types without options, we box them here so they can be used without a hashmap
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
    Check {
        check: String,
        pass: OneOrMany<String>,
        fail: OneOrMany<String>,
    },
    Prompt {
        prompt: String,
        val: String,
        #[serde(default)]
        password: bool,
    },
    Print {
        print: String,
    },
    Goto {
        goto: String,
    },
    Set {
        set: Value,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Nav {
    Pop,
    Exit,
    Pause,
}

macro_rules! impl_colors {
    ($($name:ident : $tname:ident),*) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(rename_all = "snake_case")]
        pub enum Color {
            $($name),*
        }

        impl fmt::Display for Color {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Color::$name => color::$tname.write_fg(f)),*
                }
            }
        }
    };
}

impl_colors! {
    None:Reset,
    Black:Black,
    Red:Red,
    Green:Green,
    Yellow:Yellow,
    Blue:Blue,
    Magenta:Magenta,
    Cyan:Cyan,
    White:White,
    LightBlack:LightBlack,
    LightRed:LightRed,
    LightGreen:LightGreen,
    LightYellow:LightYellow,
    LightBlue:LightBlue,
    LightMagenta:LightMagenta,
    LightCyan:LightCyan,
    LightWhite:LightWhite
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
}

impl AppConfig {
    pub fn load_file(file: PathBuf) -> Result<Self> {
        let f = std::fs::File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }

    pub fn get_actions(&self, actions: &[String]) -> Result<Vec<Action>> {
        let mut res = vec![];
        for action in actions.iter() {
            res.append(&mut self.actions[action].get());
        }
        Ok(res)
    }
}

fn default_shell() -> String {
    "sh".into()
}
