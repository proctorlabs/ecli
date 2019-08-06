use super::*;

pub fn exec(renderer: &mut Renderer, action: &Action, state: &mut State) -> Result<()> {
    match action {
        Action::Script { script, shell, .. } => {
            renderer.halt()?;
            Command::new("/usr/bin/env")
                .args(vec![shell.as_str(), "-c", script.as_str()])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?
                .wait()?;

            println!("(Process exited, press any key to continue)");
            std::io::stdin().read_exact(&mut [0])?;
            renderer.resume()?;
        }
        Action::Command { command, args, .. } => {
            renderer.halt()?;
            Command::new(&command)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?
                .wait()?;

            println!("(Process exited, press any key to continue)");
            std::io::stdin().read_exact(&mut [0])?;
            renderer.resume()?;
        }
        Action::Goto { goto } => {
            let new = Screen::new(state.config.menus[goto].clone())?;
            state.stack.push(new);
        }
        Action::Return { .. } => {
            state.stack.pop();
        }
        Action::None { .. } => {}
    };
    Ok(())
}
