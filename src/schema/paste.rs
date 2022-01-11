use crate::{
    database::{Client, Row},
    error::{ServerError, Status},
};

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Paste {
    pub id: Uuid,
    pub creator: String,
    pub creation: DateTime<Utc>,
    pub content: String,
    pub name: Option<String>,
    pub language: Option<String>,
}

impl Paste {
    async fn fetch(db: &Client, id: Uuid) -> Result<Paste, ServerError> {
        db.query_one(
            "SELECT id, creator, creation, content, name, language
            FROM pastes
            WHERE id = $1",
            &[&id],
        )
        .await?
        .try_into()
    }
    async fn list(db: &Client, creator: &str) -> Result<Vec<Paste>, ServerError> {
        db.query(
            "SELECT id, creator, creation, content, name, language
            FROM pastes
            WHERE creator = $1",
            &[&creator],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
    async fn delete(db: &Client, creator: &str, id: Option<Uuid>) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM pastes
            WHERE creator = $1 AND ($2::uuid IS NULL OR id = $2)",
            &[&creator, &id],
        )
        .await?;
        Ok(())
    }
    async fn insert(&self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "INSERT INTO pastes
            (id, creator, creation, content, name, language)
            VALUES ($1, $2, $3, $4, $5, $6)",
            &[
                &self.id,
                &self.creator,
                &self.creation,
                &self.content,
                &self.name,
                &self.language,
            ],
        )
        .await?;
        Ok(())
    }

    pub async fn create(
        db: &Client,
        username: &str,
        content: String,
        name: Option<String>,
        language: Option<String>,
    ) -> Result<Paste, ServerError> {
        let paste = Paste {
            id: Uuid::new_v4(),
            creator: username.into(),
            creation: Utc::now(),
            content,
            name,
            language,
        };
        paste.insert(db).await?;
        Ok(paste)
    }
    pub async fn get(db: &Client, id: Uuid) -> Result<Paste, ServerError> {
        Paste::fetch(db, id).await.map_err(|e| {
            ServerError::builder_from(e)
                .code(Status::NotFound)
                .message("Paste not found")
                .into()
        })
    }
    pub async fn show_all(&self, db: &Client) -> Result<Vec<Paste>, ServerError> {
        Paste::list(db, &self.creator).await
    }
    pub async fn remove(&self, db: &Client, id: Option<Uuid>) -> Result<(), ServerError> {
        Paste::delete(db, &self.creator, id).await?;
        Ok(())
    }
}

impl TryFrom<Row> for Paste {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Paste, ServerError> {
        Ok(Paste {
            id: row.try_get("id")?,
            creator: row.try_get("creator")?,
            creation: row.try_get("creation")?,
            content: row.try_get("content")?,
            name: row.try_get("name")?,
            language: row.try_get("language")?,
        })
    }
}
