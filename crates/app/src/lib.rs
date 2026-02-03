use api::{file_cache::FileCache, middleware::auth::set_jwt_secret, routes, AppState, AuthState};
use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use infrastructure::{
    establish_connection, CategoryRepositoryImpl, CommentRepositoryImpl, FileRepositoryImpl,
    Migrator, MigratorTrait, PostRepositoryImpl, SessionRepositoryImpl, StatsRepositoryImpl,
    TagRepositoryImpl, UserRepositoryImpl,
};
use rust_embed::RustEmbed;
use service::{
    CategoryService, CommentService, FileService, PostService, SessionService, StatsService,
    TagService, UserService,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Start the blog server
///
/// This function initializes the database, runs migrations, and starts the web server.
/// It uses environment variables for configuration:
/// - DATABASE_URL: PostgreSQL database connection string (default: required)
/// - JWT_SECRET: Secret key for JWT tokens (default: "change-this-secret-in-production")
/// - UPLOAD_DIR: Directory for file uploads (default: "./uploads")
/// - BASE_URL: Base URL for the application (default: "http://localhost:3000")
/// - CACHE_DIR: Directory for file-based caching (default: "./cache")
/// - GITHUB_CLIENT_ID: GitHub OAuth client ID (optional)
/// - GITHUB_CLIENT_SECRET: GitHub OAuth client secret (optional)
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "peng_blog=debug,tower_http=debug,axum=trace".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/peng_blog".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-this-secret-in-production".to_string());

    // Set the global JWT secret for token validation
    set_jwt_secret(jwt_secret.clone());

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    tracing::info!("DATABASE_URL: {}", database_url);
    let github_client_id = std::env::var("GITHUB_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let github_client_secret =
        std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());

    let db = establish_connection(&database_url).await?;

    // Run database migrations
    Migrator::up(&*db, None).await?;

    tokio::fs::create_dir_all(&upload_dir).await?;

    let cache_dir = std::env::var("CACHE_DIR").unwrap_or_else(|_| "./cache".to_string());
    let bing_cache = FileCache::new(&cache_dir)?;
    bing_cache.initialize().await?;

    // Create all repositories
    let db_clone = Arc::clone(&db);
    let post_repo = Arc::new(PostRepositoryImpl::new(db_clone.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(db_clone.clone()));
    let session_repo = Arc::new(SessionRepositoryImpl::new(db_clone.clone()));
    let file_repo = Arc::new(FileRepositoryImpl::new(db_clone.clone()));
    let comment_repo = Arc::new(CommentRepositoryImpl::new(db_clone.clone()));
    let stats_repo = Arc::new(StatsRepositoryImpl::new(db_clone.clone()));
    let category_repo = Arc::new(CategoryRepositoryImpl::new(db_clone.clone()));
    let tag_repo = Arc::new(TagRepositoryImpl::new(db_clone));

    // Create all services
    let post_service = PostService::new(post_repo);
    let user_service = UserService::new(user_repo.clone());
    let session_service = SessionService::new(session_repo);
    let file_service = FileService::new(file_repo, upload_dir.clone(), base_url);
    let comment_service = CommentService::new(
        comment_repo,
        user_repo,
        github_client_id,
        github_client_secret,
    );
    let stats_service = StatsService::new(stats_repo);
    let category_service = CategoryService::new(category_repo);
    let tag_service = TagService::new(tag_repo);
    let auth_state = AuthState::new(&jwt_secret);

    let state = AppState::builder()
        .post_service(post_service)
        .user_service(user_service)
        .session_service(session_service)
        .file_service(file_service)
        .comment_service(comment_service)
        .stats_service(stats_service)
        .category_service(category_service)
        .tag_service(tag_service)
        .auth_state(auth_state)
        .upload_dir(upload_dir)
        .bing_cache(bing_cache)
        .build();

    api::bing::start_bing_cache_refresh_task(state.clone()).await;

    let app = axum::Router::new()
        .nest("/api", routes())
        .fallback(frontend_handler)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    tracing::info!("Frontend assets embedded in binary");
    tracing::info!("API available at http://localhost:3000/api");

    axum::serve(listener, app).await?;

    Ok(())
}

/// Embedded frontend static files
#[derive(RustEmbed)]
#[folder = "../../dist/"]
struct FrontendAssets;

/// Handler for serving frontend assets
async fn frontend_handler(req: Request) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    // Skip API routes
    if path.starts_with("/api") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap();
    }

    // Remove leading slash for asset lookup
    let asset_path = path.trim_start_matches('/');
    let asset_path = if asset_path.is_empty() {
        "index.html"
    } else {
        asset_path
    };

    // Try to serve the embedded asset
    match FrontendAssets::get(asset_path) {
        Some(content) => {
            let mime = mime_guess::from_path(asset_path)
                .first_or_octet_stream()
                .to_string();

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", mime)
                .body(Body::from(content.data.to_vec()))
                .unwrap()
        }
        None => {
            // If asset not found and it's not a file request (no extension or is a route),
            // serve index.html for SPA routing
            let has_extension = path.contains('.') && !path.ends_with('/');

            if !has_extension {
                // Serve index.html for SPA routes
                match FrontendAssets::get("index.html") {
                    Some(content) => Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "text/html; charset=utf-8")
                        .body(Body::from(content.data.to_vec()))
                        .unwrap(),
                    None => {
                        // Fallback to file system for development
                        serve_from_filesystem(asset_path, true).await
                    }
                }
            } else {
                // Fallback to file system for development
                serve_from_filesystem(asset_path, false).await
            }
        }
    }
}

/// Fallback handler to serve from filesystem during development
async fn serve_from_filesystem(_path: &str, is_route: bool) -> Response<Body> {
    if is_route {
        // Serve index.html for SPA routes
        let index_path = std::path::Path::new("dist").join("index.html");
        if let Ok(content) = tokio::fs::read_to_string(&index_path).await {
            return Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html; charset=utf-8")
                .body(Body::from(content))
                .unwrap();
        }
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}
