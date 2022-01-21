pub use rocket::{
    http::{MediaType, Status},
    outcome::{IntoOutcome, Outcome},
    response::{Responder, Response},
    serde::json::Json,
};

use std::error::Error as StdError;
use std::fmt;

use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

#[derive(Debug)]
pub struct ServerError {
    code: Status,
    source: Option<Box<dyn StdError + Sync + Send>>,
    message: Option<String>,
}

impl ServerError {
    pub fn builder() -> ServerErrorBuilder {
        ServerError::default().edit()
    }
    pub fn edit(self) -> ServerErrorBuilder {
        ServerErrorBuilder { inner: self }
    }
    pub fn builder_from<T: Into<ServerError>>(source: T) -> ServerErrorBuilder {
        let error = source.into();
        error.edit()
    }

    pub fn flash_redirect(&self, url: &str) -> Flash<Redirect> {
        let message = self.message.as_deref().unwrap_or("Unknown Error.");
        Flash::error(Redirect::to(url.to_string()), message)
    }
}

impl Default for ServerError {
    fn default() -> Self {
        ServerError {
            code: Status::InternalServerError,
            source: None,
            message: None,
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.reason_lossy())?;
        if let Some(message) = &self.message {
            write!(f, " ({})", message)?;
        };
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        };
        Ok(())
    }
}

impl StdError for ServerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

impl From<ServerErrorBuilder> for ServerError {
    fn from(e: ServerErrorBuilder) -> Self {
        e.build()
    }
}

impl<T: Into<ServerError>> From<Option<T>> for ServerError {
    fn from(e: Option<T>) -> Self {
        match e {
            Some(error) => ServerError::builder_from(error),
            None => ServerError::builder(),
        }
        .build()
    }
}

impl<T: Into<ServerError>> From<(Status, T)> for ServerError {
    fn from(e: (Status, T)) -> Self {
        ServerError::builder_from(e.1).code(e.0).build()
    }
}

impl From<ServerError> for (Status, ServerError) {
    fn from(e: ServerError) -> Self {
        (e.code, e)
    }
}

impl<
        T: Send + Sync + fmt::Display + fmt::Debug + 'static,
        U: Send + Sync + fmt::Display + fmt::Debug + 'static,
    > From<rocket_db_pools::Error<T, U>> for ServerError
{
    fn from(e: rocket_db_pools::Error<T, U>) -> Self {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .source(Box::new(e))
            .message("Couldn't start database connection pool")
            .build()
    }
}

impl From<rocket::error::Error> for ServerError {
    fn from(e: rocket::error::Error) -> Self {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .source(Box::new(e))
            .message("Couldn't start up server")
            .build()
    }
}
impl From<argon2::Error> for ServerError {
    fn from(e: argon2::Error) -> Self {
        ServerError::builder()
            .code(Status::InternalServerError)
            .source(Box::new(e))
            .message("Hashing error")
            .build()
    }
}
impl From<rocket_db_pools::deadpool_postgres::tokio_postgres::Error> for ServerError {
    fn from(e: rocket_db_pools::deadpool_postgres::tokio_postgres::Error) -> Self {
        let (message, code) = match e.as_db_error() {
            Some(db_e) => {
                if db_e.constraint() == Some("users_pkey") {
                    ("This username is already in use".into(), Status::Conflict)
                } else if db_e.constraint() == Some("users_email_un") {
                    ("This email is already in use".into(), Status::Conflict)
                } else {
                    (db_e.to_string(), Status::InternalServerError)
                }
            }
            None => (e.to_string(), Status::InternalServerError),
        };

        ServerError::builder()
            .message(&message)
            .code(code)
            .source(Box::new(e))
            .build()
    }
}

impl<'r> Responder<'r, 'static> for ServerError {
    fn respond_to(
        self,
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::response::Result<'static> {
        let code = self.code;

        let response = if req.format() == Some(&MediaType::JSON) {
            Json(self).respond_to(req)
        } else {
            Template::render("error", self).respond_to(req)
        }?;

        rocket::response::Response::build()
            .status(code)
            .join(response)
            .ok()
    }
}

impl Serialize for ServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ServerError", 2)?;
        state.serialize_field("code", &format!("{}", &self.code))?;
        state.serialize_field("description", &self.message)?;
        state.serialize_field(
            "reason",
            &self
                .source
                .as_ref()
                .map(|s| format!("{:?}", s).replace("\"", "'").replace("\\'", "'")),
        )?;
        state.end()
    }
}

pub struct ServerErrorBuilder {
    inner: ServerError,
}

impl ServerErrorBuilder {
    pub fn build(self) -> ServerError {
        self.inner
    }

    pub fn code(self, code: Status) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError { code, ..self.inner },
        }
    }
    pub fn source(self, source: Box<dyn StdError + Sync + Send>) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError {
                source: Some(source),
                ..self.inner
            },
        }
    }
    pub fn message(self, message: &str) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError {
                message: Some(message.into()),
                ..self.inner
            },
        }
    }
}
