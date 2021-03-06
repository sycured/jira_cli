use std::collections::HashMap;

use attohttpc::Response;
use clap::{Arg, ArgMatches, Command};
use rayon::prelude::*;
use serde_json::Value;

use super::{lib::get_request, urls::URLS};

fn list_labels(domain: &str, user: &str, token: &str, start_at: &str, max_results: &str) {
    let url: String = format!(
        "https://{}{}?startAt={}&maxResults={}",
        domain, URLS["label"], start_at, max_results
    );
    let resp: Response = get_request(&url, user, token);
    let json: Value = resp.json().unwrap();
    json["values"].as_array().unwrap().par_iter().for_each(|x| {
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
        args.get_one::<String>("start_at")
            .unwrap_or(&"".to_string())
            .as_str(),
        args.get_one::<String>("max_results")
            .unwrap_or(&"".to_string())
            .as_str(),
    );
}
