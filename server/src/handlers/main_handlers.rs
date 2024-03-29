use axum::extract::{Path, Query,Extension, State};
use axum::body::Body as AxumBody;
use axum::response::{IntoResponse, Response};
use axum::{http::Request, Json};
use leptos::*;
use leptos_axum::handle_server_fns_with_context;
use crate::models::templates::{HomeTemplate, HtmlTemplate, NameTemplate};
use serde::{Deserialize, Serialize};
use crate::models::state::AppState;
use app::*;
use std::sync::Arc;

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

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE

    });

    Json(json_response)
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_async_with_context(

        app_state.leptos.clone(),
        move || {
            provide_context(app_state.pool.clone());
        },
        move || view! { <App /> },
    );
    handler(req).await.into_response()
}

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    Extension(options): Extension<Arc<LeptosOptions>>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.pool.clone());
            provide_context(app_state.leptos.clone());
            provide_context((*options).clone());
        },
        request,
    )
    .await
}
