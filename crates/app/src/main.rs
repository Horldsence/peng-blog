use api::{routes, AuthState};
use service::{PostService, UserService, SessionService, FileService, CommentService, StatsService};
use infrastructure::{establish_connection, Migrator, PostRepositoryImpl, UserRepositoryImpl, SessionRepositoryImpl, FileRepositoryImpl, CommentRepositoryImpl, StatsRepositoryImpl};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use infrastructure::MigratorTrait;

// Type alias for concrete AppState used in production
type AppState = api::AppState<
    PostRepositoryImpl,
    UserRepositoryImpl,
    SessionRepositoryImpl,
    FileRepositoryImpl,
    CommentRepositoryImpl,
    StatsRepositoryImpl,
>;

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
    let upload_dir = std::env::var("UPLOAD_DIR")
        .unwrap_or_else(|_| "./uploads".to_string());
    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    tracing::info!("DATABASE_URL: {}", database_url);
    let github_client_id = std::env::var("GITHUB_CLIENT_ID")
        .unwrap_or_else(|_| "".to_string());
    let github_client_secret = std::env::var("GITHUB_CLIENT_SECRET")
        .unwrap_or_else(|_| "".to_string());

    let db = establish_connection(&database_url).await?;

    // Run database migrations
    Migrator::up(&*db, None).await?;

    // Create upload directory if it doesn't exist
    tokio::fs::create_dir_all(&upload_dir).await?;

    // Create all repositories
    let post_repo = Arc::new(PostRepositoryImpl::new(Arc::clone(&db)));
    let user_repo = Arc::new(UserRepositoryImpl::new(Arc::clone(&db)));
    let session_repo = Arc::new(SessionRepositoryImpl::new(Arc::clone(&db)));
    let file_repo = Arc::new(FileRepositoryImpl::new(Arc::clone(&db)));
    let comment_repo = Arc::new(CommentRepositoryImpl::new(Arc::clone(&db)));
    let stats_repo = Arc::new(StatsRepositoryImpl::new(db));

    // Create all services
    let post_service = PostService::new(post_repo);
    let user_service = UserService::new(user_repo.clone());
    let session_service = SessionService::new(session_repo);
    let file_service = FileService::new(file_repo, upload_dir.clone(), base_url);
    let comment_service = CommentService::new(comment_repo, user_repo, github_client_id, github_client_secret);
    let stats_service = StatsService::new(stats_repo);
    let auth_state = AuthState::new(&jwt_secret);

    let state = AppState::new(
        post_service,
        user_service,
        session_service,
        file_service,
        comment_service,
        stats_service,
        auth_state,
        upload_dir,
    );

    let app = axum::Router::new()
        .nest("/api", routes::<
            PostRepositoryImpl,
            UserRepositoryImpl,
            SessionRepositoryImpl,
            FileRepositoryImpl,
            CommentRepositoryImpl,
            StatsRepositoryImpl,
        >())
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
