use clap::Parser;
use clap::command;

#[derive(Parser, Debug)]
#[command(version, about = "Summarize directory contents to clipboard")]
pub struct Args {
    /// If set, enable debug logging
    #[arg(long)]
    pub debug: bool,
}
