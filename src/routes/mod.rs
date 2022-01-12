pub mod account;
pub mod paste;
pub mod user;

pub mod home {
    use crate::schema::Session;
    use rocket::{get, request::FlashMessage, routes, Route};
    use rocket_dyn_templates::{context, Template};

    #[get("/")]
    async fn home(flash: Option<FlashMessage<'_>>, session: Option<Session>) -> Template {
        Template::render("home", context! {flash, session})
    }

    pub fn routes() -> Vec<Route> {
        routes![home]
    }
}

pub mod assets {
    use crate::style::StyleSheet;
    use rocket::{get, routes, Route, State};

    #[get("/style.css")]
    async fn assets(css: &State<StyleSheet>) -> &StyleSheet {
        css
    }

    pub fn routes() -> Vec<Route> {
        routes![assets]
    }
}

pub mod errors {
    use crate::error::ServerError;
    use rocket::{
        catch, catchers,
        http::Status,
        response::{Flash, Redirect},
        Catcher, Request,
    };
    #[catch(401)]
    fn unauthorized(req: &Request) -> Flash<Redirect> {
        let redir = req.uri().to_string();
        let uri = format!("/login?redir={}", redir);

        ServerError::builder()
            .code(Status::Unauthorized)
            .message("Please login first")
            .build()
            .flash_redirect(&uri)
    }

    #[catch(404)]
    fn not_found() -> ServerError {
        ServerError::builder()
            .code(Status::NotFound)
            .message("Page not found")
            .build()
    }

    #[catch(503)]
    fn service_unavailable() -> ServerError {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .message("Temporarily unavailable")
            .build()
    }

    #[catch(default)]
    fn unknown_error(status: Status, _: &Request) -> ServerError {
        ServerError::builder()
            .code(status)
            .message("Unexpected error ocurred")
            .build()
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, service_unavailable, unknown_error, unauthorized]
    }
}
