use crate::templates;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;

//#[derive(Error)]
#[derive(Debug, Clone)]
pub enum CustomError {
    //#[resp("{0}")]
    Internal(String),

    //#[resp("{0}")]
    NotFound(String),

    //#[resp("{0}")]
    BadRequest(String),
}

impl CustomError {
    fn get_http_status(&self) -> Status {
        match self {
            CustomError::Internal(_) => Status::InternalServerError,
            CustomError::NotFound(_) => Status::NotFound,
            _ => Status::BadRequest,
        }
    }

    fn get_error_message(&self) -> String {
        match self {
            CustomError::Internal(s) => s.to_owned(),
            CustomError::NotFound(s) => s.to_owned(),
            _ => "Unknown error".to_owned(),
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}: {}",
            self.get_http_status(),
            self.get_error_message()
        )
    }
}

impl From<rocket::response::status::Custom<std::string::String>> for CustomError {
    fn from(e: rocket::response::status::Custom<std::string::String>) -> Self {
        CustomError::Internal(e.1)
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
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
