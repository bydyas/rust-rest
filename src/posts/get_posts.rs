use axum::{Extension, Json, http::StatusCode};
use sqlx::{Pool, Postgres};
use tracing::info;

use super::Post;

pub async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    info!("Getting posts");

    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}
