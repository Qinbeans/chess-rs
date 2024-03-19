use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;

// A simple struct that represents if a request is boosted by HTMX
pub struct HxBoosted(bool);

impl HxBoosted {
    pub fn is_boosted(&self) -> bool {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HxBoosted {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("Hx-Boosted") {
            Some("true") => Outcome::Success(HxBoosted(true)),
            _ => Outcome::Success(HxBoosted(false)), // Not boosted or header is missing
        }
    }
}
