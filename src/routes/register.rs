use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn register() -> Template {
    Template::render("register", context! {})
}

pub fn routes() -> Vec<Route> {
    routes![register]
}
