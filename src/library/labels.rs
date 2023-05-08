/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::process::exit;

use rayon::prelude::*;
use serde_json::Value;

use crate::{get_request, urls::URLS};

#[allow(clippy::missing_panics_doc)]
pub fn list(domain: &str, user: &str, token: &str, start_at: &str, max_results: &str) {
    let url: String = format!(
        "https://{}{}?startAt={}&maxResults={}",
        domain, URLS["label"], start_at, max_results
    );
    match get_request(&url, user, token) {
        Err(e) => {
            eprintln!("Impossible to list labels: {e}");
            exit(1);
        }
        Ok(r) => {
            let json: Value = r.json().unwrap();
            json["values"].as_array().unwrap().par_iter().for_each(|x| {
                println!("{x}");
            });
        }
    }
}
