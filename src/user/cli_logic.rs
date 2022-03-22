use crate::user::functions;
use clap::ArgMatches;

pub fn get_account_id(matches: &ArgMatches, args: &ArgMatches) {
    functions::get_account_id(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("email_address").unwrap(),
    );
}
