/*
 * Copyright (c) 2023, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use comfy_table::{Cell, CellAlignment};
use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    create_and_print_table, delete_request, generate_url, get_request, handle_error_and_exit,
    post_request, print_output, Global,
};

#[allow(clippy::missing_panics_doc)]
pub fn create(
    global: &Global,
    name: &str,
    start_date: &str,
    end_date: &str,
    origin_board_id: i64,
    goal: &str,
) {
    let url: String = generate_url(&global.domain, "sprint", None);
    let payload: Value = json!({
            "originBoardId": origin_board_id,
            "goal": goal,
            "endDate": end_date,
            "startDate": start_date,
            "name": name
    });

    match post_request(&url, &payload, &global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Failed to create sprint: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            print_output(&format!(
                "Sprint {} created with id: {}",
                json["name"], json["id"]
            ));
        }
    }
}
#[allow(clippy::missing_panics_doc)]
pub fn delete(global: &Global, sprint_id: i64) {
    let url: String = generate_url(&global.domain, "sprint", Some(&format!("/{sprint_id}")));
    match delete_request(&url, &global.b64auth()) {
        Ok(_) => print_output(&format!("Sprint {sprint_id} deleted")),
        Err(e) => handle_error_and_exit(&format!("Impossible to delete sprint {sprint_id}: {e}")),
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn get(global: &Global, sprint_id: i64) {
    let url: String = generate_url(&global.domain, "sprint", Some(&format!("/{sprint_id}")));
    match get_request(&url, &global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Failed to get sprint: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Faileed to parse json");
            let rows: Vec<Vec<Cell>> = json
                .as_array()
                .unwrap()
                .par_iter()
                .map(|x| {
                    vec![
                        Cell::new(x["id"].as_str().unwrap()),
                        Cell::new(x["name"].as_str().unwrap()),
                        Cell::new(x["state"].as_str().unwrap_or("")),
                        Cell::new(x["goal"].as_str().unwrap_or("")),
                    ]
                })
                .collect();
            create_and_print_table(
                vec!["Id", "Name", "State", "Goal"],
                &HashMap::from([
                    (0, CellAlignment::Center),
                    (1, CellAlignment::Center),
                    (2, CellAlignment::Center),
                    (3, CellAlignment::Center),
                ]),
                rows,
            );
        }
    }
}
