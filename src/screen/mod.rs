mod exec;
mod menus;
mod renderer;

use {
    crate::{config::*, *},
    exec::*,
    handlebars::Handlebars,
    menus::*,
    renderer::*,
    std::{
        collections::HashMap,
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
    pub template: Handlebars,
    pub vars: HashMap<String, String>,
}

pub fn enter(config: AppConfig) -> Result<()> {
    let mut renderer = Renderer::new(&config)?;
    let mut screen_obj = exec::get_screen(config.menus["main"].clone())?;
    screen_obj.init(&mut renderer)?;
    let mut state = State {
        config,
        stack: vec![screen_obj],
        template: Handlebars::new(),
        vars: HashMap::new(),
    };

    let mut keys = stdin().keys();
    while !state.stack.is_empty() {
        render_screen(&mut renderer, state.stack.last_mut().unwrap())?;

        let key = keys.next().unwrap_or(Ok(Key::Null))?;
        if Key::Ctrl('c') == key {
            break;
        }

        let action = state.stack.last_mut().unwrap().process_input(key)?;

        if let Some(action) = action {
            for a in action.iter() {
                exec(&mut renderer, a, &mut state)?;
            }
        }
    }

    renderer.set_render_mode(RenderMode::Standard)?;
    Ok(())
}

#[allow(clippy::borrowed_box)]
fn render_screen(r: &mut Renderer, s: &mut Box<dyn Screen>) -> Result<()> {
    r.begin()?;
    s.render(r)?;
    r.flush()?;
    Ok(())
}
