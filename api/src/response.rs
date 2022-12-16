use std::fmt::Display;

use axum::{http::StatusCode, Json};
use serde::Serialize;

pub type Response<T> = Result<(StatusCode, Json<OkRsp<T>>), (StatusCode, Json<ErrorRsp>)>;

pub trait ResponseTrait<T> {
    fn to_error(self, code: StatusCode) -> Result<T, (StatusCode, Json<ErrorRsp>)>;
}

impl<T, E: Display> ResponseTrait<T> for Result<T, E> {
    fn to_error(self, code: StatusCode) -> Result<T, (StatusCode, Json<ErrorRsp>)> {
        self.map_err(|e| {
            (
                code,
                Json(ErrorRsp {
                    error: e.to_string(),
                }),
            )
        })
    }
}

#[derive(Serialize)]
pub struct ErrorRsp {
    error: String,
}

#[derive(Serialize)]
pub struct OkRsp<T> {
    ok: T,
}

pub fn error<E: Display>(code: StatusCode, error: E) -> (StatusCode, Json<ErrorRsp>) {
    (
        code,
        Json(ErrorRsp {
            error: error.to_string(),
        }),
    )
}

pub fn ok<T>(code: StatusCode, data: T) -> Response<T> {
    Ok((code, Json(OkRsp { ok: data })))
}
