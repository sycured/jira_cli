use std::process::exit;

use clap::{crate_version, Command};
use serde_json::Value;
use ureq::{get, Response};

fn get_request(url: &str) -> Response {
    let resp = get(url).set("Accept", "application/json").call();
    match resp {
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
        Ok(response) => response,
    }
}

fn check_version() {
    let actual_version: &str = crate_version!();
    let repository_url: &str = env!("CARGO_PKG_REPOSITORY");
    let api_url: String =
        repository_url.replace("github.com", "api.github.com/repos") + "/releases/latest";
    let resp: Response = get_request(&api_url);
    let json: Value = resp.into_json().unwrap();
    let latest_version: &str = json["tag_name"]
        .as_str()
        .unwrap()
        .strip_prefix('v')
        .unwrap();
    match actual_version {
        _ if actual_version == latest_version => {
            println!("You are using the latest version.");
        }
        _ if actual_version < latest_version => {
            println!("You are using an outdated version. The latest version is {latest_version}");
        }
        _ if actual_version > latest_version => {
            println!("You are using an unreleased version. The latest version is {latest_version}");
        }
        _ => {
            println!("You are using an unknown version. The latest version is {latest_version}");
        }
    }
}

pub fn cli_commands() -> Command<'static> {
    return Command::new("check_version")
        .visible_alias("check-version")
        .about("Check if a new version is available");
}

pub fn logic_commands() {
    check_version();
}
