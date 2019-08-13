mod choice;
mod prompt;

use super::*;
pub use {choice::*, prompt::*};

pub trait Screen: std::fmt::Debug {
    fn process_input(&mut self, key: Key) -> Result<ActionResult>;
    fn init(&self, state: &mut State) -> Result<()>;
    fn render(&self, state: &mut State) -> Result<()>;
}

pub type ScreenObj = Box<dyn Screen>;

#[derive(Default)]
pub struct ActionResult {
    pub input: Option<String>,
    pub actions: Option<Vec<Action>>,
}

impl ActionResult {
    pub fn action_needed(&self) -> bool {
        if let Some(actions) = &self.actions {
            !actions.is_empty()
        } else {
            false
        }
    }
}
