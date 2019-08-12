mod choice;
mod prompt;

use super::*;
pub use {choice::*, prompt::*};

pub trait Screen {
    fn process_input(&mut self, key: Key) -> Result<Option<Vec<Action>>>;
    fn init(&mut self, renderer: &mut Renderer) -> Result<()>;
    fn render(&mut self, renderer: &mut Renderer) -> Result<()>;
}

pub type ScreenObj = Box<dyn Screen>;
