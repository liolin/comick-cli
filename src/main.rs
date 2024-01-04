use anyhow::Result;
use comick;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;


#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(max_log_level())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    comick::cli_entrypoint().await;

    Ok(())
}

fn max_log_level() -> Level {
    match std::env::var("LOG").unwrap_or_default().as_str() {
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        _ => Level::INFO
    }
}
