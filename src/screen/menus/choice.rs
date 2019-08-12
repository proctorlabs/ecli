use super::*;

pub struct ChoiceScreen {
    pub menu: ChoiceMenu,
    pub selected: usize,
}

impl ChoiceScreen {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(menu: ChoiceMenu) -> Result<Box<dyn Screen>> {
        Ok(Box::new(ChoiceScreen { menu, selected: 0 }))
    }
}

impl Screen for ChoiceScreen {
    fn process_input(&mut self, key: Key) -> Result<Option<Vec<Action>>> {
        match key {
            Key::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            Key::Down => {
                if self.selected < (self.menu.entries.len() - 1) {
                    self.selected += 1;
                }
            }
            Key::Char('\n') => {
                return Ok(Some(self.menu.entries[self.selected].actions.clone()));
            }
            _ => {}
        };
        Ok(None)
    }

    fn init(&mut self, r: &mut Renderer) -> Result<()> {
        r.set_render_mode(RenderMode::Raw)?;
        Ok(())
    }

    fn render(&mut self, renderer: &mut Renderer) -> Result<()> {
        renderer.set_render_mode(RenderMode::Raw)?;
        renderer.border()?;
        draw!(renderer @bold
            @style: default
            @loc: ((renderer.size.0 / 2) - (self.menu.title.len() as u16 / 2 - 2),1)
            -> " {} ", self.menu.title);

        for (i, item) in self.menu.entries.iter().enumerate() {
            if i == self.selected {
                draw!(renderer @bold @style: selected @loc: (4, (i as u16) + 3) -> "⮞ {}", item.text);
            } else {
                draw!(renderer @style: default @loc: (6, (i as u16) + 3) -> "{}", item.text);
            };
        }

        draw!(renderer @style: default @loc: (renderer.size.0 - 25 , 2) -> "Width: {}, Height: {}",
            renderer.size.0,
            renderer.size.1
        );
        Ok(())
    }
}
