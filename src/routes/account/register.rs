use crate::{
    database::Database,
    error::ServerError,
    schema::{Session, User},
};

use rocket::{
    form::{Form, FromForm},
    get, post,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;

#[get("/?<redir>")]
fn get(
    flash: Option<FlashMessage<'_>>,
    session: Option<Session>,
    redir: Option<String>,
) -> Result<Template, Flash<Redirect>> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect(redir.as_deref().unwrap_or_else(|| "/")));
    }

    Ok(Template::render("register", context! {flash, redir, session}))
}

#[derive(FromForm, Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[post("/?<redir>", data = "<form>", rank = 1)]
async fn post(
    db: Connection<Database>,
    form: Form<RegisterForm>,
    session: Option<Session>,
    redir: Option<String>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect(redir.as_deref().unwrap_or_else(|| "/")));
    }

    let RegisterForm {
        username,
        email,
        password,
    } = form.into_inner();

    // Preserve redir if form is failed
    let err_redir = if let Some(r) = redir.as_ref() {
        format!("/register?redir={}", r)
    } else {
        "/register".into()
    };

    let _user = User::register(&db, username, email, password)
        .await
        .map_err(|e| e.flash_redirect(&err_redir))?;

    // Preserve redir when logging in
    let success_redir = if let Some(r) = redir.as_ref() {
        format!("/login?redir={}", r)
    } else {
        "/login".into()
    };
    Ok(Flash::success(
        Redirect::to(success_redir),
        "Registration complete. You can now login",
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, post]
}
