use paste_misterio_me::{
    database::Database,
    error::ServerError,
    routes::{
        account::{login, logout, register, sessions},
        assets, errors, home,
    },
    tera::customize,
};

use rocket_assets_fairing::Assets;
use rocket_async_compression::Compression;
use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;
use rocket_post_as_delete::PostAsDelete;

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    rocket::build()
        // Fairings
        .attach(Database::init())
        .attach(Template::custom(customize))
        .attach(Assets::fairing())
        .attach(Compression::fairing())
        .attach(PostAsDelete)
        // Error catchers
        .register("/", errors::catchers())
        // Assets
        .mount("/assets", assets::routes())
        // Home routes
        .mount("/", home::routes())
        // Account related
        .mount("/login", login::routes())
        .mount("/logout", logout::routes())
        .mount("/register", register::routes())
        .mount("/sessions", sessions::routes())
        .launch()
        .await?;
    Ok(())
}
