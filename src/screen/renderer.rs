use super::*;

pub enum Styles {
    Default,
    Selected,
}

impl Styles {
    pub fn pick(&self, styles: &StyleConfig) -> Style {
        match self {
            Styles::Default => styles.default.clone(),
            Styles::Selected => styles.selected.clone(),
        }
    }
}

pub struct Renderer {
    pub size: (u16, u16),
    pub term: RawTerminal<Stdout>,
    pub styles: StyleConfig,
}

impl Renderer {
    pub fn new(config: &AppConfig) -> Result<Self> {
        let stdout = stdout().into_raw_mode()?;
        let size = terminal_size()?;
        let mut res = Renderer {
            size,
            term: stdout,
            styles: config.styles.clone(),
        };
        res.clear()?;
        res.flush()?;
        Ok(res)
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
        write!(
            self.term,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.term.flush()?;
        Ok(())
    }

    pub fn halt(&mut self) -> Result<()> {
        self.clear()?;
        self.flush()?;
        self.term.suspend_raw_mode()?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.term.activate_raw_mode()?;
        self.clear()?;
        self.flush()?;
        Ok(())
    }

    pub fn write(&mut self, style: Styles, text: &str, loc: (u16, u16)) -> Result<()> {
        let style = style.pick(&self.styles);
        let cursor = match style.alignment {
            Alignment::Left => loc,
            Alignment::Center => (loc.0 - (text.len() as u16 / 2), loc.1),
            Alignment::Right => (loc.0 - (text.len() as u16), loc.1),
        };
        write!(
            self.term,
            "{}{}{}{}{}{}",
            Goto(cursor.0, cursor.1),
            style.bg,
            style.fg,
            text,
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )?;
        Ok(())
    }
}
