use super::*;
use crate::templates::*;

#[derive(Debug)]
pub struct PromptScreen {
    pub menu: PromptMenu,
    pub input: String,
}

impl PromptScreen {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(menu: PromptMenu) -> Result<Box<dyn Screen>> {
        Ok(Box::new(PromptScreen {
            menu,
            input: Default::default(),
        }))
    }
}

impl Screen for PromptScreen {
    fn process_input(&mut self, key: Key) -> Result<ActionResult> {
        let mut result = ActionResult::default();
        match key {
            Key::Char('\n') => {
                result.input = Some(self.input.clone());
                result.actions = Some(self.menu.then.get());
                return Ok(result);
            }
            Key::Char(c) => self.input = format!("{}{}", self.input, c),
            _ => {}
        };
        Ok(result)
    }

    fn init(&self, state: &mut State) -> Result<()> {
        let prompt = render(&self.menu.prompt)?;
        let r = &mut state.r;
        r.set_render_mode(RenderMode::Standard)?;
        draw!(r; @style: default -> "{} ➜ ", prompt);
        Ok(())
    }

    fn render(&self, state: &mut State) -> Result<()> {
        let r = &mut state.r;
        r.set_render_mode(RenderMode::Standard)?;
        Ok(())
    }
}
