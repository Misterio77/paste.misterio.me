use crate::{database::Database, error::ServerError, schema::Session};

use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::{
    delete, get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use std::convert::TryInto;

#[get("/")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    session: Result<Session, ServerError>,
) -> Result<Template, Flash<Redirect>> {
    let session = session.map_err(|e| e.flash_redirect("/login"))?;
    let sessions = session
        .show_all(&db)
        .await
        .map_err(|e| e.flash_redirect("/login"))?;

    Ok(Template::render(
        "sessions",
        context! {flash, session, sessions},
    ))
}

#[delete("/<creation_nano>")]
async fn delete(
    db: Connection<Database>,
    session: Result<Session, ServerError>,
    creation_nano: i64,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let session = session.map_err(|e| e.flash_redirect("/login"))?;
    let creation = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(
            creation_nano / 1000000000,
            (creation_nano % 1000000000).try_into().unwrap(),
        ),
        Utc,
    );

    session
        .revoke(&db, &creation)
        .await
        .map_err(|e| e.flash_redirect("/login"))?;

    Ok(Flash::success(Redirect::to("/sessions"), "Session revoked"))
}

pub fn routes() -> Vec<Route> {
    routes![list, delete]
}
