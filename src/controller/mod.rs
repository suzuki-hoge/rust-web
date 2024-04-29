use std::sync::Arc;

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
        ControllerResult { status_code: 200, content: Success::new(s).to_json() }
    }

    fn bad_request<S: Into<String>>(s: S) -> Self {
        ControllerResult { status_code: 400, content: Failure::new(s).to_json() }
    }

    fn not_found() -> Self {
        ControllerResult { status_code: 404, content: Failure::new("not found").to_json() }
    }

    fn internal_server_error<S: Into<String>>(s: S) -> Self {
        ControllerResult { status_code: 500, content: Failure::new(s).to_json() }
    }
}

#[derive(Serialize)]
struct Success<S>
where
    S: Serialize,
{
    result: String,
    content: S,
}

impl<S> Success<S>
where
    S: Serialize,
{
    fn new(s: S) -> Self {
        Self { result: String::from("success"), content: s }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize)]
struct Failure {
    result: String,
    message: String,
}

impl Failure {
    fn new<S: Into<String>>(s: S) -> Self {
        Self { result: String::from("failure"), message: s.into() }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn route(pool: Arc<Pool>, request: &Request) -> Result<ControllerResult, String> {
    match (&request.method, request.target.as_str()) {
        (&Get, "/read/light") => item_controller::read(pool, false),
        (&Get, "/read/cache") => item_controller::read(pool, true),
        (&Post, "/write/light") => item_controller::write(pool, false, request.parameter.get("code")),
        (&Post, "/write/cache") => item_controller::write(pool, true, request.parameter.get("code")),
        (&Post, "/write/heavy") => item_controller::block(pool),
        (&Get, "/thread/light") => item_controller::thread_sleep(0),
        (&Get, "/thread/heavy") => item_controller::thread_sleep(3),
        (&Get, "/error") => Ok(ControllerResult::internal_server_error("foo error")),
        _ => Ok(ControllerResult::not_found()),
    }
}
