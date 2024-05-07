use tokio::signal;

mod router;

const PORT: &str = "7878";

pub async fn start_server() {
    println!("Starting...");

    let router = router::get_router();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await
        .unwrap();

    println!("Goodbye");
}

async fn shutdown() {
    println!("Server listening on {PORT}");

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            println!("\nctrl + c pressed. Shutting down")
        },
        _ = terminate => {
            println!("Terminated")
        },
    }
}
