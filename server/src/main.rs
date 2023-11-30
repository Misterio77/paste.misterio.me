use paste_misterio_me::{
    database::Database,
    error::ServerError,
    routes::{account, assets, errors, home, paste, user},
    syntax::SyntaxSet,
    tera::customize,
    SYNTAXES,
};

use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;
use rocket_post_as_delete::PostAsDelete;

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    let syntaxes = SyntaxSet::new(SYNTAXES);
    rocket::build()
        // Fairings
        .attach(Database::init())
        .attach(Template::custom(customize))
        .attach(PostAsDelete)
        // Manage SyntaxSet
        .manage(syntaxes)
        // Error catchers
        .register("/", errors::catchers())
        // Assets
        .mount("/assets", assets::routes())
        // Home routes
        .mount("/", home::routes())
        // Account routes
        .mount("/login", account::login::routes())
        .mount("/logout", account::logout::routes())
        .mount("/register", account::register::routes())
        .mount("/sessions", account::sessions::routes())
        .mount("/keys", account::keys::routes())
        // Pastes
        .mount("/p", paste::routes())
        // Users
        .mount("/u", user::routes())
        .launch()
        .await?;
    Ok(())
}
