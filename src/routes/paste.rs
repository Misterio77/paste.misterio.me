use crate::{database::Database, error::ServerError, schema::Paste, schema::Session, syntax::SyntaxSet};

use rocket::{
    delete,
    form::{Form, FromForm},
    get, post,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route, State,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use uuid::Uuid;

#[get("/")]
async fn root() -> Redirect {
    Redirect::to("/")
}

#[get("/<id>")]
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

#[get("/<id>/raw")]
async fn get_raw(db: Connection<Database>, id: Uuid) -> Result<String, ServerError> {
    let paste = Paste::get(&db, id).await?;

    Ok(paste.content)
}

#[derive(FromForm)]
struct CreateForm {
    title: Option<String>,
    description: Option<String>,
    content: String,
    unlisted: bool,
}

#[post("/", data = "<form>")]
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

#[delete("/<id>")]
async fn delete(
    db: Connection<Database>,
    session: Session,
    id: Uuid,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let paste = Paste::get(&db, id)
        .await
        .map_err(|e| e.flash_redirect("/"))?;

    if paste.creator != session.creator {
        return Err(Flash::error(
            Redirect::to("/p/{{ id }}"),
            "This paste isn't yours",
        ));
    }

    paste
        .remove(&db, Some(id))
        .await
        .map_err(|e| e.flash_redirect("/p/{{ id }}"))?;

    Ok(Flash::success(Redirect::to("/"), "Paste deleted"))
}

pub fn routes() -> Vec<Route> {
    routes![root, get, delete, post, get_raw]
}
