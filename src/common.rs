use rocket::{
    http::{Header, hyper::http::header::LOCATION},
    Responder,
};

#[derive(Responder)]
#[response(status = 201)]
pub struct Created<T> {
    inner: T,
    header: Header<'static>,
}

impl<T> Created<T> {
    pub fn new(inner: T, location: &str) -> Self {
        let header = Header::new(LOCATION.as_str(), location.to_string());
        Created { inner, header }
    }
}
