pub struct Client;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    redirect, Client as ReqClient, ClientBuilder,
};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl Client {
    pub fn new() -> ReqClient {
        Client::base(false).build().unwrap()
    }
    pub fn new_with_redir() -> ReqClient {
        Client::base(true).build().unwrap()
    }
    pub fn base(redirect: bool) -> ClientBuilder {
        let mut headers = HeaderMap::new();

        let json: HeaderValue = "application/json".parse().unwrap();
        headers.insert(header::CONTENT_TYPE, json.clone());
        headers.insert(header::ACCEPT, json);

        let redirect = if redirect {
            redirect::Policy::default()
        } else {
            redirect::Policy::none()
        };

        ReqClient::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .redirect(redirect)
    }
}
