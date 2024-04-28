use crate::controller::ControllerResult;

pub fn all() -> ControllerResult {
    ControllerResult::ok("todo")
}

pub fn create(code: &String) -> ControllerResult {
    ControllerResult::ok(format!("id: {}, code: {}", 42, code))
}
