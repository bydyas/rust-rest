use axum::{Extension, Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tracing::info;

use super::Post;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
}

pub async fn update_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(updated_post): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    info!("Updating the post: {updated_post:?}");

    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2, user_id = $3 WHERE id = $4 RETURNING id, user_id, title, body",
        updated_post.title,
        updated_post.body,
        updated_post.user_id,
        id
    )
    .fetch_one(&pool)
    .await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
