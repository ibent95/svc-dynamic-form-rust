use clap::{Parser, Subcommand};

pub mod queue;
pub mod migrate;
pub mod watch;
pub mod schedule;
pub mod make;
pub mod seed;
pub mod serve;

#[allow(unused_imports)]
pub mod proxy {
    pub use crate::commands::*;
}

#[derive(Parser)]
#[command(name = "svc", version, about = "Dynamic Form CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Queue,
    Migrate,
    Watch,
    Schedule,
    Make,
    Seed,
    Serve,
}

pub async fn dispatch(cmd: Commands) {
    match cmd {
        Commands::Queue => queue::run().await,
        Commands::Migrate => migrate::run().await,
        Commands::Watch => watch::run().await,
        Commands::Schedule => schedule::run().await,
        Commands::Make => make::run().await,
        Commands::Seed => seed::run().await,
        Commands::Serve => serve::run().await,
    }
}