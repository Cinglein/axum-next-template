use axum::{
    response::{IntoResponse, Response},
    Error as AxumError, Json,
};
use hyper::StatusCode;
use sqlx::{migrate::MigrateError, Error as SqlxError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerErr {
    #[error("Error creating message stream")]
    MessageStreamErr(#[from] AxumError),
    #[error("Error setting up sql server")]
    SqlxErr(#[from] SqlxError),
    #[error("Error migrating sql")]
    SqlxMigrateErr(#[from] MigrateError),
}

impl IntoResponse for ServerErr {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}
