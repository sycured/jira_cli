/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use rayon::prelude::*;
use serde_json::Value;

use crate::{generate_url, get_request, handle_error_and_exit, print_output, Global};

#[allow(clippy::missing_panics_doc)]
pub fn list(global: &Global, start_at: &str, max_results: &str) {
    let url: String = generate_url(
        &global.domain,
        "label",
        Some(&format!("?startAt={start_at}&maxResults={max_results}")),
    );
    match get_request(&url, &global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Impossible to list labels: {e}")),
        Ok(r) => {
            let json: Value = r.json().expect("Failed to parse json");
            json["values"].as_array().unwrap().par_iter().for_each(|x| {
                print_output(&format!("{x}"));
            });
        }
    }
}
