mod services;

use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

pub async fn routes(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => services::persist_node(req).await,
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    };
    Ok(response)
}
