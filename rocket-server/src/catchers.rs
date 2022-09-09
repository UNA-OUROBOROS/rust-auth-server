use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> Value {
    json!({
        "result": "failed",
        "details": {
            "code": 404,
            "message": format!("'{}' is not a valid path.", req.uri())
        }
    })
}

#[catch(400)]
pub fn bad_request(req: &Request) -> Value {
    json!({
        "result": "failed",
        "details": {
            "code": 400,
            "message": format!("'{}' is not a valid request.", req.uri()),
        }
    })
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Value {
    json!({
        "result": "failed",
        "details": {
            "code": 422,
            "message": format!("'{}' contains semantic errors.", req.uri()),
        }
    })
}
