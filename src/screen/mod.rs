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
        event::Key,
        input::TermRead,
        raw::{IntoRawMode, RawTerminal},
        terminal_size,
    },
};

pub struct State {
    pub config: AppConfig,
    pub stack: Vec<ScreenObj>,
}

pub fn enter(config: AppConfig) -> Result<()> {
    let stdin = stdin();
    let mut renderer = Renderer::new(&config)?;
    let screen = exec::get_screen(config.menus["main"].clone())?;
    let mut state = State {
        config,
        stack: vec![screen],
    };
    state.stack[0].render(&mut renderer)?;
    renderer.flush()?;

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
        renderer.clear()?;
        state.stack.last_mut().unwrap().render(&mut renderer)?;
        draw!(renderer @loc: (renderer.size.0, renderer.size.1) <<); //reset cursor...
        renderer.flush()?;
    }

    renderer.halt()?;
    Ok(())
}
