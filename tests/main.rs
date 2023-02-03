use actix_api_response::ApiResponse;
use actix_web::Responder;
use serde::Serialize;

#[derive(ApiResponse)]
struct TestStruct {
    a: i32,
    b: i32
}

#[test]
fn serialize_basic_struct(){}
