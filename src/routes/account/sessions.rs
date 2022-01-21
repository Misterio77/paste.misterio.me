use crate::{database::Database, error::ServerError, schema::Session};

use rocket::{
    delete, get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

#[get("/", rank = 1)]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    session: Session,
) -> Result<Template, Flash<Redirect>> {
    let sessions = session
        .show_all(&db)
        .await
        .map_err(|e| e.flash_redirect("/login"))?;

    Ok(Template::render(
        "sessions",
        context! {flash, session, sessions},
    ))
}

#[get("/", format = "json")]
async fn list_json(
    db: Connection<Database>,
    session: Session,
) -> Result<Json<Vec<Session>>, ServerError> {
    let sessions = session.show_all(&db).await?;
    Ok(Json(sessions))
}

#[delete("/<id>", rank = 1)]
async fn delete(
    db: Connection<Database>,
    session: Session,
    id: Uuid,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let redir = if session.id == id {
        "/login"
    } else {
        "/sessions"
    };

    session
        .revoke(&db, Some(id))
        .await
        .map_err(|e| e.flash_redirect("/login"))?;

    Ok(Flash::success(Redirect::to(redir), "Session revoked"))
}

#[delete("/<id>", format = "json")]
async fn delete_json(
    db: Connection<Database>,
    session: Session,
    id: Uuid,
) -> Result<Json<()>, ServerError> {
    session.revoke(&db, Some(id)).await?;

    Ok(Json(()))
}

pub fn routes() -> Vec<Route> {
    routes![list, list_json, delete, delete_json]
}
