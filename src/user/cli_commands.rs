use clap::{Arg, Command};

pub fn create() -> Command<'static> {
    return Command::new("create")
        .about("Create a new user")
        .visible_aliases(&["au", "add_user", "add-user", "c"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("email")
                .help("The email address of the user")
                .required(true),
        )
        .arg(Arg::new("display_name")
            .help("A suggested display name for the user. If the user has an Atlassian account, their display name is not changed. If the user does not have an Atlassian account, this display name is used as a suggestion for creating an account. The user is sent an email asking them to set their display name and privacy preferences.")
            .required(true));
}

pub fn delete() -> Command<'static> {
    return Command::new("delete")
        .about("Delete a user")
        .visible_aliases(&["d", "del", "rm"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("account_id")
                .help("The account ID of the user to delete")
                .required(true),
        );
}

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

pub fn get_user_groups() -> Command<'static> {
    return Command::new("get_user_groups")
        .about("Get user groups")
        .visible_aliases(&["gg", "gug", "get-user-groups", "get_groups", "get-groups"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("account_id")
                .help("The account ID of the user")
                .required(true),
        );
}
