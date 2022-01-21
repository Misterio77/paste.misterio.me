use crate::{
    database::{Client, Row},
    error::{ServerError, Status},
    syntax::SyntaxSet,
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
    pub unlisted: bool,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl Paste {
    async fn fetch(db: &Client, id: Uuid) -> Result<Paste, ServerError> {
        db.query_one(
            "SELECT id, creator, creation, content, unlisted, title, description
            FROM pastes
            WHERE id = $1",
            &[&id],
        )
        .await?
        .try_into()
    }
    async fn list(db: &Client, creator: &str) -> Result<Vec<Paste>, ServerError> {
        db.query(
            "SELECT id, creator, creation, content, unlisted, title, description
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
            (id, creator, creation, content, unlisted, title, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &self.id,
                &self.creator,
                &self.creation,
                &self.content,
                &self.unlisted,
                &self.title,
                &self.description,
            ],
        )
        .await?;
        Ok(())
    }

    pub async fn create(
        db: &Client,
        creator: &str,
        content: String,
        unlisted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<Paste, ServerError> {
        let paste = Paste {
            id: Uuid::new_v4(),
            creator: creator.into(),
            creation: Utc::now(),
            content,
            unlisted,
            title: title.filter(|s| !s.is_empty()),
            description: description.filter(|s| !s.is_empty()),
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
    pub fn extension(&self) -> Option<&str> {
        self.title.as_ref().and_then(|n| n.split('.').last())
    }
    pub fn highlight(&self, ss: &SyntaxSet) -> String {
        ss.highlight(self.extension(), &self.content)
    }
    pub async fn show_all(
        db: &Client,
        creator: &str,
        requester: Option<&str>,
    ) -> Result<Vec<Paste>, ServerError> {
        Ok(Paste::list(db, creator)
            .await?
            .into_iter()
            .filter(|p| !p.unlisted || requester == Some(&p.creator))
            .collect())
    }
    pub async fn remove(
        &self,
        db: &Client,
        id: Option<Uuid>,
        requester: &str,
    ) -> Result<(), ServerError> {
        if self.creator == requester {
            Paste::delete(db, &self.creator, id).await?;
            Ok(())
        } else {
            Err(ServerError::builder()
                .code(Status::Forbidden)
                .message("This paste isn't yours")
                .into())
        }
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
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            unlisted: row.try_get("unlisted")?,
        })
    }
}
