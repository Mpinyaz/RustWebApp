use axum::extract::{Path, Query, State};
use axum::response::{Html, IntoResponse, Response};
use axum::{http::StatusCode, Json};

use crate::models::templates::{HomeTemplate, HtmlTemplate, NameTemplate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub name: String,
    pub age: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub name: Option<String>,
    pub age: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    pub greeting: String,
}

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {
        title: "🚀 Mpinyaz".to_string(),
    };
    HtmlTemplate(template)
}

pub async fn name_handler(Path(name): Path<String>) -> impl IntoResponse {
    let template = NameTemplate { name };
    HtmlTemplate(template)
}

pub async fn json_handler(Json(data): Json<RequestData>) -> Json<ResponseData> {
    let greeting = format!("Hello, {} year old named {}!", data.age, data.name);
    let response_data = ResponseData { greeting };
    Json(response_data)
}

pub async fn greeting_handler(Query(params): Query<QueryData>) -> String {
    match (params.name, params.age) {
        (Some(name), Some(age)) => format!("Hello, {} year old named {}!", age, name),
        (Some(name), None) => format!("Hello, named {}!", name),
        (_, _) => "Hello, World!".to_string(),
    }
}
