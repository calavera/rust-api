use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    let res = std::fs::create_dir("./cache");
    if let Err(err) = res {
        println!("error creating directories: {}", err);
    }
    let res = std::fs::write("./cache/foo.txt", "cache file");
    if let Err(err) = res {
        println!("error writing file: {}", err);
    }
    "Hello, from a Rust server!"
}
