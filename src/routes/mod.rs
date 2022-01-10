pub mod login;
pub mod logout;
pub mod register;
pub mod sessions;

pub mod home {
    use crate::schema::Session;
    use rocket::{get, request::FlashMessage, routes, Route};
    use rocket_dyn_templates::{context, Template};

    #[get("/")]
    async fn home(flash: Option<FlashMessage<'_>>, session: Option<Session>) -> Template {
        Template::render("base", context! {flash, session})
    }

    pub fn routes() -> Vec<Route> {
        routes![home]
    }
}

pub mod assets {
    use rocket::{get, routes, Route};
    use rocket_sass_fairing::SassSheet;

    #[get("/style.css")]
    async fn style(sheet: &SassSheet) -> &SassSheet {
        sheet
    }

    pub fn routes() -> Vec<Route> {
        routes![style]
    }
}

pub mod errors {
    use crate::error::ServerError;
    use rocket::{catch, catchers, http::Status, Catcher, Request};
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
        catchers![not_found, service_unavailable, unknown_error]
    }
}
