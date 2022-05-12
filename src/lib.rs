#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::process::exit;

use attohttpc::{delete, get, post, put, Response};
use base64::encode as b64encode;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, ContentArrangement,
    Table,
};
use either::Either;
use serde_json::Value;

fn b64auth(user: &str, token: &str) -> String {
    return b64encode(format!("{}:{}", user, token));
}

pub fn create_table(
    header: Vec<&str>,
    column_alignment: HashMap<usize, CellAlignment>,
    rows: Vec<Vec<Cell>>,
) -> Table {
    let mut table = Table::new();
    table
        .set_header(header)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);
    column_alignment.iter().for_each(|(key, value)| {
        table
            .get_column_mut(*key)
            .unwrap()
            .set_cell_alignment(*value);
    });
    rows.into_iter().for_each(|row| {
        table.add_row(row);
    });
    return table;
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

#[inline]
pub fn post_request(
    url: &str,
    payload: &Value,
    user: &str,
    token: &str,
    success_message: &str,
    return_response: bool,
) -> Either<(), Response> {
    let resp = post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
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
                #[allow(clippy::unit_arg)]
                Either::Left(println!("{}", success_message))
            }
        }
    }
}

#[inline]
pub fn put_request(url: &str, payload: &Value, user: &str, token: &str, success_message: &str) {
    let resp = put(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
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
