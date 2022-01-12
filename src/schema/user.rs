use crate::{
    database::{Client, Row},
    error::{ServerError, Status},
    schema::Session,
};

use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use std::net::IpAddr;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
}

impl User {
    async fn fetch(db: &Client, username: &str) -> Result<User, ServerError> {
        db.query_one(
            "SELECT username, email, password
            FROM users
            WHERE username = $1",
            &[&username],
        )
        .await?
        .try_into()
    }
    async fn _delete(db: &Client, username: &str) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM users
            WHERE username = $1",
            &[&username],
        )
        .await?;
        Ok(())
    }
    async fn insert(&self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "INSERT INTO users
            (username, email, password)
            VALUES ($1, $2, $3)",
            &[&self.username, &self.email, &self.password],
        )
        .await?;
        Ok(())
    }
    async fn _update(&self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "UPDATE users SET
            username = $1, email = $2, password = $3",
            &[&self.username, &self.email, &self.password],
        )
        .await?;
        Ok(())
    }

    pub async fn get(db: &Client, username: &str) -> Result<User, ServerError> {
        User::fetch(db, username).await
    }
    pub async fn register(
        db: &Client,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, ServerError> {
        let password = hash_password(&password)?;
        let user = User {
            username,
            email,
            password,
        };
        user.insert(db).await?;
        Ok(user)
    }
    pub async fn login(
        db: &Client,
        username: String,
        password: String,
        source: IpAddr,
    ) -> Result<Session, ServerError> {
        let user = User::get(db, &username).await.map_err(|e| {
            ServerError::builder_from(e)
                .message("User not found")
                .code(Status::Unauthorized)
                .build()
        })?;

        if verify_password(&password, &user.password)? {
            Ok(Session::create(db, &username, source).await?)
        } else {
            Err(ServerError::builder()
                .message("Invalid credentials")
                .code(Status::Unauthorized)
                .build())
        }
    }
}

impl TryFrom<Row> for User {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<User, ServerError> {
        Ok(User {
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
        })
    }
}

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    use argon2::Config;
    use rand::Rng;

    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}
