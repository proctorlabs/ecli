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
    fn input(&mut self, key: Key) -> Result<Option<Action>> {
        match key {
            Key::Char('\n') => {
                return Ok(Some(Action::Goto {
                    goto: "main".into(),
                }))
            }
            Key::Char(c) => self.input = format!("{}{}", self.input, c),
            _ => {}
        };
        Ok(None)
    }

    fn render(&mut self, renderer: &mut Renderer) -> Result<()> {
        draw!(renderer @style: default @loc: (4, renderer.size.1 - 2) -> "{}", self.menu.prompt);
        draw!(renderer @style: default @loc: (4, renderer.size.1 - 1) -> "{}", self.input);
        Ok(())
    }
}
