use crate::boosted;
use rocket::{
    serde::json::json,
    fs::NamedFile,
    response::status::NotFound
};
use std::path::{PathBuf, Path};
use rocket_dyn_templates::Template;

#[get("/")]
pub fn index(hx_boosted: boosted::HxBoosted) -> Template {
    let context = json!({
        "title": "HTMX Example",
        "description": "This is an example of HTMX in action.",
        "boosted": hx_boosted.is_boosted(),
        "items": [
            { "name": "Home", "url": "/", "active": true},
            { "name": "Chat", "url": "/chat_menu", "active": false},
            { "name": "Chess", "url": "/chess_menu", "active": false}
        ]
    });
    Template::render("pages/home.html", &context)
}

#[get("/chat_menu")]
pub fn chat_room(hx_boosted: boosted::HxBoosted) -> Template {
    let context = json!({
        "title": "Chat Room",
        "description": "This is an example of HTMX in action.",
        "boosted": hx_boosted.is_boosted(),
        "items": [
            { "name": "Home", "url": "/", "active": false},
            { "name": "Chat", "url": "/chat_menu", "active": true},
            { "name": "Chess", "url": "/chess_menu", "active": false}
        ]
    });
    Template::render("pages/chat_menu.html", &context)
}

#[get("/chess_menu")]
pub fn chess_room(hx_boosted: boosted::HxBoosted) -> Template {
    let context = json!({
        "title": "Chess Room",
        "description": "This is an example of HTMX in action.",
        "boosted": hx_boosted.is_boosted(),
        "items": [
            { "name": "Home", "url": "/", "active": false},
            { "name": "Chat", "url": "/chat_menu", "active": false},
            { "name": "Chess", "url": "/chess_menu", "active": true}
        ]
    });
    Template::render("pages/chess_menu.html", &context)
}

#[get("/asset/<file..>")]
pub async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}