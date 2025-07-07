use actix_web::{get, HttpResponse, Responder};

#[get("/help")]
pub async fn help_h() -> impl Responder {
    HttpResponse::Ok()
        .json(
            serde_json::json!({ "status": "ok" })
        )
}

#[get("/help_me")]
pub async fn help_me() -> impl Responder {
    HttpResponse::Ok()
        .json(
            serde_json::json!({ "status": "ok bro" })
        )
}
