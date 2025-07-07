#[path = "../configs/mod.rs"]
mod configs;

mod commands;
mod controllers;
mod middlewares;
mod models;
mod repositories;
mod services;

use actix_web::{App, HttpServer};
use clap::Parser;
use dotenvy::dotenv;
use env_logger;
use std::{cmp::Ordering, env, io};
use rand::Rng;
use crate::commands::Cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    // Cek: jika env CARGO_WATCH_STARTED ada, abaikan subcommand run-watch
    let is_watch = std::env::args().any(|a| a == "run-watch");
    let is_cargo_watch = std::env::var("CARGO_WATCH_STARTED").is_ok();

    if is_watch && !is_cargo_watch {
        // Hanya parent process yang boleh jalankan cargo-watch
        commands::watch::run().await;
        return Ok(());
    }

    let cli = Cli::parse();

    match cli.command {

        Some(cmd) => {
            commands::dispatch(cmd).await;
            Ok(())
        }

        None => {
            println!("Service svc-dynamic-form-rust");

            let db = configs::database::establish_connection().await;

            let url = env::var("APP_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = env::var("APP_PORT").unwrap_or_else(|_| "7500".to_string());

            println!("🚀 Rust service running at http://{url}:{port}");

            HttpServer::new(move || {
                App::new()
                    .app_data(actix_web::web::Data::new(db.clone()))
                    .configure(configs::routes::config)
            })
            .bind((url, port.parse().unwrap()))?
            .run()
            .await
        }

    }
}

pub fn guest_game() {
    println!("🎮 Welcome to the Guessing Game!");
    let secret_number = rand::rng()
        .random_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
