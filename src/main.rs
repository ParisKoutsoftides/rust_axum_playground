use axum::{
    routing::get,
    Router,
    response::Json,
};

use serde_json::{
    Value,
    json,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/testerino", get(testerino))
        .route("/json", get(json));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn testerino() -> &'static str {
    "Testerino!"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}