use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

#[derive(Deserialize, Serialize, Debug)]
struct CrawlingOutput {
    parent: String,
    childs: Vec<String>,
}

async fn persist_crawling_output(
    crawling_output: CrawlingOutput,
) -> Result<impl warp::Reply, Infallible> {
    println!("{:#?}", crawling_output);
    Ok(warp::reply())
}

#[tokio::main]
async fn main() {
    let routes = warp::post()
        .and(warp::any())
        .and(warp::body::json())
        .and_then(persist_crawling_output);

    println!("Server listening on 127.0.0.1:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
