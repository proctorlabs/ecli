use super::*;

pub fn exec(renderer: &mut Renderer, action: &Action, state: &mut State) -> Result<()> {
    match action {
        Action::Script { script, shell, .. } => {
            renderer.set_render_mode(RenderMode::Standard)?;
            Command::new("/usr/bin/env")
                .args(vec![shell.as_str(), "-c", script.as_str()])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?
                .wait()?;

            println!("(Process exited, press any key to continue)");
            std::io::stdin().read_exact(&mut [0])?;
        }
        Action::Command { command, args, .. } => {
            renderer.set_render_mode(RenderMode::Standard)?;
            Command::new(&command)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?
                .wait()?;

            println!("(Process exited, press any key to continue)");
            std::io::stdin().read_exact(&mut [0])?;
        }
        Action::Goto { goto } => {
            let mut new = get_screen(state.config.menus[goto].clone())?;
            new.init(renderer)?;
            state.stack.push(new);
        }
        Action::Return { .. } => {
            state.stack.pop();
        }
        Action::None { .. } => {}
    };
    Ok(())
}

pub fn get_screen(menu: Menu) -> Result<ScreenObj> {
    match menu {
        Menu::Choice(m) => ChoiceScreen::new(m),
        Menu::Prompt(m) => PromptScreen::new(m),
    }
}
