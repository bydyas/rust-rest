use axum::{Extension, Json, extract::Path, http::StatusCode};
use sqlx::{Pool, Postgres};
use tracing::info;

use super::Post;

pub async fn get_post_by_id(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    info!("Getting post by id {id}");

    let post = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, body FROM posts WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(post))
}
