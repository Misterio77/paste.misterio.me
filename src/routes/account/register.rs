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
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;

#[get("/")]
fn get(
    flash: Option<FlashMessage<'_>>,
    session: Option<Session>,
) -> Result<Template, Flash<Redirect>> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    Ok(Template::render("register", context! {flash, session}))
}

#[derive(FromForm, Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[post("/", data = "<form>", rank = 1)]
async fn post(
    db: Connection<Database>,
    form: Form<RegisterForm>,
    session: Option<Session>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    let RegisterForm {
        username,
        email,
        password,
    } = form.into_inner();

    let _user = User::register(&db, username, email, password)
        .await
        .map_err(|e| e.flash_redirect("/register"))?;

    Ok(Flash::success(
        Redirect::to("/login"),
        "Registration complete. You can now login",
    ))
}

#[post("/", data = "<body>", format = "json")]
async fn post_json(
    db: Connection<Database>,
    body: Form<RegisterForm>,
    session: Option<Session>,
) -> Result<Json<()>, ServerError> {
    if session.is_some() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build());
    }

    let RegisterForm {
        username,
        email,
        password,
    } = body.into_inner();

    let _user = User::register(&db, username, email, password).await?;

    Ok(Json(()))
}

pub fn routes() -> Vec<Route> {
    routes![get, post, post_json]
}
