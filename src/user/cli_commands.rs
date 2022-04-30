use clap::{Arg, Command};

pub fn get_account_id() -> Command<'static> {
    return Command::new("get_account_id")
        .about("Get account id")
        .visible_aliases(&["gai", "get-account-id"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("email_address")
                .help("Email address")
                .required(true),
        );
}
