mod actions;
mod menus;
mod misc;
mod styles;

use crate::*;
pub use actions::*;
pub use menus::*;
pub use misc::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{collections::BTreeMap, fmt, path::PathBuf};
pub use styles::*;
pub use termion::color::{self, Color as TermColor};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct Options {
    #[serde(default)]
    pub debug: bool,
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
