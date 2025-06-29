
use actix_web::{HttpResponse, Responder};

pub fn http_response() -> impl Responder {
    HttpResponse::Ok()
		.json(
			serde_json::json!({ "status": "ok" })
		)
}
