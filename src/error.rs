use std::error::Error as StdError;
use std::fmt;

use rocket::{
    http::Status,
    response::{Flash, Redirect},
};
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
    /// Obter o erro originário
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

/// Converte erro do rocket
impl From<rocket::error::Error> for ServerError {
    fn from(e: rocket::error::Error) -> Self {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .source(Box::new(e))
            .message("Couldn't start up server")
            .build()
    }
}
impl From<rocket_db_pools::deadpool_postgres::tokio_postgres::Error> for ServerError {
    fn from(e: rocket_db_pools::deadpool_postgres::tokio_postgres::Error) -> Self {
        let message = match e.as_db_error() {
            Some(db_e) => {
                db_e.message().into()
            }
            None => {
                format!("{}", e)
            }
        };

        ServerError::builder()
            .message(&message)
            .source(Box::new(e))
            .build()
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for ServerError {
    fn respond_to(
        self,
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::response::Result<'static> {
        let media_type = req.accept().map(|a| a.preferred().media_type());

        let mut response = rocket::response::Response::build();
        response.status(self.code);

        if media_type == Some(&rocket::http::MediaType::JSON) {
            let json = rocket::serde::json::Json(self);
            response.join(json.respond_to(req)?)
        } else {
            let template = Template::render("error", self);
            response.join(template.respond_to(req)?)
        };

        response.ok()
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
