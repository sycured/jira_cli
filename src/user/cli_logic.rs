use crate::user::functions;
use clap::ArgMatches;
use std::collections::HashMap;

pub fn get_account_id(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_account_id(
        global["domain"],
        global["user"],
        global["token"],
        args.value_of("email_address").unwrap(),
    );
}
