//! Goki entrypoint

use anyhow::Result;

/// Entrypoint to the CLI.
#[tokio::main]
async fn main() -> Result<()> {
    goki::cli::run().await?;
    Ok(())
}
