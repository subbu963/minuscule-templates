use minuscule_types::{request::Request, response::Response};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[derive(Serialize, Deserialize, Debug)]
struct AddQs {
    a: isize,
    b: isize,
}
#[wasmedge_bindgen]
pub fn add(request_string: String) -> String {
    let request: Request = Request::from_str(request_string);
    let query_params: AddQs = request.get_query().deserialize().unwrap();
    let response = Response {
        headers: None,
        status: Some(200),
        body: json!({"answer": _add(query_params.a, query_params.b)}).to_string(),
    };
    return Response::to_str(&response);
}

pub fn _add(left: isize, right: isize) -> isize {
    left + right
}
