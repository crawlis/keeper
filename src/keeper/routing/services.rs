use hyper::{Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Node {
    parent: String,
    children: Vec<String>,
}

pub async fn persist_node(req: Request<Body>) -> Response<Body> {
    let status_code: StatusCode;
    if let Ok(body) = hyper::body::to_bytes(req.into_body()).await {
        if let Ok(_node) = serde_json::from_slice(&body) as Result<Node, _> {
            println!("Received data !");
            status_code = StatusCode::OK;
        } else {
            status_code = StatusCode::BAD_REQUEST;
        }
    } else {
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}
