use clap::{Parser, Subcommand};

pub mod kafka;

#[derive(Parser)]
#[command(name = "svc", version, about = "Dynamic Form CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Kafka,
}

pub async fn dispatch(cmd: Commands) {
    match cmd {
        Commands::Kafka => kafka::run().await,
    }
}