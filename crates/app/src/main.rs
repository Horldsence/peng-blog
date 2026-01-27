use api::{routes, AppState, AuthState};
use tower_http::services::ServeDir;
use infrastructure::{establish_connection, PostRepositoryImpl, UserRepositoryImpl};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "peng_blog=debug,tower_http=debug,axum=trace".into()),
        )
        .init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-this-secret-in-production".to_string());
    
    let db = establish_connection(&database_url).await?;

    let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(db));
    let auth_state = AuthState::new(&jwt_secret);
    
    let state = AppState::new(post_repo, user_repo, auth_state);

    let app = axum::Router::new()
        .nest("/api", routes())
        .nest_service("/", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
