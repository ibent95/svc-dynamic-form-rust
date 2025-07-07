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

use clap::{Parser, Subcommand};
use std::fmt;

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
    Make {
        #[command(subcommand)]
        kind: MakeKind,
    },
    Seed,
    Serve,
}

#[derive(Subcommand)]
pub enum MakeKind {
    Controller { name: String },
    Enum { name: String },
    Service { name: String },
    Repository { name: String },
    Model { name: String },
    Module { name: String },
    Command { name: String },
    Middleware { name: String },
    Request { name: String },
    ValueObject { name: String },
}

impl fmt::Display for MakeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            MakeKind::Controller { .. } => "controller",
            MakeKind::Enum { .. } => "enum",
            MakeKind::Service { .. } => "service",
            MakeKind::Repository { .. } => "repository",
            MakeKind::Model { .. } => "model",
            MakeKind::Module { .. } => "module",
            MakeKind::Command { .. } => "command",
            MakeKind::Middleware { .. } => "middleware",
            MakeKind::Request { .. } => "request",
            MakeKind::ValueObject { .. } => "value_object",
        };
        write!(f, "{}", label)
    }
}

pub async fn dispatch(cmd: Commands) {
    match cmd {
        Commands::Queue => queue::run().await,
        Commands::Migrate => migrate::run().await,
        Commands::Watch => watch::run().await,
        Commands::Schedule => schedule::run().await,
        Commands::Make { kind } => make::run(kind).await,
        Commands::Seed => seed::run().await,
        Commands::Serve => serve::run().await,
    }
}