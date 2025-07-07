use std::{env, process::Command};

pub async fn run() {
    // Jalankan `cargo-watch`, hanya sekali dari proses utama
    let is_watch = env::var("CARGO_WATCH_STARTED").is_ok();

    if is_watch {
        println!("✅ Detected as child process from cargo-watch, skipping re-entry...");
        return;
    }

    // Cek apakah cargo-watch sudah terinstall
    let check = Command::new("cargo-watch").arg("--version").output();

    if check.is_err() {
        println!("📦 `cargo-watch` is not installed, trying to install it...");

        let install = Command::new("cargo")
            .args(&["install", "cargo-watch"])
            .status()
            .expect("Failed to run cargo install");

        if !install.success() {
            eprintln!("❌ Failed to install cargo-watch");
            std::process::exit(1);
        }
    }

    println!("▶️ Starting cargo-watch to monitor and run `cargo run -- serve`");

    let status = Command::new("cargo")
        .args(&["watch", "-w", "src", "-w", "configs", "-s", "./target/debug/svc-dynamic-form-rust serve"])
        .status()
        .expect("Failed to run the cargo-watch.");

    if !status.success() {
        eprintln!("❌ cargo-watch failed to run.");
        std::process::exit(1);
    }

}
