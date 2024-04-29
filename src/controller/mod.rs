use serde::Serialize;

use Method::{Get, Post};

use crate::database::mysql::Pool;
use crate::tcp::request::{Method, Request};

mod item_controller;

pub struct ControllerResult {
    pub status_code: u16,
    pub content: String,
}

impl ControllerResult {
    fn ok<S: Serialize>(s: S) -> Self {
        ControllerResult { status_code: 200, content: serde_json::to_string(&s).unwrap() }
    }

    fn bad_request<S: Into<String>>(s: S) -> Self {
        ControllerResult { status_code: 400, content: s.into() }
    }

    fn not_found() -> Self {
        ControllerResult { status_code: 404, content: String::from("not found") }
    }

    fn internal_server_error<S: Into<String>>(s: S) -> Self {
        ControllerResult { status_code: 500, content: s.into() }
    }
}

pub fn route(request: &Request) -> Result<ControllerResult, String> {
    let mut pool = Pool::new("localhost", "13306", "app", "secret", "sales");

    match (&request.method, request.target.as_str()) {
        (&Get, "/item/all") => item_controller::all(&mut pool),
        (&Post, "/item/create") => match request.parameter.get("code") {
            Ok(code) => item_controller::create(&mut pool, code),
            Err(e) => Ok(ControllerResult::bad_request(e)),
        },
        (&Get, "/error") => Ok(ControllerResult::internal_server_error("foo error")),
        _ => Ok(ControllerResult::not_found()),
    }
}
