//! Template microservice Rust dengan Actix-Web + SeaORM

mod configs;
mod services;
mod models;
mod commands;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use env_logger;
use std::{cmp::Ordering, env, io};
use rand::Rng;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("New svc-dynamic-form-rust");

    dotenv().ok();
    env_logger::init();

    let db = configs::database::establish_connection().await;

    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "7000".to_string());
    println!("🚀 Server running at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(
                actix_web::web::Data::new(
                    db.clone()
                )
            )
            .configure(configs::routes::config)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}

async fn guest_game() {

    println!("Guess the number!");

    let secret_number = rand::rng()
        .random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

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
