use actix_web::{App, HttpServer};
use std::env;
use dotenvy::dotenv;

pub async fn run() {
    println!("Service svc-dynamic-form-rust");

    println!("🚀 Run the server...");

    dotenv().ok();
    let db = crate::configs::database::establish_connection().await;

    let url = env::var("APP_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "7500".to_string());

    println!("🌐 Rust service running at http://{url}:{port}");

    if std::env::var("CARGO_WATCH_STARTED").is_ok() {
        println!("Running under cargo-watch.");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .configure(crate::configs::routes::config)
    })
    .bind((url, port.parse().unwrap()))
    .expect("Failed to bind the server")
    .run()
    .await
    .expect("Server error");
}
