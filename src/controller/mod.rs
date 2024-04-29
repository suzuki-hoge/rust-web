use serde::Serialize;
use std::thread;
use std::time::Duration;

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

pub fn route(request: &Request) -> Result<ControllerResult, String> {
    let mut pool = Pool::new("localhost", "13306", "app", "secret", "sales");

    match (&request.method, request.target.as_str()) {
        (&Get, "/item/all") => item_controller::all(&mut pool),
        (&Post, "/item/create") => match request.parameter.get("code") {
            Ok(code) => item_controller::create(&mut pool, code),
            Err(e) => Ok(ControllerResult::bad_request(e)),
        },
        (&Get, "/error") => Ok(ControllerResult::internal_server_error("foo error")),
        (&Get, "/sleep") => {
            thread::sleep(Duration::from_secs(3));
            Ok(ControllerResult::ok("3 seconds slept"))
        }
        _ => Ok(ControllerResult::not_found()),
    }
}
