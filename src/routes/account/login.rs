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

#[get("/")]
fn login(
    flash: Option<FlashMessage<'_>>,
    session: Option<Session>,
) -> Result<Template, Flash<Redirect>> {
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
    source: IpAddr,
    cookies: &CookieJar<'_>,
    session: Option<Session>,
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

    // Redirection cookie
    let redir: String = if let Some(after_login) = cookies.get("after_login") {
        cookies.remove(after_login.to_owned());
        after_login.value()
    } else {
        "/"
    }
    .into();

    Ok(Redirect::to(redir))
}

pub fn routes() -> Vec<Route> {
    routes![login, post]
}
