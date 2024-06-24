use crate::templates;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub enum RenderError {
    Internal(String),
    #[allow(dead_code)]
    NotFound(String),
    #[allow(dead_code)]
    BadRequest(String),
}

impl RenderError {
    fn get_http_status(&self) -> Status {
        match self {
            RenderError::Internal(_) => Status::InternalServerError,
            RenderError::NotFound(_) => Status::NotFound,
            _ => Status::BadRequest,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            RenderError::Internal(s) => s.to_owned(),
            RenderError::NotFound(s) => s.to_owned(),
            _ => "Unknown error".to_owned(),
        }
    }
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}: {}",
            self.get_http_status(),
            self.get_error_message()
        )
    }
}

impl From<rocket::response::status::Custom<String>> for RenderError {
    fn from(e: rocket::response::status::Custom<String>) -> Self {
        RenderError::Internal(e.1)
    }
}

impl From<base_url::BaseUrlError> for RenderError {
    fn from(e: base_url::BaseUrlError) -> Self {
        match e {
            base_url::BaseUrlError::CannotBeBase => {
                RenderError::Internal("cannot be base URL".to_string())
            }
            base_url::BaseUrlError::ParseError(e) => {
                RenderError::Internal(format!("URL parse error: {:?}", e))
            }
        }
    }
}

impl From<anyhow::Error> for RenderError {
    fn from(e: anyhow::Error) -> Self {
        RenderError::Internal(format!("{:#?}\n{}", e, e.backtrace()))
    }
}

impl<'r> Responder<'r, 'static> for RenderError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = templates::Error {
            title: "Uh-oh".to_string(),
            url: None,
            error: self.to_string(),
        }
        .to_string();

        Ok(Response::build()
            .status(self.get_http_status())
            .header(ContentType::HTML)
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}
