/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use attohttpc::{get, Error as AttohttpcError, Response};
use clap::{crate_version, Command};
use serde_json::Value;
use std::{error::Error, process::exit};

fn get_request(url: &str) -> Result<Response, AttohttpcError> {
    get(url).header("Accept", "application/json").send()
}

fn get_api_url() -> String {
    env!("CARGO_PKG_REPOSITORY").replace("github.com", "api.github.com/repos") + "/releases/latest"
}

fn get_latest_version(api_url: &str) -> Result<String, Box<dyn Error>> {
    let response = get_request(&api_url)?;

    if !response.is_success() {
        return Err(format!(
            "Failed to fetch latest version. Server responded with status code: {}",
            response.status()
        )
        .into());
    }

    let json: Value = response.json()?;
    Ok(extract_version_from_response(json)?)
}

fn extract_version_from_response(json: Value) -> Result<String, Box<dyn Error>> {
    let version = json["tag_name"]
        .as_str()
        .ok_or("Failed to parse version from JSON")?
        .trim_start_matches('v')
        .to_string();
    Ok(version)
}

fn print_version_info(actual_version: &str, latest_version: &str) {
    match actual_version {
        x if x == latest_version => {
            println!("You are using the latest version.");
        }
        x if x < latest_version => {
            println!(
                "You are using an outdated version. The latest version is {}",
                latest_version
            );
        }
        x if x > latest_version => {
            println!(
                "You are using an unreleased version. The latest version is {}",
                latest_version
            );
        }
        _ => {
            println!(
                "You are using an unknown version. The latest version is {}",
                latest_version
            );
        }
    }
}

fn check_version() {
    let actual_version: &str = crate_version!();
    let api_url = get_api_url();

    match get_latest_version(&api_url) {
        Ok(latest_version) => {
            print_version_info(actual_version, &latest_version);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}

pub fn cli_commands() -> Command {
    Command::new("check_version")
        .visible_alias("check-version")
        .about("Check if a new version is available")
        .ignore_errors(true)
}

pub fn logic_commands() {
    check_version();
}
