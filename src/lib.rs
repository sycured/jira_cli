/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]

use std::{collections::HashMap, error::Error};

use attohttpc::{delete, get, post, put, Response};
use base64::encode as b64encode;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, ContentArrangement,
    Table,
};
use dialoguer::Confirm;
use serde_json::Value;

fn b64auth(user: &str, token: &str) -> String {
    b64encode(format!("{user}:{token}"))
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn confirm(prompt: String) -> bool {
    Confirm::new().with_prompt(prompt).interact().unwrap()
}

#[allow(clippy::missing_panics_doc)]
pub fn create_and_print_table<S: std::hash::BuildHasher>(
    header: Vec<&str>,
    column_alignment: &HashMap<usize, CellAlignment, S>,
    rows: Vec<Vec<Cell>>,
) {
    let mut table = Table::new();
    table
        .set_header(header)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);
    for (key, value) in column_alignment.iter() {
        table.column_mut(*key).unwrap().set_cell_alignment(*value);
    }
    for row in rows {
        table.add_row(row);
    }
    println!("{table}");
}

#[allow(clippy::missing_errors_doc)]
pub fn delete_request(url: &str, user: &str, token: &str) -> Result<Response, Box<dyn Error>> {
    Ok(delete(url)
        .header(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .send()?)
}

#[allow(clippy::missing_errors_doc)]
pub fn get_request(url: &str, user: &str, token: &str) -> Result<Response, Box<dyn Error>> {
    Ok(get(url)
        .header("Accept", "application/json")
        .header_append(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .send()?)
}

#[allow(clippy::missing_errors_doc)]
pub fn post_request(
    url: &str,
    payload: &Value,
    user: &str,
    token: &str,
) -> Result<Response, Box<dyn Error>> {
    Ok(post(url)
        .header("Accept", "application/json")
        .header_append(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .json(payload)?
        .send()?)
}

#[allow(clippy::missing_errors_doc)]
pub fn put_request(
    url: &str,
    payload: &Value,
    user: &str,
    token: &str,
) -> Result<Response, Box<dyn Error>> {
    Ok(put(url)
        .header("Accept", "application/json")
        .header_append(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .json(payload)?
        .send()?)
}
