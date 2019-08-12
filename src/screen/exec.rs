use super::*;

pub fn exec(renderer: &mut Renderer, action: &Action, state: &mut State) -> Result<()> {
    match render_action(action, &state.template, &state.vars)? {
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
            let mut new = get_screen(state.config.menus[&goto].clone())?;
            new.init(renderer)?;
            state.stack.push(new);
        }
        Action::Return { .. } => {
            state.stack.pop();
        }
    };
    Ok(())
}

fn render_action(
    action: &Action,
    t: &Handlebars,
    vars: &HashMap<String, String>,
) -> Result<Action> {
    let mut a = action.clone();
    match &mut a {
        Action::Script {
            ref mut script,
            ref mut shell,
        } => {
            *script = t.render_template(script, vars).unwrap();
            *shell = t.render_template(shell, vars).unwrap();
        }
        Action::Command {
            ref mut command,
            ref mut args,
        } => {
            *command = t.render_template(command, vars).unwrap();
            for a in args.iter_mut() {
                *a = t.render_template(a, vars).unwrap();
            }
        }
        _ => {}
    }
    Ok(a)
}

pub fn get_screen(menu: Menu) -> Result<ScreenObj> {
    match menu {
        Menu::Choice(m) => ChoiceScreen::new(m),
        Menu::Prompt(m) => PromptScreen::new(m),
    }
}
