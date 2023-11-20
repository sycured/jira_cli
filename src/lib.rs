/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]
pub mod library;
use attohttpc::{Error, Method, Response};
use base64::{engine::general_purpose as b64, Engine};
use clap::ArgMatches;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, ContentArrangement,
    Table,
};
use dialoguer::Confirm;
pub use library::*;
use serde_json::Value;
use std::{collections::HashMap, process::exit};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Global {
    pub domain: String,
    pub user: String,
    pub token: String,
}

pub trait Authorization {
    fn b64auth(&self) -> String;
}

impl Authorization for Global {
    fn b64auth(&self) -> String {
        b64::STANDARD.encode(format!("{}:{}", self.user, self.token))
    }
}

pub fn handle_error_and_exit(message: &str) {
    eprintln!("{message}");
    exit(1);
}

pub fn print_output(output: &str) {
    println!("{output}");
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn get_single_arg<'a>(args: &'a ArgMatches, arg_name: &str) -> &'a str {
    args.get_one::<String>(arg_name).unwrap().as_str()
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

    for (key, value) in column_alignment {
        table.column_mut(*key).unwrap().set_cell_alignment(*value);
    }

    for row in rows {
        table.add_row(row);
    }

    println!("{table}");
}

#[allow(clippy::missing_errors_doc)]
pub fn make_request<A: Authorization>(
    method: Method,
    url: &str,
    payload: Option<&Value>,
    b64auth: &A,
) -> Result<Response, Error> {
    let builder = attohttpc::RequestBuilder::new(method, url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Basic {}", b64auth.b64auth()));

    let payload = payload.unwrap_or(&Value::Null);
    let builder = builder.json(payload)?;

    builder.send().and_then(Response::error_for_status)
}

//New enum for HTTP methods
pub enum HttpRequest {
    GET,
    POST,
    PUT,
    DELETE,
}

//Refactored request functions - now one function that accepts the enum value
#[allow(clippy::missing_errors_doc)]
pub fn request(
    method: &HttpRequest,
    url: &str,
    payload: Option<&Value>,
    b64auth: &Global,
) -> Result<Response, Error> {
    let method = match method {
        HttpRequest::GET => Method::GET,
        HttpRequest::POST => Method::POST,
        HttpRequest::PUT => Method::PUT,
        HttpRequest::DELETE => Method::DELETE,
    };

    make_request(method, url, payload, b64auth)
}
