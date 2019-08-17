use super::*;

#[derive(Debug)]
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
    fn process_input(&mut self, key: Key) -> Result<ActionResult> {
        let mut result = ActionResult::default();
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
                result.actions = Some(self.menu.entries[self.selected].actions.get());
                return Ok(result);
            }
            _ => {}
        };
        Ok(result)
    }

    fn init(&self, state: &mut State) -> Result<()> {
        let r = &mut state.r;
        r.set_render_mode(RenderMode::Raw)?;
        Ok(())
    }

    fn render(&self, state: &mut State) -> Result<()> {
        let title = crate::templates::render(&self.menu.title)?;
        let entries: Vec<(usize, Result<String>)> = self
            .menu
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, crate::templates::render(&e.text)))
            .collect();
        state.r.set_render_mode(RenderMode::Raw)?;
        state.r.border()?;
        draw!(state.r; @bold
            @style: default
            @loc: ((state.r.size.0 / 2) - (title.len() as u16 / 2 - 2),1)
            -> " {} ", title);

        for (i, text) in entries.into_iter() {
            if i == self.selected {
                draw!(state.r; @bold @style: selected @loc: (4, (i as u16) + 3) -> "⮞ {}", &text?);
            } else {
                draw!(state.r; @style: default @loc: (6, (i as u16) + 3) -> "{}", &text?);
            };
        }

        Ok(())
    }
}
