use serde::de::DeserializeOwned;
use actix_multipart::Multipart;
use actix_web::{
    web::{BytesMut, Payload},
    HttpRequest,
};
use futures_util::StreamExt;
use serde_json::Value;
use std::{collections::HashMap, fs, io::Write, path::PathBuf};
use serde::de::Error as SerdeDeError;

#[derive(Debug, Default)]
pub enum RequestBody {
    Json(Value),
    Form(HashMap<String, String>),
    Multipart {
        fields: HashMap<String, String>,
        files: Vec<String>,
    },
    #[default]
    Empty,
}

pub async fn parse_request(req: &HttpRequest, mut payload: Payload) -> RequestBody {
    let content_type = req
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    // application/json
    if content_type.starts_with("application/json") {
        let mut body = BytesMut::new();
        while let Some(chunk) = payload.next().await {
            if let Ok(bytes) = chunk {
                body.extend_from_slice(&bytes);
            }
        }

        let json: Option<Value> = serde_json::from_slice(&body).ok();
        return json.map(RequestBody::Json).unwrap_or_default();
    }

    // application/x-www-form-urlencoded
    if content_type.starts_with("application/x-www-form-urlencoded") {
        let mut body = BytesMut::new();
        while let Some(chunk) = payload.next().await {
            if let Ok(bytes) = chunk {
                body.extend_from_slice(&bytes);
            }
        }

        let form_str = String::from_utf8_lossy(&body);
        let form: HashMap<String, String> = form_str
            .split('&')
            .filter_map(|pair| {
                let mut split = pair.splitn(2, '=');
                Some((
                    split.next()?.to_string(),
                    split.next().unwrap_or("").to_string(),
                ))
            })
            .collect();

        return RequestBody::Form(form);
    }

    // multipart/form-data
    if content_type.starts_with("multipart/form-data") {
        let mut multipart = Multipart::new(req.headers(), payload.into_inner());
        let mut fields = HashMap::new();
        let mut uploaded_files = Vec::new();

        while let Some(Ok(mut field)) = multipart.next().await {
            let name = field.name().unwrap_or_default().to_string();
            let content_disposition = field.content_disposition();

            if let Some(filename) = content_disposition.and_then(|cd| cd.get_filename()) {
                let safe_filename = sanitize_filename::sanitize(&filename);
                let filepath: PathBuf = format!("./tmp/{}", safe_filename).into();

                fs::create_dir_all("./tmp").ok();
                let mut f = fs::File::create(&filepath).unwrap();

                while let Some(Ok(chunk)) = field.next().await {
                    f.write_all(&chunk).unwrap();
                }

                uploaded_files.push(filepath.to_string_lossy().to_string());
            } else {
                let mut value = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    value.extend_from_slice(&chunk);
                }

                let text = String::from_utf8_lossy(&value).to_string();
                fields.insert(name, text);
            }
        }

        return RequestBody::Multipart {
            fields,
            files: uploaded_files,
        };
    }

    RequestBody::Empty
}

pub fn parse_as_struct<T: DeserializeOwned>(body: &RequestBody) -> Result<T, serde_json::Error> {
    match body {
        RequestBody::Json(value) => serde_json::from_value(value.clone()),
        RequestBody::Form(fields) => {
            let map: serde_json::Map<String, serde_json::Value> = fields
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            serde_json::from_value(serde_json::Value::Object(map))
        }
        RequestBody::Multipart { fields, .. } => {
            let map: serde_json::Map<String, serde_json::Value> = fields
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            serde_json::from_value(serde_json::Value::Object(map))
        }
        _ => Err(serde_json::Error::custom("Unsupported request body")),
    }
}
