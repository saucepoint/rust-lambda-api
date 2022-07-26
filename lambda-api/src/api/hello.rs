use lambda_http::{Error, Request, RequestExt, Response, Body};
use serde::{Serialize, Deserialize};
use serde_json;

// Utilized as both input requests (query params & request body) as well as response body.
// You will likely need to define separate structs for request and response.
#[derive(Debug, Serialize, Deserialize, Default)]
struct HelloRequest {
    #[serde(default)]
    name: String,
    #[serde(default)]
    number: i64
}

// For response bodies to serialize correctly, you should implement 
// `into_response(self) -> Response<Body>`
impl HelloRequest {
    fn into_response(self) -> Response<Body> {
        let s = serde_json::to_string_pretty(&self).unwrap();
        let resp = Body::from(s);
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(resp)
            .map_err(Box::new).unwrap();
        resp
    }
}

pub fn get(event: Request) -> Result<Response<Body>, Error> {
    let args = event.query_string_parameters();
    let data = HelloRequest {
        name: args.first("name").unwrap_or("World").to_string(),
        number: args.first("number").unwrap_or("0").parse::<i64>().unwrap_or(0)
    };
    // ...
    // do stuff with query params
    // ...

    Ok(data.into_response())
}

pub fn post(event: Request) -> Result<Response<Body>, Error> {
    let args: HelloRequest = event.payload().unwrap_or_else(|_parse_error| None).unwrap_or_default();
    // ...
    // do stuff with request body
    // ...

    Ok(args.into_response())
}
