use super::*;
use crate::templates::*;
use termion::input::TermRead;

pub fn exec(mut state: &mut State, result: &ActionResult) -> Result<()> {
    if let Some(actions) = &result.actions {
        let input = result
            .input
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_default();
        for action in state.config.get_actions(actions)?.iter() {
            match render_action(action)? {
                Action::Nav(Nav::Pop) => {
                    state.pop()?;
                }
                Action::Nav(Nav::Exit) => while state.pop().is_ok() {},
                Action::Nav(Nav::Pause) => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    println!("(Press any key to continue)");
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
                Action::Script { script, shell, .. } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    Command::new("/usr/bin/env")
                        .args(vec![shell.as_str(), "-c", script.as_str()])
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
                Action::Command { command, args, .. } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    Command::new(&command)
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
                Action::Prompt {
                    prompt,
                    val,
                    password,
                } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    draw!(state.r; @style: default -> "{} ➜ ", prompt);
                    state.r.flush()?;
                    let res = if password {
                        let res = std::io::stdin().read_passwd(&mut vec![])?;
                        draw!(state.r; -> "{}", '\n');
                        res.unwrap_or_default().trim().to_string()
                    } else {
                        let mut buf: String = String::new();
                        std::io::stdin().read_line(&mut buf)?;
                        buf.trim().to_string()
                    };
                    crate::templates::context_set(&val, &res)?;
                }
                Action::Print { print } => {
                    state.r.set_render_mode(RenderMode::Standard)?;
                    draw!(state.r; @style: default -> "{}\n", print);
                }
                Action::Check { check, pass, fail } => {
                    return if check.parse() == Ok(true) {
                        exec(
                            state,
                            &ActionResult {
                                input: Some(input),
                                actions: Some(pass.get()),
                            },
                        )
                    } else {
                        exec(
                            state,
                            &ActionResult {
                                input: Some(input),
                                actions: Some(fail.get()),
                            },
                        )
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
        Action::Check { ref mut check, .. } => {
            let tmp = format!("{{{{ {check} }}}}", check = check);
            *check = render(&tmp).unwrap_or_default();
        }
        Action::Prompt { ref mut prompt, .. } => {
            *prompt = render(prompt)?;
        }
        Action::Print { ref mut print } => {
            *print = render(print)?;
        }
        _ => {}
    }
    Ok(a)
}

pub fn get_screen(menu: Menu) -> Result<ScreenObj> {
    match menu {
        Menu::Choice(m) => ChoiceScreen::new(m),
    }
}
