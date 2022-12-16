use crate::context::*;
use crate::response::*;

use entity::*;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseBackend, EntityTrait, Statement};
use serde::Deserialize;

pub async fn users_get(
    Query(params): Query<UserParams>,
    State(ctx): State<Ctx>,
) -> Response<Vec<user::Model>> {
    let limit = params.limit.unwrap_or(20).min(50);
    let offset = params.offset.unwrap_or_default();

    let pages = User::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"SELECT "user"."id", "user"."name", "user"."email" FROM "user" LIMIT $1, $2"#,
            vec![offset.into(), limit.into()],
        ))
        .all(&ctx.db)
        .await
        .to_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    ok(StatusCode::OK, pages)
}

pub async fn users_post(
    State(ctx): State<Ctx>,
    Json(body): Json<user::Model>,
) -> Response<user::Model> {
    let active_user = user::ActiveModel {
        id: NotSet,
        ..body.into()
    };

    let user = active_user
        .insert(&ctx.db)
        .await
        .to_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    ok(StatusCode::CREATED, user)
}

pub async fn users_put(
    State(ctx): State<Ctx>,
    Path(id): Path<i32>,
    Json(body): Json<user::OptionModel>,
) -> Response<user::Model> {
    let Some(user) = User::find_by_id(id).one(&ctx.db).await.to_error(StatusCode::INTERNAL_SERVER_ERROR)? else {
        return Err(error(StatusCode::NOT_FOUND, format!("User not found with id {}", id)));
    };

    let active_user = body.get_changes(user);

    active_user
        .clone()
        .update(&ctx.db)
        .await
        .to_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    ok(StatusCode::OK, active_user.try_into().unwrap())
}

pub async fn users_delete(State(ctx): State<Ctx>, Path(id): Path<i32>) -> Response<()> {
    User::delete_by_id(id)
        .exec(&ctx.db)
        .await
        .to_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    ok(StatusCode::OK, ())
}

#[derive(Debug, Deserialize)]
pub struct UserParams {
    limit: Option<u64>,
    offset: Option<u64>,
}
