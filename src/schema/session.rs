use crate::{
    database::{Client, Row},
    error::{ServerError, Status},
};

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use std::net::IpAddr;

#[derive(Debug, Serialize)]
pub struct Session {
    pub creator: String,
    pub source: IpAddr,
    pub creation: DateTime<Utc>,
    #[serde(skip_serializing)]
    token: String,
}

impl Session {
    async fn get(db: &Client, token: &str) -> Result<Session, ServerError> {
        db.query_one(
            "SELECT token, creator, source, creation
            FROM sessions
            WHERE token = $1",
            &[&token],
        )
        .await?
        .try_into()
    }
    async fn list(db: &Client, creator: &str) -> Result<Vec<Session>, ServerError> {
        db.query(
            "SELECT token, creator, source, creation
            FROM sessions
            WHERE creator = $1",
            &[&creator],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
    async fn delete(
        db: &Client,
        creator: &str,
        creation: &DateTime<Utc>,
    ) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM sessions
            WHERE creation = $1 AND creator = $2",
            &[&creation, &creator],
        )
        .await?;
        Ok(())
    }
    async fn delete_all(db: &Client, creator: &str) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM sessions
            WHERE creator = $1",
            &[&creator],
        )
        .await?;
        Ok(())
    }
    async fn insert(&self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "INSERT INTO sessions
            (token, creator, source, creation)
            VALUES ($1, $2, $3, $4)",
            &[&self.token, &self.creator, &self.source, &self.creation],
        )
        .await?;
        Ok(())
    }

    pub async fn create(
        db: &Client,
        username: &str,
        source: IpAddr,
    ) -> Result<Session, ServerError> {
        let token = generate_token();
        let creator = username.into();
        let creation = Utc::now();

        let session = Session {
            token,
            creator,
            source,
            creation,
        };
        session.insert(db).await?;
        Ok(session)
    }
    pub async fn authenticate(db: &Client, token: &str) -> Result<Session, ServerError> {
        Session::get(db, &token).await.map_err(|e| {
            ServerError::builder_from(e)
                .code(Status::Unauthorized)
                .message("Session expired, please login again")
                .into()
        })
    }
    pub async fn show_all(&self, db: &Client) -> Result<Vec<Session>, ServerError> {
        Session::list(db, &self.creator).await
    }
    pub async fn revoke(self, db: &Client, creation: &DateTime<Utc>) -> Result<(), ServerError> {
        Session::delete(db, &self.creator, creation).await?;
        Ok(())
    }
    pub async fn revoke_self(self, db: &Client) -> Result<(), ServerError> {
        let creation = self.creation.clone();
        self.revoke(db, &creation).await?;
        Ok(())
    }
    pub async fn revoke_all(self, db: &Client) -> Result<(), ServerError> {
        Session::delete_all(db, &self.creator).await?;
        Ok(())
    }
}

fn generate_token() -> String {
    use rand::{distributions::Alphanumeric, Rng};

    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

impl TryFrom<Row> for Session {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Session, ServerError> {
        Ok(Session {
            token: row.try_get("token")?,
            creator: row.try_get("creator")?,
            source: row.try_get("source")?,
            creation: row.try_get("creation")?,
        })
    }
}

use rocket::http::Cookie;
impl From<Session> for Cookie<'_> {
    fn from(session: Session) -> Self {
        Cookie::new("session", session.token)
    }
}

use crate::database::Database;
use rocket::{
    outcome::{try_outcome, IntoOutcome},
    request::{self, FromRequest, Request},
};
use rocket_db_pools::Connection;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ServerError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        let db = try_outcome!(request
            .guard::<Connection<Database>>()
            .await
            .map_failure(|e| ServerError::from(e)));

        let token = try_outcome!(cookies
            .get_private("session")
            .and_then(|cookie| cookie.value().parse::<String>().ok())
            .into_outcome(
                ServerError::builder()
                    .code(Status::Unauthorized)
                    .message("You're not logged in")
                    .build()
            ));

        let session = try_outcome!(Session::authenticate(&db, &token)
            .await
            .into_outcome(Status::Unauthorized));

        request::Outcome::Success(session)
    }
}
