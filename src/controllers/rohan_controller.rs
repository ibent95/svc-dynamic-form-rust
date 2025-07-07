use actix_web::{get, HttpResponse, Responder};

#[get("/rohan")]
pub async fn rohan() -> impl Responder {
    HttpResponse::Ok()
        .json(
            serde_json::json!({ "status": "ok" })
        )
}
