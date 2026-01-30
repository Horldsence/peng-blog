//! File API Routes
//!
//! This module provides HTTP handlers for file management.
//! Files are uploaded by users and can be downloaded later.
//!
//! Design Principles:
//! - Simple RESTful endpoints
//! - Direct file download (no streaming complexity)
//! - Upload validation in service layer
//! - No special cases - all files follow the same rules

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use domain::UploadFile;
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::Claims,
    state::AppState,
    PostRepository,
    UserRepository,
    SessionRepository,
    FileRepository,
    CommentRepository,
    StatsRepository,
    CategoryRepository,
    TagRepository,
};

// ============================================================================
// Routes
// ============================================================================
pub fn routes<PR, UR, SR, FR, CR, STR, CTR, TR>() -> Router<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    Router::new()
        // POST /api/files - Upload a file
        .route("/", axum::routing::post(upload_file))
        // GET /api/files/{id} - Get file info
        .route("/{id}", axum::routing::get(get_file))
        // GET /api/files/{id}/download - Download file
        .route("/{id}/download", axum::routing::get(download_file))
        // GET /api/files - List user's files
        .route("/", axum::routing::get(list_files))
        // DELETE /api/files/{id} - Delete a file
        .route("/{id}", axum::routing::delete(delete_file))
}

// ============================================================================
// Handlers
// ============================================================================

/// POST /api/files
/// Upload a new file
///
/// Request body: multipart/form-data with file field
/// Response: FileResponse with file metadata
pub async fn upload_file<PR, UR, SR, FR, CR, STR, CTR, TR>(
    user: Claims,
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    mut multipart: axum_extra::extract::Multipart,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| ApiError::Internal(format!("Failed to read multipart: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let filename = field.file_name()
                .unwrap_or("unknown")
                .to_string();
            let content_type = field.content_type()
                .unwrap_or("application/octet-stream")
                .to_string();

            let data = field.bytes()
                .await
                .map_err(|e| ApiError::Internal(format!("Failed to read file data: {}", e)))?
                .to_vec();

            let upload = UploadFile {
                filename,
                content_type,
                data,
            };

            let response = state
                .file_service
                .upload_file(user_id, upload)
                .await
                .map_err(ApiError::Domain)?;

            return Ok((StatusCode::CREATED, Json(response)));
        }
    }

    Err(ApiError::Validation("No file found in request".to_string()))
}

/// GET /api/files/:id
/// Get file metadata
pub async fn get_file<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let file_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid file ID: {}", e)))?;

    let response = state
        .file_service
        .get_file(file_id)
        .await
        .map_err(ApiError::Domain)?;

    match response {
        Some(file) => Ok((StatusCode::OK, Json(file))),
        None => Err(ApiError::NotFound("File not found".to_string())),
    }
}

/// GET /api/files/:id/download
/// Download file content
pub async fn download_file<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let file_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid file ID: {}", e)))?;

    let file_opt = state
        .file_service
        .get_file(file_id)
        .await
        .map_err(ApiError::Domain)?;

    let file = file_opt.ok_or_else(|| ApiError::NotFound("File not found".to_string()))?;

    // Read file from disk
    let file_path = format!("{}/{}",
        state.upload_dir.trim_end_matches('/'),
        file.filename
    );

    let file_content = tokio::fs::read(&file_path)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to read file: {}", e)))?;

    let headers = [
        (header::CONTENT_TYPE, file.content_type),
        (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", file.original_filename)),
    ];

    Ok((headers, file_content))
}

/// GET /api/files?limit=50
/// List files uploaded by the current user
pub async fn list_files<PR, UR, SR, FR, CR, STR, CTR, TR>(
    user: Claims,
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let limit = 50; // Default limit

    let files = state
        .file_service
        .list_files(user_id, limit)
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::OK, Json(files)))
}

/// DELETE /api/files/:id
/// Delete a file
pub async fn delete_file<PR, UR, SR, FR, CR, STR, CTR, TR>(
    user: Claims,
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let file_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid file ID: {}", e)))?;
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .file_service
        .delete_file(file_id, user_id)
        .await
        .map_err(|e| match e {
            domain::Error::NotFound(msg) => ApiError::NotFound(msg),
            domain::Error::Validation(msg) => ApiError::Validation(msg),
            _ => ApiError::Domain(e),
        })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "File deleted successfully" })),
    ))
}
