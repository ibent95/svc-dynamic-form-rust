use actix_web::{route, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PublicationController {
    results: String,
}

#[route("/publication", method = "GET")]
pub async fn index(request: HttpRequest) -> HttpResponse {
    let request_query_params = request.query_string();
    let request_headers: Vec<_> = request
        .headers()
        .iter()
        .map(|(key, value)| {
            (
                key.as_str(),
                value.to_str()
                    .unwrap_or("<invalid UTF-8>")
                    .to_string(),
            )
        })
        .collect();
    let request_method = request.method().as_str();
    let request_uri = request.uri().to_string();
    let request_url = request.full_url().to_string();

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Publication controller for index is working.",
        "query_params": request_query_params,
        "headers": request_headers,
        "method": request_method,
        "uri": request_uri,
        "url": request_url,
    }))
}

#[route("/publication/details", method = "GET")]
pub async fn details(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Publication controller for details is working."
    }))
}

#[route("/publication", method = "POST")]
pub async fn create(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Publication controller for create is working."
    }))
}

#[route("/publication", method = "PUT")]
pub async fn update(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Publication controller for update is working."
    }))
}

#[route("/publication", method = "DELETE")]
pub async fn delete(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Publication controller for delete is working."
    }))
}
