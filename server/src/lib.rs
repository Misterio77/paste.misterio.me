pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod asset;
pub mod common;
pub mod database;
pub mod error;
pub mod routes;
pub mod schema;
pub mod syntax;
pub mod tera;

pub use common::Created;

pub static SYNTAXES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/syntaxes.bin"));
pub static STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
pub static TURBO: &str = include_str!(concat!(env!("TURBO_PATH"), "/turbo.es2017-umd.js"));
