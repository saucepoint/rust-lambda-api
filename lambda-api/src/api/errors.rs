use hyper::StatusCode;
use lambda_http::{Error, Response, Body};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Default)]
struct GenericError {
    #[serde(default)]
    message: String,
}

// For response bodies to serialize correctly, you should implement 
// `into_response(self) -> Response<Body>`
impl GenericError {
    fn into_response(self, status_code: StatusCode) -> Response<Body> {
        let s = serde_json::to_string_pretty(&self).unwrap();
        let resp = Body::from(s);
        let resp = Response::builder()
            .status(status_code)
            .header("content-type", "text/html")
            .body(resp)
            .map_err(Box::new).unwrap();
        resp
    }
}

pub fn handle_404() -> Result<Response<Body>, Error> {
    let resp = GenericError {
        message: "Not Found".to_string(),
    };
    Ok(resp.into_response(StatusCode::NOT_FOUND))
}

pub fn handle_405() -> Result<Response<Body>, Error> {
    let resp = GenericError {
        message: "Unsupported HTTP Method".to_string(),
    };
    Ok(resp.into_response(StatusCode::METHOD_NOT_ALLOWED))
}

// Not really an error, but putting it here to keep the example code cleaner
pub fn handle_welcome() -> Result<Response<Body>, Error> {
    let resp = GenericError {
        message: "Welcome to the rust-lambda-api".to_string(),
    };
    Ok(resp.into_response(StatusCode::OK))
}
