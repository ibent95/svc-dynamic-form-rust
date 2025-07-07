use std::str::FromStr;
use actix_web::{post, web::{Payload, Json, Path, Query}, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, de::{self, Deserializer}};

use crate::framework::shared::request_service::{parse_request, parse_as_struct, RequestBody};

// ====== Struktur data ======

#[derive(Deserialize)]
struct MyPath {
    id: String,
}

#[derive(Deserialize)]
struct MyQuery {
    filter: Option<String>,
}

#[derive(Deserialize)]
struct MyJson {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct MyForm {
    name: String,
    #[serde(deserialize_with = "deserialize_u8_from_str")]
    age: u8,
}

fn deserialize_u8_from_str<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u8::from_str(&s).map_err(de::Error::custom)
}

#[post("/handle/json/{id}")]
pub async fn handle_json(
    path: Path<MyPath>,
    query: Query<MyQuery>,
    json: Json<MyJson>,
    req: HttpRequest,
) -> impl Responder {

    let method = req.method().as_str();
    let headers = req.headers();
    let uri = req.uri();

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Received JSON data",
        "path_id": path.id,
        "query_filter": query.filter,
        "title": json.title,
        "content": json.content,
        "method": method,
        "uri": uri.to_string(),
        "headers": headers.iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap_or(""))).collect::<Vec<_>>(),
    }))
}

#[post("/handle/form/{id}")]
pub async fn handle_form(
    req: HttpRequest,
    payload: Payload,
) -> impl Responder {
    let parsed = parse_request(&req, Payload::from(payload)).await;

    let data: MyForm = match parse_as_struct(&parsed) {
        Ok(d) => d,
        Err(err) => return HttpResponse::BadRequest().body(format!("Failed to parse: {}", err)),
    };

    let method = req.method().as_str();
    let headers = req.headers();
    let uri = req.uri();

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Received Form data",
        "name": data.name,
        "age": data.age,
        "method": method,
        "uri": uri.to_string(),
        "headers": headers.iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap_or(""))).collect::<Vec<_>>(),
    }))
}

#[post("/handle/multipart/{id}")]
pub async fn handle_multipart(
    path: Path<MyPath>,
    query: Query<MyQuery>,
    req: HttpRequest,
    payload: Payload,
) -> impl Responder {

    let parsed = parse_request(&req, payload).await;

    let fields = match &parsed {
        RequestBody::Multipart { fields, .. } => fields,
        _ => return HttpResponse::BadRequest().body("Expected multipart/form-data"),
    };

    let data: MyForm = match parse_as_struct(&parsed) {
        Ok(d) => d,
        Err(err) => return HttpResponse::BadRequest().body(format!("Failed to parse: {}", err)),
    };

    let files = match &parsed {
        RequestBody::Multipart { files, .. } => files.clone(),
        _ => vec![],
    };

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Received multipart/form-data",
        "path_id": path.id,
        "query_filter": query.filter,
        "name": data.name,
        "age": data.age,
        "files": files,
        "fields": fields,
        "method": req.method().as_str(),
        "uri": req.uri().to_string(),
        "headers": req.headers().iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap_or(""))).collect::<Vec<_>>(),
    }))
}
