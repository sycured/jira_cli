use std::collections::HashMap;

use clap::ArgMatches;

use super::functions;

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
        global,
        args.value_of("email").unwrap(),
        args.value_of("display_name").unwrap(),
    );
}

pub fn delete(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete(global, args.value_of("account_id").unwrap());
}

pub fn get_account_id(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_account_id(global, args.value_of("email_address").unwrap());
}

pub fn get_user_groups(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_user_groups(global, args.value_of("account_id").unwrap());
}
