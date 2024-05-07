use axum::{extract::Path, routing::get, Router};

// Built on hyper
pub async fn basic() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 7878
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn routing() {
    let app = Router::new()
        .route("/", get(|| async { "Hello root" }))
        .route("/sum/:num1/:num2", get(sum))
        .route(
            "/:num1/times/:num2",
            get(|Path((num1, num2)): Path<(u32, u32)>| async move {
                format!("{} * {} = {}", num1, num2, num1 * num2)
            }),
        );

    // run our app with hyper, listening globally on port 7878
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn sum(Path((num1, num2)): Path<(u32, u32)>) -> String {
    format!("{} + {} = {}", num1, num2, num1 + num2)
}
