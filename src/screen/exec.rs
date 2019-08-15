use super::*;
use crate::templates::*;

pub fn exec(mut state: &mut State, result: &ActionResult) -> Result<()> {
    if let Some(actions) = &result.actions {
        let input = result
            .input
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_default();
        for action in state
            .config
            .get_actions(OneOrMany::Many(actions.clone()))?
            .iter()
        {
            match render_action(action)? {
                Action::Script { script, shell, .. } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    let status = Command::new("/usr/bin/env")
                        .args(vec![shell.as_str(), "-c", script.as_str()])
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .spawn()?
                        .wait()?;

                    println!(
                        "(Process exited with status {}, press any key to continue)",
                        status
                    );
                    std::io::stdin().read_exact(&mut [0])?;
                }
                Action::Command { command, args, .. } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    let status = Command::new(&command)
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .spawn()?
                        .wait()?;

                    println!(
                        "(Process exited with status {}, press any key to continue)",
                        status
                    );
                    std::io::stdin().read_exact(&mut [0])?;
                }
                Action::Goto { goto } => {
                    let new = get_screen(state.config.menus[&goto].clone())?;
                    new.init(&mut state)?;
                    state.push(new)?;
                }
                Action::Set { set } => {
                    context_set(&set, &input)?;
                }
                Action::Nav(Nav::Pop) => {
                    state.pop()?;
                }
                Action::Nav(Nav::Exit) => while state.pop().is_ok() {},
                Action::Validate {
                    validate,
                    shell,
                    on_fail,
                    ..
                } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    let status = Command::new("/usr/bin/env")
                        .args(vec![shell.as_str(), "-c", &validate])
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .spawn()?
                        .wait()?;

                    if !status.success() {
                        let mut result = ActionResult::default();
                        result.actions = Some(on_fail.get());
                        return exec(state, &result);
                    }
                }
            };
        }
    }
    Ok(())
}

fn render_action(action: &Action) -> Result<Action> {
    let mut a = action.clone();
    match &mut a {
        Action::Script {
            ref mut script,
            ref mut shell,
        } => {
            *script = render(script)?;
            *shell = render(shell)?;
        }
        Action::Command {
            ref mut command,
            ref mut args,
        } => {
            *command = render(command)?;
            for a in args.iter_mut() {
                *a = render(a)?;
            }
        }
        Action::Validate {
            ref mut validate, ..
        } => {
            *validate = render(validate)?;
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
