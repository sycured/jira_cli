use std::{collections::HashMap, io::stdout};

use clap::Command;
use clap_complete::{generate, Generator, Shell};

mod check_version;
mod cli;
mod issue;
mod labels;
mod lib;
mod project;
mod user;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_owned(), &mut stdout());
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let global = HashMap::from([
        ("domain", matches.value_of("domain").unwrap()),
        ("token", matches.value_of("token").unwrap()),
        ("user", matches.value_of("user").unwrap()),
    ]);

    match matches.subcommand() {
        Some(("check_version", _)) => {
            check_version::logic_commands();
        }
        Some(("generate", args)) => {
            let shell = args.value_of_t::<Shell>("shell").unwrap();
            let mut cmd = cli::build_cli();
            print_completions(shell, &mut cmd);
        }
        Some(("issue", args)) => issue::logic_commands(global, args),
        Some(("labels", args)) => labels::logic_commands(global, args),
        Some(("project", args)) => project::logic_commands(global, args),
        Some(("user", args)) => user::logic_commands(global, args),
        _ => unreachable!(),
    }
}
