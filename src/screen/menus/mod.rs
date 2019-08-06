use super::*;

pub struct Screen {
    pub menu: Menu,
    pub selected: usize,
}

impl Screen {
    pub fn new(menu: Menu) -> Result<Self> {
        Ok(Screen { menu, selected: 0 })
    }

    pub fn input(&mut self, key: Key) -> Result<Option<Action>> {
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
                return Ok(Some(self.menu.entries[self.selected].action.clone()));
            }
            _ => {}
        };
        Ok(None)
    }

    pub fn render(&mut self, renderer: &mut Renderer) -> Result<()> {
        renderer.border()?;
        renderer.write(
            Styles::Default,
            &format!(
                " {}{}{} ",
                termion::style::Bold,
                self.menu.title,
                termion::style::Reset
            ),
            (
                (renderer.size.0 / 2) - (self.menu.title.len() as u16 / 2 - 2),
                1,
            ),
        )?;
        for (i, item) in self.menu.entries.iter().enumerate() {
            let (style, offset, prefix) = if i == self.selected {
                (Styles::Selected, 4, format!("⮞ {}", termion::style::Bold))
            } else {
                (Styles::Default, 6, String::new())
            };
            let t = format!("{}{}{}", prefix, item.text, termion::style::Reset);
            renderer.write(style, &t, (offset, (i + 3) as u16))?
        }

        write!(
            renderer.term,
            "{}Width: {}, Height: {}{}",
            termion::cursor::Goto(renderer.size.0 - 25, 2),
            renderer.size.0,
            renderer.size.1,
            termion::cursor::Goto(renderer.size.0, renderer.size.1)
        )?;
        renderer.flush()?;
        Ok(())
    }
}
