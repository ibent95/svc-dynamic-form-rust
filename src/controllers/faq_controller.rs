use actix_web::{get, HttpResponse, Responder};

#[get("/faq")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .json(
            serde_json::json!({ "status": "ok" })
        )
}
