use crate::lib::get_request;
use clap::{Arg, ArgMatches, Command};
use serde_json::Value;
use std::collections::HashMap;
use ureq::Response;

fn list_labels(domain: &str, user: &str, token: &str, start_at: &str, max_results: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/label?startAt={}&maxResults={}",
        domain, start_at, max_results
    );
    let resp: Response = get_request(&url, user, token);
    let json: Value = resp.into_json().unwrap();
    json["values"].as_array().unwrap().iter().for_each(|x| {
        println!("{}", x);
    });
}

pub fn cli_commands() -> Command<'static> {
    return Command::new("labels")
        .about("List available labels for the global label field")
        .visible_alias("l")
        .arg(
            Arg::new("max_results")
                .value_name("max_results")
                .help("The maximum number of items to return")
                .default_value("1000"),
        )
        .arg(
            Arg::new("start_at")
                .value_name("start_at")
                .help("The index of the first item to return")
                .default_value("0"),
        );
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    list_labels(
        global["domain"],
        global["user"],
        global["token"],
        args.value_of("start_at").unwrap_or(""),
        args.value_of("max_results").unwrap_or(""),
    );
}
