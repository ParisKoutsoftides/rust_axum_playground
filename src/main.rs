use axum::{
    routing::{get, post},
    Router,
    response::Json,
    extract,
    extract::{Query}
};
use serde_json::{
    Value,
    json,
};
use std::collections::HashMap;
use serde::Deserialize;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt as tracing_sub;

#[derive(Deserialize)]
struct Message {
    text: String,
}

const CONFIG_KEY: &str = "configKey";
const EMPTY_STR: &str = "empty";

#[tokio::main]
async fn main() {
    tracing_sub::init();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/testerino", get(testerino).post(testerino_post))
        .route("/json", get(json))
        .route("/query", get(query))
        .route("/get_config", get(get_config))
        .route("/posterino_string", post(posterino_string))
        .route("/posterino_json", post(posterino_json))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        );

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

async fn testerino_post(extract::Json(message): Json<Message>) -> String {
    format!("{}", message.text)
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn query(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let param_testerino = params.get("paramerino").cloned().unwrap_or(String::from(EMPTY_STR));
    Json(json!({ "data": param_testerino }))
}

async fn get_config(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let mut config_map = HashMap::new();
    config_map.insert(
        "configKey1".to_string(),
        "configValue1".to_string(),
    );

    config_map.insert(
        "configKey2".to_string(),
        "configValue2".to_string(),
    );

    let config_key = params.get(CONFIG_KEY).cloned().unwrap_or(String::from(EMPTY_STR));
    let config_value = config_map.get(&config_key).cloned().unwrap_or(String::from(EMPTY_STR));
    Json(json!({ config_key: config_value }))
}

async fn posterino_string(extract::Json(message): Json<Message>) -> String {
    format!("Message: {}", message.text)
}

async fn posterino_json(extract::Json(message): Json<Message>) -> Json<Value> {
    Json(json!({ "message": message.text }))
}
