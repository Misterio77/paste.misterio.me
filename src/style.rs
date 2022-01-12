use rocket::http::ContentType;
use rocket::response::{self, Responder};
use rocket::Request;
use rocket::Response;

pub struct StyleSheet {
    inner: &'static str,
    cache_max_age: i32,
}

impl<'r> Responder<'r, 'static> for &StyleSheet {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let cache_control = format!("max-age={}", self.cache_max_age);
        Response::build_from(self.inner.to_owned().respond_to(req)?)
            .header(ContentType::CSS)
            .raw_header("Cache-control", cache_control)
            .ok()
    }
}

impl StyleSheet {
    pub fn new(inner: &'static str, cache_max_age: i32) -> Self {
        Self {
            inner,
            cache_max_age,
        }
    }
}
