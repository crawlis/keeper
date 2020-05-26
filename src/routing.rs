use super::persistence::database::Database;
use super::persistence::models::NewNode;
use hyper::{Body, Method, Request, Response, StatusCode};

pub async fn routes(
    req: Request<Body>,
    database: &Database,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            let mut response = Response::new(Body::empty());
            let body = hyper::body::to_bytes(req.into_body()).await?;
            if let Ok(node) = serde_json::from_slice(&body) as Result<NewNode, _> {
                if let Ok(node) = database.insert(node).await {
                    if let Ok(node) = serde_json::to_string(&node) {
                        *response.status_mut() = StatusCode::OK;
                        *response.body_mut() = Body::from(node);
                        return Ok(response);
                    }
                }
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            } else {
                *response.status_mut() = StatusCode::BAD_REQUEST;
            }
            Ok(response)
        }
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}
