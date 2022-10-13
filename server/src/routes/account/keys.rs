use crate::{database::Database, schema::ApiKey, schema::Session};

use rocket::{
    delete,
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
use uuid::Uuid;

#[get("/")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    session: Session,
    cookies: &CookieJar<'_>,
) -> Result<Template, Flash<Redirect>> {
    let keys = ApiKey::show_all(&db, &session)
        .await
        .map_err(|e| e.flash_redirect("/u"))?;

    let new_key = cookies.get_private("new_key").map(|c| {
        let value = c.value().to_owned();
        cookies.remove_private(c);
        value
    });

    Ok(Template::render(
        "keys",
        context! {flash, session, keys, new_key},
    ))
}

#[delete("/<id>")]
async fn delete(
    db: Connection<Database>,
    session: Session,
    id: Uuid,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    ApiKey::revoke(&db, &session, Some(id))
        .await
        .map_err(|e| e.flash_redirect("/keys"))?;

    Ok(Flash::success(Redirect::to("/keys"), "Key revoked"))
}

#[derive(FromForm)]
struct CreateForm {
    pub name: Option<String>,
}

#[post("/", data = "<form>")]
async fn create(
    db: Connection<Database>,
    form: Form<CreateForm>,
    session: Session,
    cookies: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let form = form.into_inner();

    let new_key = ApiKey::create(&db, &session, form.name)
        .await
        .map_err(|e| e.flash_redirect("/keys"))?;

    cookies.add_private(Cookie::new("new_key", new_key));

    Ok(Flash::success(Redirect::to("/keys"), "Key created"))
}

pub fn routes() -> Vec<Route> {
    routes![list, delete, create]
}
