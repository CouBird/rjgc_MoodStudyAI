use backend::{app, config::AppConfig, state::AppState};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), backend::error::AppError> {
    backend::init_tracing();

    let config = AppConfig::from_env()?;
    let address = config.server.address()?;
    let state = AppState::new(config).await?;
    let router = app::router(state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    info!("backend listening on http://{address}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
