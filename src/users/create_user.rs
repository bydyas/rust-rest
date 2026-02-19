use axum::{Extension, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tracing::info;

use super::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

pub async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    info!("Creating the user: {new_user:?}.");

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        new_user.username,
        new_user.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
