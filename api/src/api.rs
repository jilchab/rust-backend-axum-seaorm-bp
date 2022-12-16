use crate::context::Ctx;
use crate::routes::*;

use axum::{
    routing::{get, put},
    Router,
};

pub async fn start_server() -> eyre::Result<()> {
    let db = Ctx::new().await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(users::users_get).post(users::users_post))
        .route(
            "/users/:id",
            put(users::users_put).delete(users::users_delete),
        )
        .with_state(db);

    let server_url = format!(
        "{}:{}",
        dotenvy::var("SERVER_URL")?,
        dotenvy::var("SERVER_PORT")?
    );

    axum::Server::bind(&server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
