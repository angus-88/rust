// Comparing Rust web frameworks
// Axum
// - >10m downloads recently
// - last update: 1 month

// Rocket - going to ignore due to it's relative unpopularity and slow release cadence
// - 0.5m recently
// - last update: 5 months

// Actix-web
// - >2.5m recently
// - last updated: 3 months ago

// Warp
// - >2m recently
// - last updated 20 days ago

use actix_web::{App, HttpServer};

mod actix_web_demo;
mod axum;
mod warp;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Warp
    // warp::basic().await;
    // warp::routing().await;

    // Axum
    // axum::basic().await;
    // axum::routing().await;

    // Actix-web
    HttpServer::new(|| {
        App::new()
            .service(actix_web_demo::basic)
            .service(actix_web_demo::routes)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
