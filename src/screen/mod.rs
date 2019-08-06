mod exec;
mod menus;
mod renderer;

use {
    crate::{config::*, *},
    exec::*,
    menus::*,
    renderer::*,
    std::{
        io::{stdin, stdout, Read, Stdout, Write},
        process::{Command, Stdio},
    },
    termion::{
        cursor::Goto,
        event::Key,
        input::TermRead,
        raw::{IntoRawMode, RawTerminal},
        terminal_size,
    },
};

pub struct State {
    pub config: AppConfig,
    pub stack: Vec<Screen>,
}

pub fn enter(config: AppConfig) -> Result<()> {
    let stdin = stdin();
    let mut renderer = Renderer::new(&config)?;
    let screen = Screen::new(config.menus["main"].clone())?;
    let mut state = State {
        config,
        stack: vec![screen],
    };
    state.stack[0].render(&mut renderer)?;

    for c in stdin.keys() {
        let key = c?;
        if Key::Ctrl('c') == key {
            break;
        }
        let action = state.stack.last_mut().unwrap().input(key)?;
        if let Some(action) = action {
            exec(&mut renderer, &action, &mut state)?;
        }
        if state.stack.is_empty() {
            break;
        }
        write!(renderer.term, "{}", termion::clear::All)?;
        state.stack.last_mut().unwrap().render(&mut renderer)?;
    }

    renderer.clear()?;
    renderer.flush()?;
    Ok(())
}
