#[macro_use] extern crate rocket;

mod boosted;
mod root;

use rocket_dyn_templates::Template;
use tera::Tera;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root::index,root::chat_room,root::chess_room,root::files])
        // using tera with .html files not .tera
        .attach(Template::custom(|engines| {
            engines.tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tera/**/*.html")) {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
        }))
}