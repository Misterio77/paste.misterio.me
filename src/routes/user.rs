use crate::{
    database::Database,
    error::ServerError,
    schema::{ApiKey, Paste, Session, User},
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

#[get("/", rank = 1)]
async fn root(session: Session) -> Redirect {
    Redirect::to(format!("/u/{}", session.creator))
}

#[get("/", format = "json")]
async fn api_root(key: ApiKey) -> Redirect {
    Redirect::to(format!("/u/{}", key.creator))
}

#[get("/pastes", rank = 1)]
async fn root_pastes(session: Session) -> Redirect {
    Redirect::to(format!("/u/{}/pastes", session.creator))
}

#[get("/pastes", format = "json")]
async fn api_root_pastes(key: ApiKey) -> Redirect {
    Redirect::to(format!("/u/{}/pastes", key.creator))
}

#[get("/<username>")]
async fn get(username: String) -> Redirect {
    Redirect::to(format!("/u/{}/pastes", username))
}

#[get("/<username>/pastes", rank = 1)]
async fn get_pastes(
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
        "pastes",
        context! {session, user, flash, pastes},
    ))
}

#[get("/<username>/pastes", format = "json")]
async fn api_get_pastes(
    db: Connection<Database>,
    key: Option<ApiKey>,
    username: String,
) -> Result<Json<Vec<Paste>>, ServerError> {
    let requester = key.as_ref().map(|k| &*k.creator);
    let mut pastes = Paste::show_all(&db, &username, requester).await?;
    pastes.sort_unstable_by(|a, b| b.creation.partial_cmp(&a.creation).unwrap());

    Ok(Json(pastes))
}

pub fn routes() -> Vec<Route> {
    routes![
        root,
        root_pastes,
        get,
        get_pastes,
        api_root,
        api_root_pastes,
        api_get_pastes
    ]
}
