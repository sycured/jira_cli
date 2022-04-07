use std::process::exit;

use base64::encode as b64encode;
use either::Either;
use serde_json::Value;
use ureq::{delete, get, post, put, Response};

fn b64auth(user: &str, token: &str) -> String {
    return b64encode(format!("{}:{}", user, token));
}

#[inline]
pub fn delete_request(url: &str, user: &str, token: &str, success_message: &str) {
    let resp = delete(url)
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .call();
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
        .set("Accept", "application/json")
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .call();
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
    payload: Value,
    user: &str,
    token: &str,
    success_message: &str,
    return_response: bool,
) -> Either<(), Response> {
    let resp = post(url)
        .set("Accept", "application/json")
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .send_json(payload);
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
pub fn put_request(url: &str, payload: Value, user: &str, token: &str, success_message: &str) {
    let resp = put(url)
        .set("Accept", "application/json")
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(user, token)),
        )
        .send_json(payload);
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
