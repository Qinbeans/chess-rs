use axum::{
    extract::FromRef,
    routing::get,
    Router,
    response::IntoResponse,
};
use tower_service::Service;
use handlebars::Handlebars;
use worker::*;
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::Serialize;

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[derive(Serialize, Debug)]
pub struct RootPage {
    title: String,
    description: String,
}

fn router() -> Router {
    // add template renderer
    let mut handlebars = Handlebars::new();
    // render from files
    let template_str = include_str!("templates/root.hbs");
    handlebars.register_template_string("root", template_str).unwrap();
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .with_state(AppState {
            engine: Engine::from(handlebars),
        })
        
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

pub async fn root(engine: AppEngine) -> impl IntoResponse {
    let data = RootPage {
        title: "Axum".to_string(),
        description: "Axum + Handlbars + Cloudflare Workers".to_string(),
    };
    // "root.hbs" is the template file name
    let key = Key::from("root".to_string());
    RenderHtml(key, engine, data)
}