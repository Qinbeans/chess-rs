use handlebars::Handlebars;
use axum_template::engine::Engine;
use axum::extract::FromRef;

pub type AppEngine = Engine<Handlebars<'static>>;

#[derive(Clone, FromRef)]
pub struct AppState {
    engine: AppEngine,
}

impl AppState {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        // render from files
        let root_tmpl = include_str!("../../templates/root.hbs");
        let root_nh_tmpl = include_str!("../../templates/no-headers/root-nh.hbs");
        handlebars.register_template_string("root", root_tmpl).unwrap();
        handlebars.register_template_string("no-headers/root", root_nh_tmpl).unwrap();

        Self {
            engine: Engine::from(handlebars),
        }
    }
}