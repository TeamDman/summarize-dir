use args::Args;
use clap::Parser;
use tracing::{debug, info};

pub mod args;
pub mod dir_walker;
pub mod init;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();
    let filter = if args.debug {
        tracing::level_filters::LevelFilter::DEBUG
    } else {
        tracing::level_filters::LevelFilter::INFO
    };
    init::init(filter)?;

    info!("Hello, world!");
    debug!("Hello, debug!");
    Ok(())
}
