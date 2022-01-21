use crate::{
    database::Database,
    error::ServerError,
    schema::{Paste, Session, User},
};

use rocket::{
    get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn root(session: Session) -> Redirect {
    Redirect::to(format!("/u/{}", session.creator))
}

#[get("/<username>", rank = 1)]
async fn get(
    db: Connection<Database>,
    session: Option<Session>,
    username: String,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Flash<Redirect>> {
    let user = User::get(&db, &username)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    let requester = session.as_ref().map(|s| &*s.creator);

    let mut pastes = Paste::show_all(&db, &username, requester)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    pastes.sort_unstable_by(|a, b| b.creation.partial_cmp(&a.creation).unwrap());

    Ok(Template::render(
        "user",
        context! {session, user, flash, pastes},
    ))
}

#[get("/<username>", format = "json")]
async fn get_json(db: Connection<Database>, username: String) -> Result<Json<User>, ServerError> {
    let user = User::get(&db, &username).await?;
    Ok(Json(user))
}

#[get("/<username>/pastes", format = "json")]
async fn get_pastes_json(
    db: Connection<Database>,
    session: Option<Session>,
    username: String,
) -> Result<Json<Vec<Paste>>, ServerError> {
    let requester = session.as_ref().map(|s| &*s.creator);
    let mut pastes = Paste::show_all(&db, &username, requester).await?;
    pastes.sort_unstable_by(|a, b| b.creation.partial_cmp(&a.creation).unwrap());

    Ok(Json(pastes))
}

pub fn routes() -> Vec<Route> {
    routes![root, get, get_json, get_pastes_json]
}
