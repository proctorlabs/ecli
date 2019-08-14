mod exec;
mod menus;
mod renderer;

use {
    crate::{config::*, *},
    exec::*,
    menus::*,
    renderer::*,
    std::{
        fmt,
        io::{stdin, stdout, Read, Stdout, Write},
        process::{Command, Stdio},
    },
    termion::{
        event::Key,
        input::TermRead,
        raw::{IntoRawMode, RawTerminal},
        terminal_size,
    },
};

pub struct State {
    pub config: AppConfig,
    pub stack: Vec<ScreenObj>,
    pub r: Renderer,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( X: {} Y: {} Screen: '{:?}')",
            self.r.size.0,
            self.r.size.1,
            self.stack.last()
        )
    }
}

impl State {
    pub fn pop(&mut self) -> Result<ScreenObj> {
        self.stack
            .pop()
            .ok_or_else(|| AppError::Fatal("Stack is empty!".into()))
    }

    pub fn push(&mut self, screen: ScreenObj) -> Result<()> {
        self.stack.push(screen);
        Ok(())
    }

    fn init_screen(&mut self) -> Result<()> {
        let s = self.pop()?;
        s.init(self)?;
        self.push(s)?;
        Ok(())
    }

    fn render_screen(&mut self) -> Result<()> {
        self.r.begin()?;
        let s = self.pop()?;
        s.render(self)?;
        self.push(s)?;
        if self.config.options.debug && !self.r.has_cursor()? {
            let debug = self.to_string();
            let debug_loc = (4, self.r.size.1 - (debug.len() as u16 / self.r.size.0) - 2);
            draw!(self.r; @style: default @loc: (debug_loc.0, debug_loc.1) -> "{}", debug);
        }
        self.r.flush()?;
        Ok(())
    }

    fn process_input(&mut self, key: Key) -> Result<()> {
        let action;

        if let Some(s) = self.stack.last_mut() {
            action = s.process_input(key)?;
            if action.action_needed() {
                exec(self, &action)?;
            }
        }

        Ok(())
    }

    fn running(&self) -> Result<bool> {
        Ok(!self.stack.is_empty())
    }
}

pub fn enter(config: AppConfig) -> Result<()> {
    let mut state = State {
        stack: vec![exec::get_screen(config.menus["main"].clone())?],
        r: Renderer::new(&config)?,
        config,
    };
    state.init_screen()?;

    let mut keys = stdin().keys();
    while state.running()? {
        state.render_screen()?;
        let key = keys.next().unwrap_or(Ok(Key::Null))?;
        if Key::Ctrl('c') == key {
            break;
        }
        state.process_input(key)?;
    }

    state.r.set_render_mode(RenderMode::Standard)?;
    Ok(())
}
