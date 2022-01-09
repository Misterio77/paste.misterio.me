use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn login() -> Template {
    Template::render("login", context! {})
}

pub fn routes() -> Vec<Route> {
    routes![login]
}
