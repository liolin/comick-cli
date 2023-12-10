use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use comick;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    comick::cli_entrypoint().await;

    Ok(())
}
