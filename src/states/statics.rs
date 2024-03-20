use axum::{http::{header, StatusCode}, response::{IntoResponse, Response}};
use rust_embed::RustEmbed;

// Statics
#[derive(RustEmbed)]
#[folder = "static/"]
pub struct Statics;

pub struct StaticFile<T>(pub T);


impl<T> IntoResponse for StaticFile<T>
where
  T: Into<String>,
{
  fn into_response(self) -> Response {
    let path = self.0.into();

    match Statics::get(path.as_str()) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
  }
}