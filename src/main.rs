use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

#[derive(Deserialize, Serialize, Debug)]
struct Node {
    value: String,
    parent: String,
}

async fn persist_node(node: Node) -> Result<impl warp::Reply, Infallible> {
    println!("{:#?}", node);
    Ok(warp::reply())
}

#[tokio::main]
async fn main() {
    let routes = warp::post()
        .and(warp::any())
        .and(warp::body::json())
        .and_then(persist_node);

    println!("Server listening on 127.0.0.1:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
