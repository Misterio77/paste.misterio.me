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

#[get("/")]
fn get(
    flash: Option<FlashMessage<'_>>,
    session: Result<Session, ServerError>,
) -> Result<Template, Flash<Redirect>> {
    if session.is_ok() {
        return Err(ServerError::builder()
            .message("You're already logged in")
            .build()
            .flash_redirect("/"));
    }

    Ok(Template::render("register", context! {flash, session}))
}

#[derive(FromForm)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[post("/", data = "<form>")]
async fn post(
    db: Connection<Database>,
    form: Form<RegisterForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let RegisterForm {
        username,
        email,
        password,
    } = form.into_inner();

    let _user = User::register(&db, username, email, password)
        .await
        .map_err(|e| e.flash_redirect("/register"))?;

    Ok(Flash::success(Redirect::to(format!("/login")), "Registration complete. Please login"))
}

pub fn routes() -> Vec<Route> {
    routes![get, post]
}
