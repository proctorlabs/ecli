mod choice;
mod prompt;

use super::*;
pub use {choice::*, prompt::*};

pub trait Screen {
    fn input(&mut self, key: Key) -> Result<Option<Vec<Action>>>;
    fn render(&mut self, renderer: &mut Renderer) -> Result<()>;
}

pub type ScreenObj = Box<dyn Screen>;
