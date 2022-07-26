use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lambda_http::request::{RequestContext};
use hyper::Method;
mod api;

fn resolve_routes(method: Method, route: &str, event: Request) -> Result<Response<Body>, Error> {
    println!("Resolving Route: {:?}", route);

    // route to function handlers based on the method and route
    match route {
        "/hello" => {
            match method {
                Method::POST => api::hello::post(event),
                Method::GET => api::hello::get(event),
                _ => api::errors::handle_405(),
            }
        }
        // add additional routes here
        // "/foo" => {
        //     match method {
        //         Method::POST => api::foo::post(event),
        //         Method::GET => api::foo::get(event),
        //         _ => api::errors::handle_405(),
        //     }
        // }
        "/" => {
            match method {
                Method::GET => api::errors::handle_welcome(),
                _ => api::errors::handle_405(),
            }
        }
        _ => {
            api::errors::handle_404()
        }
    }
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let ctx: RequestContext = event.request_context();
    let resp = match ctx {
        RequestContext::ApiGatewayV2(ref c) => {
            match &c.http.path {
                Some(path) => {
                    let mut route: &str = path.as_str();
                    if route.len() >= 2 {
                        // local execution with `cargo lambda watch` is prepending
                        // an extra `/` to the path. this logic strips off a prepended `/`
                        // if the first two characters in the path are `//`
                        route = match &path[..2] {
                            "//" => &path[1..],
                            _ => &path,
                        };
                    }

                    resolve_routes(c.http.method.clone(), route, event)
                }
                None => {
                    api::errors::handle_404()
                }
            }
        }
    }.unwrap();

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
