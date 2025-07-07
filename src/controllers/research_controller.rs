use actix_web::{get, HttpResponse, Responder};

#[get("/research")]
pub async fn research() -> impl Responder {
    HttpResponse::Ok()
        .json(
            serde_json::json!({ "status": "ok" })
        )
}
