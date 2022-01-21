use crate::{
    database::Database, error::ServerError, schema::Paste, schema::Session, syntax::SyntaxSet,
};

use rocket::{
    delete,
    form::{Form, FromForm},
    get, post,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes,
    serde::json::Json,
    Route, State,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;

use uuid::Uuid;

#[get("/")]
async fn root() -> Redirect {
    Redirect::to("/")
}

#[get("/<id>", rank = 1)]
async fn get(
    db: Connection<Database>,
    session: Option<Session>,
    id: Uuid,
    ss: &State<SyntaxSet>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Flash<Redirect>> {
    let paste = Paste::get(&db, id)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    let highlighted = paste.highlight(ss);

    Ok(Template::render(
        "paste",
        context! {session, paste, highlighted, flash},
    ))
}

#[get("/<id>", format = "json")]
async fn get_json(db: Connection<Database>, id: Uuid) -> Result<Json<Paste>, ServerError> {
    let paste = Paste::get(&db, id).await?;

    Ok(Json(paste))
}

#[get("/<id>/raw")]
async fn get_raw(db: Connection<Database>, id: Uuid) -> Result<String, ServerError> {
    let paste = Paste::get(&db, id).await?;

    Ok(paste.content)
}

#[derive(FromForm, Deserialize)]
struct CreateForm {
    title: Option<String>,
    description: Option<String>,
    content: String,
    unlisted: bool,
}

#[post("/", data = "<form>", rank = 1)]
async fn post(
    db: Connection<Database>,
    session: Session,
    form: Form<CreateForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();

    let paste = Paste::create(
        &db,
        &session.creator,
        form.content,
        form.unlisted,
        form.title,
        form.description,
    )
    .await
    .map_err(|e| e.flash_redirect("/"))?;

    Ok(Redirect::to(format!("/p/{}", paste.id)))
}

#[post("/", data = "<body>", format = "json")]
async fn post_json(
    db: Connection<Database>,
    session: Session,
    body: Form<CreateForm>,
) -> Result<Json<Paste>, ServerError> {
    let body = body.into_inner();

    let paste = Paste::create(
        &db,
        &session.creator,
        body.content,
        body.unlisted,
        body.title,
        body.description,
    )
    .await?;

    Ok(Json(paste))
}

#[delete("/<id>", rank = 1)]
async fn delete(
    db: Connection<Database>,
    session: Session,
    id: Uuid,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let paste = Paste::get(&db, id)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    paste
        .remove(&db, Some(id), &session.creator)
        .await
        .map_err(|e| e.flash_redirect("/p/{{ id }}"))?;

    Ok(Flash::success(Redirect::to("/"), "Paste deleted"))
}

#[delete("/<id>", format = "json")]
async fn delete_json(
    db: Connection<Database>,
    session: Session,
    id: Uuid,
) -> Result<Json<()>, ServerError> {
    let paste = Paste::get(&db, id).await?;
    paste.remove(&db, Some(id), &session.creator).await?;

    Ok(Json(()))
}

pub fn routes() -> Vec<Route> {
    routes![
        root,
        get,
        get_json,
        delete,
        delete_json,
        post,
        post_json,
        get_raw
    ]
}
