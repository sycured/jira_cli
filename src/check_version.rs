/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::process::exit;

use attohttpc::{get, Response};
use clap::{Command, crate_version};
use serde_json::Value;

fn get_request(url: &str) -> Response {
    let resp = get(url).header("Accept", "application/json").send();
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
    let api_url: String = env!("CARGO_PKG_REPOSITORY")
        .replace("github.com", "api.github.com/repos")
        + "/releases/latest";
    let json: Value = get_request(&api_url).json().unwrap();
    let latest_version: &str = json["tag_name"]
        .as_str()
        .unwrap()
        .strip_prefix('v')
        .unwrap();
    match actual_version {
        x if x == latest_version => {
            println!("You are using the latest version.");
        }
        x if x < latest_version => {
            println!("You are using an outdated version. The latest version is {latest_version}");
        }
        x if x > latest_version => {
            println!("You are using an unreleased version. The latest version is {latest_version}");
        }
        _ => {
            println!("You are using an unknown version. The latest version is {latest_version}");
        }
    }
}

pub fn cli_commands() -> Command {
    Command::new("check_version")
        .visible_alias("check-version")
        .about("Check if a new version is available")
}

pub fn logic_commands() {
    check_version();
}
