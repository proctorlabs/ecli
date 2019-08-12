use super::*;

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
    fn process_input(&mut self, key: Key) -> Result<Option<Vec<Action>>> {
        match key {
            Key::Char('\n') => {
                return Ok(Some(vec![
                    Action::Return { r#return: () },
                    self.menu.then.clone(),
                ]))
            }
            Key::Char(c) => self.input = format!("{}{}", self.input, c),
            _ => {}
        };
        Ok(None)
    }

    fn init(&mut self, r: &mut Renderer) -> Result<()> {
        r.set_render_mode(RenderMode::Standard)?;
        draw!(r @style: default -> "{} ➜ ", self.menu.prompt);
        Ok(())
    }

    fn render(&mut self, r: &mut Renderer) -> Result<()> {
        r.set_render_mode(RenderMode::Standard)?;
        Ok(())
    }
}
