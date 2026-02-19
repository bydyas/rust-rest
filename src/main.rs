mod posts;
mod users;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};
use tracing_subscriber;

use crate::posts::{create_post, delete_post, get_post_by_id, get_posts, update_post};
use crate::users::create_user;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/posts", get(get_posts).post(create_post))
        .route(
            "/posts/{id}",
            get(get_post_by_id).put(update_post).delete(delete_post),
        )
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://localhost:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
