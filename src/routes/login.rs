use crate::{
    database::Database,
    error::ServerError,
    schema::{Session, User},
};

use rocket::{
    form::{Form, FromForm},
    get,
    http::CookieJar,
    post,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use std::net::SocketAddr;

#[get("/")]
fn login(
    flash: Option<FlashMessage<'_>>,
    session: Option<Session>,
) -> Result<Template, Flash<Redirect>> {
    // If already logged in, redirect to home
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    Ok(Template::render("login", context! {flash, session}))
}

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/", data = "<form>")]
async fn post(
    db: Connection<Database>,
    form: Form<LoginForm>,
    source: SocketAddr,
    cookies: &CookieJar<'_>,
    session: Option<Session>,
) -> Result<Redirect, Flash<Redirect>> {
    // If already logged in, redirect to home
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    let LoginForm { username, password } = form.into_inner();

    let (_user, session) = User::login(&db, username, password, source.ip())
        .await
        .map_err(|e| e.flash_redirect("/login"))?;

    cookies.add_private(session.into());

    Ok(Redirect::to(format!("/")))
}

pub fn routes() -> Vec<Route> {
    routes![login, post]
}
