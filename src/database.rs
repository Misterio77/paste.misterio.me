use rocket_db_pools::{deadpool_postgres, Database as DatabaseTrait};

pub use deadpool_postgres::{tokio_postgres::row::Row, ClientWrapper as Client};

#[derive(DatabaseTrait)]
#[database("database")]
pub struct Database(rocket_db_pools::deadpool_postgres::Pool);
