pub mod commands;
pub mod controllers;
pub mod enums;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod services;

#[path = "../framework"]
pub mod framework {

    #[path = "services/shared/mod.rs"]
    pub mod shared;

    pub use shared::*;

}
