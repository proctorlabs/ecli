use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Open { file: PathBuf },
    Generate { file: PathBuf },
}

pub fn get_args() -> Command {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::SubcommandsNegateReqs,
            AppSettings::VersionlessSubcommands,
            AppSettings::DisableHelpSubcommand,
            AppSettings::ArgsNegateSubcommands,
        ])
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("file")
                .index(1)
                .required(true)
                .help("Open the specified menu"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Generate a new config")
                .arg(
                    Arg::with_name("target")
                        .index(1)
                        .help("filename to create")
                        .required(true),
                )
                .alias("gen"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("new") => {
            let m = matches.subcommand_matches("new").unwrap();
            Command::Generate {
                file: value_t_or_exit!(m.value_of("target"), PathBuf),
            }
        }
        _ => Command::Open {
            file: value_t_or_exit!(matches.value_of("file"), PathBuf),
        },
    }
}
