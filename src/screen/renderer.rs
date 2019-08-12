use super::*;

#[derive(Debug, PartialEq)]
pub enum RenderMode {
    Unitialized,
    Raw,
    Standard,
}

pub struct Renderer {
    pub size: (u16, u16),
    pub term: RawTerminal<Stdout>,
    pub styles: StyleConfig,
    mode: RenderMode,
}

impl Renderer {
    pub fn new(config: &AppConfig) -> Result<Self> {
        Ok(Renderer {
            size: terminal_size()?,
            term: stdout().into_raw_mode()?,
            styles: config.styles.clone(),
            mode: RenderMode::Unitialized,
        })
    }

    pub fn set_render_mode(&mut self, mode: RenderMode) -> Result<()> {
        if self.mode != mode {
            match mode {
                RenderMode::Raw => {
                    self.term.activate_raw_mode()?;
                    self.size = terminal_size()?;
                    self.clear()?;
                    draw!(self -> "{}", termion::cursor::Hide);
                    self.flush()?;
                }
                RenderMode::Standard => {
                    self.clear()?;
                    draw!(self -> "{}", termion::cursor::Show);
                    self.flush()?;
                    self.size = terminal_size()?;
                    self.term.suspend_raw_mode()?;
                }
                _ => {
                    return Err(AppError::Info(format!(
                        "Cannot change modes to {:?}!",
                        mode
                    )))
                }
            }
            self.mode = mode;
        }
        Ok(())
    }

    pub fn border(&mut self) -> Result<()> {
        write!(self.term, "{}┏", termion::cursor::Goto(1, 1))?;
        for _ in 2..self.size.0 {
            write!(self.term, "━")?;
        }
        write!(self.term, "┓")?;

        for i in 2..self.size.1 {
            write!(
                self.term,
                "{}┃{}┃",
                termion::cursor::Goto(1, i),
                termion::cursor::Goto(self.size.0, i)
            )?;
        }

        write!(self.term, "{}┗", termion::cursor::Goto(1, self.size.1))?;
        for _ in 2..self.size.0 {
            write!(self.term, "━")?;
        }
        write!(self.term, "┛")?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        draw!(self -> "{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        Ok(())
    }

    pub fn begin(&mut self) -> Result<()> {
        match self.mode {
            RenderMode::Raw | RenderMode::Unitialized => {
                draw!(self -> "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                );
            }
            _ => {}
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.term.flush()?;
        Ok(())
    }
}
