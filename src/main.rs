use paste_misterio_me::{
    database::Database,
    error::ServerError,
    routes::{assets, errors},
    tera::customize,
};

use rocket::routes;
// use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;
use rocket_post_as_delete::PostAsDelete;
use rocket_sass_fairing::SassSheet;
use rocket_async_compression::Compression;

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    rocket::build()
        .attach(Database::init())
        .attach(Template::custom(customize))
        .attach(SassSheet::fairing())
        .attach(Compression::fairing())
        .attach(PostAsDelete)
        .register("/", errors::catchers())
        .mount("/assets", routes![assets::style])
        .launch()
        .await?;
    Ok(())
}
