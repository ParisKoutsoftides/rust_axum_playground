use axum::{
    routing::{get, post, delete, put},
    Router,
    response::Json,
    extract,
    extract::{Query},
    http::StatusCode
};
use serde_json::{
    Value,
    json,
};
use std::collections::HashMap;
use serde::Deserialize;
use tower_http::classify::{SharedClassifier, StatusInRangeAsFailures, StatusInRangeFailureClass};
use tower_http::trace::{
    self,
    TraceLayer
};
use tracing::{
    Level,
    error
};
use tracing_subscriber::fmt as tracing_sub;
use std::time::Duration;

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
        .route("/dummy_delete", delete(dummy_delete))
        .route("/dummy_put", put(dummy_put))
        .route("/crud_user", get(get_user).post(post_user).put(put_user).delete(delete_user))
        .layer(
            TraceLayer::new(SharedClassifier::new(StatusInRangeAsFailures::new(400..=599)))
                .on_failure(|failure, _: Duration, _: &tracing::Span| {
                    match failure {
                        StatusInRangeFailureClass::StatusCode(status) => {
                            error!("http_request_trace error: {status}");
                        }
                        StatusInRangeFailureClass::Error(_err) => {
                            //custom errors
                        }
                    }
                })
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Result<&'static str, StatusCode> {
    Ok("Hello, World!")
}

async fn testerino() -> Result<&'static str, StatusCode> {
    Ok("Testerino!")
}

async fn testerino_post(extract::Json(message): Json<Message>) -> Result<String, StatusCode> {
    Ok(format!("{}", message.text))
}

async fn json() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "data": 42 })))
}

async fn query(Query(params): Query<HashMap<String, String>>) -> Result<Json<Value>, StatusCode> {
    let param_testerino = params.get("paramerino").cloned().unwrap_or(String::from(EMPTY_STR));
    Ok(Json(json!({ "data": param_testerino })))
}

async fn get_config(Query(params): Query<HashMap<String, String>>) -> Result<Json<Value>, StatusCode> {
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
    Ok(Json(json!({ config_key: config_value })))
}

async fn posterino_string(extract::Json(message): Json<Message>) -> Result<String, StatusCode> {
    Ok(format!("Message: {}", message.text))
}

async fn posterino_json(extract::Json(message): Json<Message>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "message": message.text })))
}

async fn dummy_delete() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "deleted": true })))
}

async fn dummy_put() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "new_created": true })))
}

async fn get_user() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "user_retrieved": true })))
}

async fn post_user() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "user_updated": true })))
}

async fn put_user() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "user_created": true })))
}

async fn delete_user() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "user_deleted": true })))
}