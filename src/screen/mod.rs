use crate::config::*;
use std::io::{stdin, stdout, Stdout, Write};
use std::process::{Command, Stdio};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub fn enter(menu: Menu) -> Result<(), String> {
    let stdin = stdin();
    let mut renderer = Renderer::new();
    let mut screen = Screen::new(menu);
    screen.render(&mut renderer, None);

    for c in stdin.keys() {
        let key = c.unwrap();
        if Key::Ctrl('c') == key {
            break;
        }
        write!(renderer.term, "{}", termion::clear::All).unwrap();
        screen.render(&mut renderer, Some(key));
    }

    renderer.clear();
    renderer.flush();
    Ok(())
}

pub struct Renderer {
    pub size: (u16, u16),
    pub term: RawTerminal<Stdout>,
}

impl Renderer {
    pub fn new() -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        let size = terminal_size().unwrap();
        let mut res = Renderer { size, term: stdout };
        res.clear();
        res.flush();
        res
    }

    pub fn clear(&mut self) {
        write!(
            self.term,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
    }

    pub fn flush(&mut self) {
        self.term.flush().unwrap()
    }

    pub fn write(&mut self, style: &Style, text: &str, loc: (u16, u16)) {
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
        )
        .unwrap();
    }
}

pub struct Screen {
    pub menu: Menu,
    pub selected: usize,
}

impl Screen {
    pub fn new(menu: Menu) -> Self {
        Screen { menu, selected: 0 }
    }

    pub fn render(&mut self, renderer: &mut Renderer, key: Option<Key>) {
        match key.unwrap_or_else(|| Key::Char(' ')) {
            Key::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            Key::Down => {
                if self.selected < (self.menu.menu.len() - 1) {
                    self.selected += 1;
                }
            }
            Key::Char('\n') => {
                let m = &self.menu.menu[self.selected];
                match m {
                    MenuItem::Script { cmd, .. } => {
                        renderer.clear();
                        renderer.flush();
                        renderer.term.suspend_raw_mode().unwrap();
                        Command::new("/usr/bin/env")
                            .args(vec!["bash", "-c", cmd])
                            .stdin(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .spawn()
                            .expect("Failed to exec script")
                            .wait()
                            .unwrap();
                        Command::new("/usr/bin/env")
                            .args(vec![
                                "bash",
                                "-c",
                                "read -p '(Process exited, press any key to continue)' -n 1 -r",
                            ])
                            .stdin(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .spawn()
                            .unwrap()
                            .wait()
                            .unwrap();
                        renderer.term.activate_raw_mode().unwrap();
                        renderer.clear();
                        renderer.flush();
                    }
                }
            }
            _ => {}
        }

        for (i, item) in self.menu.menu.iter().enumerate() {
            let (style, offset, prefix) = if i == self.selected {
                (&self.menu.selected, 4, "➜ ")
            } else {
                (&self.menu.default, 6, "")
            };
            match item {
                MenuItem::Script { text, .. } => {
                    let t = format!("{}{}", prefix, text);
                    renderer.write(style, &t, (offset, (i + 2) as u16))
                }
            };
        }

        write!(
            renderer.term,
            "{}{}Width: {}, Height: {}, Key: {:?}{}",
            termion::cursor::Goto(1, renderer.size.1),
            termion::clear::CurrentLine,
            renderer.size.0,
            renderer.size.1,
            key,
            termion::cursor::Goto(renderer.size.0, renderer.size.1)
        )
        .unwrap();

        renderer.flush();
    }
}
