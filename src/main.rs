mod anim;
mod cli;
mod ease;
mod events;

use clap::Parser;
use cli::Args;
use events::run_event_loop;

#[tokio::main]
async fn main() -> swayipc::Fallible<()> {
    let args = Args::parse();
    run_event_loop(args).await
}
