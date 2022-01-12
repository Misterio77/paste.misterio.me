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
use std::net::IpAddr;

#[get("/?<redir>")]
pub fn get(
    flash: Option<FlashMessage<'_>>,
    session: Option<Session>,
    redir: Option<String>,
) -> Result<Template, Flash<Redirect>> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    Ok(Template::render("login", context! {flash, session, redir}))
}

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/?<redir>", data = "<form>")]
async fn post(
    db: Connection<Database>,
    form: Form<LoginForm>,
    source: IpAddr,
    cookies: &CookieJar<'_>,
    session: Option<Session>,
    redir: Option<String>,
) -> Result<Redirect, Flash<Redirect>> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    let LoginForm { username, password } = form.into_inner();

    let new_session = User::login(&db, username, password, source)
        .await
        .map_err(|e| e.flash_redirect("/login"))?;

    cookies.add_private(new_session.into());

    Ok(Redirect::to(redir.unwrap_or_else(|| "/".into())))
}

pub fn routes() -> Vec<Route> {
    routes![get, post]
}
