pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod common;
pub mod database;
pub mod error;
pub mod routes;
pub mod schema;
pub mod style;
pub mod syntax;
pub mod tera;

pub use common::Created;
