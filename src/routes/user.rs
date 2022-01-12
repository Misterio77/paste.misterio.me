use crate::{
    database::Database,
    schema::{Paste, Session, User},
};

use rocket::{
    get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn root(session: Session) -> Redirect {
    Redirect::to(format!("/u/{}", session.creator))
}

#[get("/<username>")]
async fn get(
    db: Connection<Database>,
    session: Option<Session>,
    username: String,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Flash<Redirect>> {
    let user = User::get(&db, &username)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    let mut pastes = Paste::show_all(&db, &username)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    pastes.sort_unstable_by(|a, b| b.creation.partial_cmp(&a.creation).unwrap());

    Ok(Template::render(
        "user",
        context! {session, user, flash, pastes},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![root, get]
}
