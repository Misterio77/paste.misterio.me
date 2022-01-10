use paste_misterio_me::{
    database::Database,
    error::ServerError,
    routes::{assets, errors, home, login, logout, register, sessions},
    tera::customize,
};

use rocket_async_compression::Compression;
use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;
use rocket_post_as_delete::PostAsDelete;
use rocket_sass_fairing::SassSheet;

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    rocket::build()
        // Fairings
        .attach(Database::init())
        .attach(Template::custom(customize))
        .attach(SassSheet::fairing())
        .attach(Compression::fairing())
        .attach(PostAsDelete)
        // Error catchers
        .register("/", errors::catchers())
        // Assets
        .mount("/assets", assets::routes())
        // Routes
        .mount("/", home::routes())
        .mount("/login", login::routes())
        .mount("/logout", logout::routes())
        .mount("/register", register::routes())
        .mount("/sessions", sessions::routes())
        .launch()
        .await?;
    Ok(())
}
