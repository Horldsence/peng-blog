use api::{file_cache::FileCache, middleware::auth::set_jwt_secret, routes, AppState, AuthState};
use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use config::load_config;
use infrastructure::{
    establish_connection, CategoryRepositoryImpl, CommentRepositoryImpl, ConfigRepositoryImpl,
    FileRepositoryImpl, Migrator, MigratorTrait, PostRepositoryImpl, SessionRepositoryImpl,
    StatsRepositoryImpl, TagRepositoryImpl, UserRepositoryImpl,
};
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;
use service::{
    CategoryService, CommentService, ConfigService, FileService, PostService, SessionService,
    StatsService, TagService, UserService,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = load_config()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "peng_blog=debug,tower_http=debug,axum=trace".into()),
        )
        .init();

    tracing::info!("DATABASE_URL: {}", config.database.url);
    tracing::info!("ALLOW_REGISTRATION: {}", config.site.allow_registration);
    tracing::info!("DATABASE_URL env override: {:?}", config.database.url_env_override);
    tracing::info!("UPLOAD_DIR env override: {:?}", config.storage.upload_dir_env_override);
    tracing::info!("JWT_SECRET env override: {:?}", config.auth.jwt_secret_env_override);

    if let Some(true) = config.site.allow_registration_env_override {
        tracing::warn!("ALLOW_REGISTRATION is overridden by environment variable");
    }

    set_jwt_secret(config.auth.jwt_secret.clone());

    let db = establish_connection(&config.database.url).await?;
    Migrator::up(&*db, None).await?;

    tokio::fs::create_dir_all(&config.storage.upload_dir).await?;

    let bing_cache = FileCache::new(&config.storage.cache_dir)?;
    bing_cache.initialize().await?;

    let db_clone = Arc::clone(&db);
    let post_repo = Arc::new(PostRepositoryImpl::new(db_clone.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(db_clone.clone()));
    let session_repo = Arc::new(SessionRepositoryImpl::new(db_clone.clone()));
    let file_repo = Arc::new(FileRepositoryImpl::new(db_clone.clone()));
    let comment_repo = Arc::new(CommentRepositoryImpl::new(db_clone.clone()));
    let stats_repo = Arc::new(StatsRepositoryImpl::new(db_clone.clone()));
    let category_repo = Arc::new(CategoryRepositoryImpl::new(db_clone.clone()));
    let tag_repo = Arc::new(TagRepositoryImpl::new(db_clone));

    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let post_service = PostService::new(post_repo);
    let user_service = UserService::new(user_repo.clone(), config.site.allow_registration);
    let session_service = SessionService::new(session_repo);
    let file_service = FileService::new(file_repo, config.storage.upload_dir.clone(), base_url);
    let comment_service = CommentService::new(
        comment_repo,
        user_repo,
        config.github.client_id.clone(),
        config.github.client_secret.clone(),
    );
    let stats_service = StatsService::new(stats_repo);
    let category_service = CategoryService::new(category_repo);
    let tag_service = TagService::new(tag_repo);
    let config_repo = Arc::new(ConfigRepositoryImpl::new());
    let config_service = ConfigService::new(config_repo);
    let auth_state = AuthState::new(&config.auth.jwt_secret);

    let state = AppState::builder()
        .config(config.clone())
        .config_service(config_service)
        .post_service(post_service)
        .user_service(user_service)
        .session_service(session_service)
        .file_service(file_service)
        .comment_service(comment_service)
        .stats_service(stats_service)
        .category_service(category_service)
        .tag_service(tag_service)
        .auth_state(auth_state)
        .upload_dir(config.storage.upload_dir.clone())
        .bing_cache(bing_cache)
        .build();

    api::bing::start_bing_cache_refresh_task(state.clone()).await;

    let app = axum::Router::new()
        .nest("/api", routes())
        .fallback(frontend_handler)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);
    let listener =
        tokio::net::TcpListener::bind((config.server.host.as_str(), config.server.port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    tracing::info!("Frontend assets embedded in binary");
    tracing::info!(
        "API available at http://{}:{}/api",
        config.server.host,
        config.server.port
    );

    axum::serve(listener, app).await?;

    Ok(())
}

/// Embedded frontend static files (only in release builds)
#[cfg(not(debug_assertions))]
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

    #[cfg(not(debug_assertions))]
    {
        // Try to serve the embedded asset in release mode
        match FrontendAssets::get(asset_path) {
            Some(content) => {
                let mime = mime_guess::from_path(asset_path)
                    .first_or_octet_stream()
                    .to_string();

                return Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", mime)
                    .body(Body::from(content.data.to_vec()))
                    .unwrap();
            }
            None => {
                // If asset not found and it's not a file request (no extension or is a route),
                // serve index.html for SPA routing
                let has_extension = path.contains('.') && !path.ends_with('/');

                if !has_extension {
                    // Serve index.html for SPA routes
                    match FrontendAssets::get("index.html") {
                        Some(content) => {
                            return Response::builder()
                                .status(StatusCode::OK)
                                .header("content-type", "text/html; charset=utf-8")
                                .body(Body::from(content.data.to_vec()))
                                .unwrap();
                        }
                        None => {
                            // This shouldn't happen in release mode, but fallback just in case
                            return serve_from_filesystem(asset_path, true).await;
                        }
                    }
                }
            }
        }
    }

    // Debug mode or fallback: serve from filesystem
    let has_extension = path.contains('.') && !path.ends_with('/');
    serve_from_filesystem(asset_path, !has_extension).await
}

/// Fallback handler to serve from filesystem during development
async fn serve_from_filesystem(path: &str, is_route: bool) -> Response<Body> {
    let dist_path = std::path::Path::new("dist");

    if is_route {
        // Serve index.html for SPA routes
        let index_path = dist_path.join("index.html");
        if let Ok(content) = tokio::fs::read_to_string(&index_path).await {
            return Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html; charset=utf-8")
                .body(Body::from(content))
                .unwrap();
        }
    } else {
        // Serve static file
        let file_path = dist_path.join(path);
        if let Ok(content) = tokio::fs::read(&file_path).await {
            let mime = mime_guess::from_path(path)
                .first_or_octet_stream()
                .to_string();

            return Response::builder()
                .status(StatusCode::OK)
                .header("content-type", mime)
                .body(Body::from(content))
                .unwrap();
        }
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}
