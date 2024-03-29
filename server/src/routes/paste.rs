use crate::{
    common::Created,
    database::Database,
    error::ServerError,
    schema::{ApiKey, Paste, Session},
    syntax::SyntaxSet,
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

#[get("/?<existing>")]
async fn create(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    session: Option<Session>,
    existing: Option<Uuid>,
) -> Template {
    let existing = if let Some(e) = existing {
        Paste::get(&db, e).await.ok()
    } else {
        None
    };

    Template::render("create", context! {flash, session, existing})
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

    let highlighted = paste.highlight(ss).map_err(|e| e.flash_redirect("/"))?;

    Ok(Template::render(
        "paste",
        context! {session, paste, highlighted, flash},
    ))
}

#[get("/<partial_id>", rank = 2)]
async fn find(db: Connection<Database>, partial_id: &str) -> Result<Redirect, Flash<Redirect>> {
    let id = Paste::locate(&db, partial_id)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    Ok(Redirect::to(format!("/p/{}", id)))
}

#[get("/<id>", format = "json")]
async fn api_get(db: Connection<Database>, id: Uuid) -> Result<Json<Paste>, ServerError> {
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
    #[serde(default)]
    unlisted: bool,
}

#[post("/", data = "<form>", rank = 1)]
async fn post(
    db: Connection<Database>,
    session: Option<Session>,
    form: Form<CreateForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();

    let paste = Paste::create(
        &db,
        session.map(|s| s.creator),
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
async fn api_post(
    db: Connection<Database>,
    key: Option<ApiKey>,
    body: Json<CreateForm>,
) -> Result<Created<Json<Paste>>, ServerError> {
    let body = body.into_inner();

    let paste = Paste::create(
        &db,
        key.map(|k| k.creator),
        body.content,
        body.unlisted,
        body.title,
        body.description,
    )
    .await?;

    let location = format!("/p/{}", paste.id);

    Ok(Created::new(Json(paste), &location))
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
        .map_err(|e| e.flash_redirect("/u"))?;

    Ok(Flash::success(Redirect::to("/u"), "Paste deleted"))
}

#[delete("/<id>", format = "json")]
async fn api_delete(db: Connection<Database>, key: ApiKey, id: Uuid) -> Result<(), ServerError> {
    let paste = Paste::get(&db, id).await?;
    paste.remove(&db, Some(id), &key.creator).await?;

    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![create, get, find, get_raw, delete, post, api_get, api_delete, api_post]
}
