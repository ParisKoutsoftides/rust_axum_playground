use axum::{
    routing::get,
    Router,
    response::Json,
};

use serde_json::{
    Value,
    json,
};

use axum::extract::{
    Query,
};

use std::collections::HashMap;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/testerino", get(testerino))
        .route("/json", get(json))
        .route("/query", get(query));

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

async fn query(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let param_testerino = params.get("paramerino").cloned().unwrap_or(String::from("empty"));
    Json(json!({ "data": param_testerino }))
}