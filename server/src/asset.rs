use rocket::http::ContentType;
use rocket::response::{self, Responder};
use rocket::Request;
use rocket::Response;

pub struct Asset<'a> {
    inner: &'a str,
    cache_max_age: i32,
    content_type: ContentType,
}

impl<'a> Asset<'a> {
    pub fn new(inner: &'a str, cache_max_age: i32, content_type: ContentType) -> Self {
        Self {
            inner,
            cache_max_age,
            content_type,
        }
    }
}

impl<'r> Responder<'r, 'static> for &'r Asset<'_> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let cache_control = format!("max-age={}", self.cache_max_age);
        Response::build_from(self.inner.to_owned().respond_to(req)?)
            .header(self.content_type.clone())
            .raw_header("Cache-control", cache_control)
            .ok()
    }
}

impl Responder<'_, 'static> for Asset<'_> {
    fn respond_to(self, req: &Request<'_>) -> response::Result<'static> {
        (&self).respond_to(&req)
    }
}
