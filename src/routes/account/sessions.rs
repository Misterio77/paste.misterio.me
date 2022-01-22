use crate::{database::Database, schema::Session};

use rocket::{
    delete, get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
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

pub fn routes() -> Vec<Route> {
    routes![list, delete]
}
