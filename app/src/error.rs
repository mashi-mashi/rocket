use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::Request;
use sqlx::Error as SqlxError;
use std::io::Cursor;

#[derive(Debug)]
pub enum CustomError {
    DatabaseError(SqlxError),
    NotFound,
    // 他のエラーケースも追加できます
}

impl From<SqlxError> for CustomError {
    fn from(err: SqlxError) -> Self {
        CustomError::DatabaseError(err)
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, error_message) = match self {
            CustomError::DatabaseError(_) => (Status::InternalServerError, "Database error"),
            CustomError::NotFound => (Status::NotFound, "Resource not found"),
            // 他のエラーに応じて異なるステータスコードやメッセージを設定
        };

        Response::build()
            .status(status)
            .sized_body(error_message.len(), Cursor::new(error_message))
            .ok()
    }
}
