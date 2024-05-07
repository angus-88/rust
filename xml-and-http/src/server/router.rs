use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

pub fn get_router() -> Router {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/about", get_about_routes())
        .route("/xml-to-json", post(post_xml_handler).get(get_xml_handler));

    router
}

fn get_about_routes() -> Router {
    let about_routes = Router::new()
        .route("/", get(get_about_handler))
        .route("/health", get(get_health_handler));

    about_routes
}

async fn get_about_handler() -> &'static str {
    "This server accepts XML data POSTed to /xml-to-json"
}

async fn get_health_handler() -> &'static str {
    "Service is healthy!"
}

async fn get_xml_handler() -> (StatusCode, String) {
    (
        StatusCode::BAD_REQUEST,
        String::from("Try a POST request instead"),
    )
}

async fn post_xml_handler() -> &'static str {
    "XML accepted"
}
