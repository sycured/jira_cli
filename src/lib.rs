/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::process::exit;

use attohttpc::{delete, get, post, put, Response};
use base64::encode as b64encode;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, ContentArrangement,
    Table,
};
use dialoguer::Confirm;
use either::Either;
use serde_json::Value;

fn b64auth(user: &str, token: &str) -> String {
    b64encode(format!("{}:{}", user, token))
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn confirm(prompt: String) -> bool {
    Confirm::new().with_prompt(prompt).interact().unwrap()
}

#[allow(clippy::missing_panics_doc)]
#[inline]
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
    println!("{}", table);
}

#[inline]
pub fn delete_request(url: &str, user: &str, token: &str, success_message: &str) {
    let resp = delete(url)
        .header(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .send();
    match resp {
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
        Ok(_) => {
            println!("{}", success_message);
        }
    }
}

#[inline]
#[must_use]
pub fn get_request(url: &str, user: &str, token: &str) -> Response {
    let resp = get(url)
        .header("Accept", "application/json")
        .header_append(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .send();
    match resp {
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
        Ok(response) => response,
    }
}

#[allow(clippy::missing_panics_doc)]
#[inline]
#[must_use]
pub fn post_request(
    url: &str,
    payload: &Value,
    user: &str,
    token: &str,
    return_response: bool,
) -> Either<bool, Response> {
    let resp = post(url)
        .header("Accept", "application/json")
        .header_append(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .json(payload)
        .unwrap()
        .send();
    match resp {
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
        Ok(response) => {
            if return_response {
                Either::Right(response)
            } else {
                Either::Left(response.status().is_success())
            }
        }
    }
}

#[allow(clippy::missing_panics_doc)]
#[inline]
pub fn put_request(url: &str, payload: &Value, user: &str, token: &str, success_message: &str) {
    let resp = put(url)
        .header("Accept", "application/json")
        .header_append(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .json(payload)
        .unwrap()
        .send();
    match resp {
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
        Ok(_) => {
            println!("{}", success_message);
        }
    }
}
