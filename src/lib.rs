mod states;

use states::{
    template::{
        AppEngine,
        self,
    },
    statics::StaticFile,
};
use axum::{
    body, extract::Path, http::{
        HeaderMap,
        Request,
        Response
    }, response::IntoResponse, routing::get, Router
};
use tower_service::Service;
use worker::{
    Env,
    event,
    Body,
    Result,
};
use axum_template::{Key, RenderHtml};
use serde_json::json;

#[event(fetch)]
pub async fn main(req: Request<Body>, _env: Env, _ctx: worker::Context) -> Result<Response<body::Body>> {
    console_error_panic_hook::set_once();

    let mut _router = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/asset/:path", get(statics))
        .with_state(template::AppState::new());
    Ok(_router.call(req).await?)
}

pub async fn hello() -> &'static str {
    "Hello Axum!"
}

pub async fn root(
    engine: AppEngine,
    headers: HeaderMap
) -> impl IntoResponse {
    let mut boosted = false;
    if headers.contains_key("Hx-Boosted") {
        boosted = true;
    }
    // get params from request
    let data = json!({
        "title": "Axum",
        "description": "Axum + Handlbars + Cloudflare Workers",
        "boosted": boosted,
        "navbar": [
            {
                "name": "Home",
                "url": "/",
                "active": true
            },
        ]
    });
    // "root.hbs" is the template file name
    let key = Key::from("pages/home.html".to_string());
    RenderHtml(key, engine, data)
}

pub async fn statics(
    Path(path): Path<String>
) -> impl IntoResponse {
    StaticFile(path)
}
