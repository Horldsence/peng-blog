use api::{routes, AuthState};
use blog_service::{PostService, UserService};
use infrastructure::{establish_connection, PostRepositoryImpl, UserRepositoryImpl};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

// Type alias for concrete AppState used in production
type AppState = api::AppState<PostRepositoryImpl, UserRepositoryImpl>;

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
    let post_service = PostService::new(post_repo);
    let user_service = UserService::new(user_repo);
    let auth_state = AuthState::new(&jwt_secret);

    let state = AppState::new(post_service, user_service, auth_state);

    let app = axum::Router::new()
        .nest("/api", routes::<PostRepositoryImpl, UserRepositoryImpl>())
        .nest_service("/", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
