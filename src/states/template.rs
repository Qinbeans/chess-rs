use tera::Tera;
use axum_template::engine::Engine;
use axum::extract::FromRef;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "tera/"]
pub struct Templates;

pub type AppEngine = Engine<Tera>;

#[derive(Clone, FromRef)]
pub struct AppState {
    engine: AppEngine,
}

impl AppState {
    pub fn new() -> Self {
        // load templates from Templates struct
        let mut tera = Tera::default();
        // load layout.html and intermediate.html
        let content = Templates::get("layout.html").unwrap().data.into_owned().iter().map(|u| *u as char).collect::<String>();
        tera.add_raw_template("layout.html", &content).unwrap();
        let content = Templates::get("intermediate.html").unwrap().data.into_owned().iter().map(|u| *u as char).collect::<String>();
        tera.add_raw_template("intermediate.html", &content).unwrap();
        for name in Templates::iter() {
            // name is a cow
            let name = name.into_owned();
            if name == "intermediate.html" && name == "layout.html" {
                continue;
            }
            let content = Templates::get(&name).unwrap().data.into_owned().iter().map(|u| *u as char).collect::<String>();
            tera.add_raw_template(&name, &content).unwrap();
        }
        let engine = Engine::new(tera);
        Self {
            engine
        }
    }
}