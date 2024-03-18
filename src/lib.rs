mod template;

use crate::template::{
    AppState,
    AppEngine
};
use axum::{
    extract::Query,
    routing::get,
    Router,
    response::{
        IntoResponse,
        Response
    },
};
use std::collections::HashMap;
use tower_service::Service;
use worker::*;
use axum_template::{Key, RenderHtml};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RootPage {
    title: String,
    description: String,
}

fn router() -> Router {
    // add template renderer
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        // send with header: text/css
        .route("/app.css", get(get_style))
        .with_state(AppState::new())
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}

pub async fn hello() -> &'static str {
    "Hello Axum!"
}

pub async fn root(
    engine: AppEngine,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // get params from request
    let mut data = RootPage {
        title: "Axum".to_string(),
        description: "Axum + Handlbars + Cloudflare Workers".to_string(),
    };
    if params.contains_key("no-header") {
        data.description += " boosted";
        return RenderHtml(Key::from("no-headers/root".to_string()), engine, data);
    }
    // "root.hbs" is the template file name
    let key = Key::from("root".to_string());
    RenderHtml(key, engine, data)
}

pub async fn get_style() -> Response {
    let app_css = include_str!("../public/app.css");
    Response::builder()
        .status(200)
        .header("content-type", "text/css")
        .body(app_css.into())
        .unwrap()
}