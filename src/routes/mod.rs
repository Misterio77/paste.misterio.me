pub mod account;
pub mod paste;
pub mod user;

pub mod home {
    use crate::schema::Session;
    use rocket::{get, request::FlashMessage, response::Redirect, routes, Route};
    use rocket_dyn_templates::{context, Template};

    #[get("/", rank = 1)]
    async fn home(
        flash: Option<FlashMessage<'_>>,
        session: Option<Session>,
    ) -> Result<Redirect, Template> {
        if session.is_some() {
            Ok(Redirect::to("/p"))
        } else {
            Err(Template::render("home", context! {flash, session}))
        }
    }

    #[get("/", format = "json")]
    async fn api_home() {}

    pub fn routes() -> Vec<Route> {
        routes![home, api_home]
    }
}

pub mod assets {
    use crate::style::StyleSheet;
    use rocket::{get, response::Redirect, routes, Route, State};

    #[get("/style.css")]
    fn style() -> Redirect {
        Redirect::to(format!("/assets/{}/style.css", crate::VERSION))
    }

    #[get("/<_version>/style.css")]
    fn style_versioned<'a>(css: &'a State<StyleSheet>, _version: String) -> &'a StyleSheet<'a> {
        css
    }

    pub fn routes() -> Vec<Route> {
        routes![style, style_versioned]
    }
}

pub mod errors {
    use crate::error::ServerError;
    use rocket::{
        catch, catchers,
        http::{MediaType, Status},
        response::{Flash, Redirect},
        Catcher, Request,
    };

    #[catch(401)]
    fn unauthorized(req: &Request) -> Result<Flash<Redirect>, ServerError> {
        let error = ServerError::builder()
            .code(Status::Unauthorized)
            .message("Please login first")
            .build();

        // If JSON, just return the error
        if req.format() == Some(&MediaType::JSON) {
            Err(error)
        // Else, redirect to login with error as flash messge
        } else {
            let redir = req.uri().to_string();
            let uri = format!("/login?redir={}", redir);
            Ok(error.flash_redirect(&uri))
        }
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
