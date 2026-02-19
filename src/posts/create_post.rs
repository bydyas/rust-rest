use axum::{Extension, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tracing::info;

use super::Post;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
}

pub async fn create_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    info!("Creating the post: {new_post:?}");

    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, title, body, user_id",
        new_post.user_id,
        new_post.title,
        new_post.body
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}
