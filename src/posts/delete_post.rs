use axum::{Extension, Json, extract::Path, http::StatusCode};
use sqlx::{Pool, Postgres};
use tracing::info;

pub async fn delete_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Deleting the post: {id}");

    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json!({
            "message": "Post deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
