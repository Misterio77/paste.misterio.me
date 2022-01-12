use rocket::http::ContentType;
use rocket::response::{self, Responder};
use rocket::Request;
use rocket::Response;

pub struct StyleSheet<'a> {
    inner: &'a str,
    cache_max_age: i32,
}

impl<'a> StyleSheet<'a> {
    pub fn new(inner: &'a str, cache_max_age: i32) -> Self {
        Self {
            inner,
            cache_max_age,
        }
    }
}

impl<'r> Responder<'r, 'static> for &'r StyleSheet<'_> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let cache_control = format!("max-age={}", self.cache_max_age);
        Response::build_from(self.inner.to_owned().respond_to(req)?)
            .header(ContentType::CSS)
            .raw_header("Cache-control", cache_control)
            .ok()
    }
}
