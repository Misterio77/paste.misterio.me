use crate::{database::Database, error::ServerError, schema::Session};

use rocket::{
    form::{Form, FromForm},
    get,
    http::{Cookie, CookieJar},
    post,
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
    // If not logged in, redirect to login with error msg
    let session = session.map_err(|e| e.flash_redirect("/login"))?;

    // If is, show logout form
    Ok(Template::render("logout", context! {flash, session}))
}

#[derive(FromForm)]
struct LogoutForm {
    all: bool,
}

#[post("/", data = "<form>")]
async fn post(
    db: Connection<Database>,
    form: Form<LogoutForm>,
    cookies: &CookieJar<'_>,
    session: Result<Session, ServerError>,
) -> Result<Redirect, Flash<Redirect>> {
    // If not logged in, redirect to login with error msg
    let session = session.map_err(|e| e.flash_redirect("/login"))?;

    cookies.remove_private(Cookie::named("session"));
    if form.all {
        session.revoke_all(&db).await
    } else {
        session.revoke_self(&db).await
    }
    .map_err(|e| e.flash_redirect("/logout"))?;

    Ok(Redirect::to(format!("/")))
}

pub fn routes() -> Vec<Route> {
    routes![get, post]
}
