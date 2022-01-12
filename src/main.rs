use paste_misterio_me::{
    database::Database,
    error::ServerError,
    routes::{account, assets, errors, home, paste, user},
    style::StyleSheet,
    tera::customize,
};

use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;
use rocket_post_as_delete::PostAsDelete;
use syntect::parsing::SyntaxSet;

static SYNTAXES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/syntaxes.bin"));
static CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    let ss = {
        let loaded: SyntaxSet = syntect::dumps::from_binary(SYNTAXES);
        let mut builder = loaded.into_builder();
        builder.add_plain_text_syntax();
        builder.build()
    };

    let css = StyleSheet::new(CSS, 86400);

    rocket::build()
        // Fairings
        .attach(Database::init())
        .attach(Template::custom(customize))
        .attach(PostAsDelete)
        // Manage SyntaxSet and StyleSheet
        .manage(ss)
        .manage(css)
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
        // Pastes
        .mount("/p", paste::routes())
        // Users
        .mount("/u", user::routes())
        .launch()
        .await?;
    Ok(())
}
