use compass::{init_router, Config};
use std::error::Error;
use tokio::net::TcpListener;
use tracing::info;
use tracing_appender::{non_blocking, rolling};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;
    let (logger, _guard) = non_blocking(rolling::daily(&config.log_dir, "app.log"));
    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .with_ansi(false)
        .with_writer(logger)
        .json()
        .init();

    let listener = TcpListener::bind(&config.addr).await?;
    let router = init_router(&config).await?;
    info!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}
