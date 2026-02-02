//! Post API Routes
//!
//! This module provides HTTP handlers for blog post management.
//! Following RESTful principles with intuitive URI design.
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | GET | /posts | List posts with filters |
//! | GET | /posts/search | Search posts |
//! | POST | /posts | Create new post |
//! | GET | /posts/{id} | Get post details |
//! | PUT | /posts/{id} | Full update post |
//! | PATCH | /posts/{id} | Partial update (title, content, category, status) |
//! | DELETE | /posts/{id} | Delete post |
//! | GET | /posts/{id}/comments | Get post comments |
//! | POST | /posts/{id}/comments | Add comment to post |
//! | GET | /posts/{id}/tags | Get post tags |
//! | POST | /posts/{id}/tags | Add tag to post |
//! | DELETE | /posts/{id}/tags/{tag_id} | Remove tag from post |

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json, Router,
};
use domain::post::{CreatePost, SearchPostsRequest, UpdatePost};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::Claims,
    response::{helpers as resp, Pagination},
    state::AppState,
};
use domain::{POST_CREATE, USER_MANAGE};

/// Query parameters for listing posts
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    page: u64,
    /// Items per page
    #[serde(default = "default_per_page")]
    per_page: u64,
    /// Filter by author ID
    author: Option<String>,
    /// Filter by category ID
    category: Option<String>,
    /// Filter by tag ID
    tag: Option<String>,
    /// Filter by status: "published", "draft", or "all" (admin/owner only)
    #[serde(default = "default_status")]
    status: String,
}

/// Query parameters for searching posts
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// Search query string
    q: String,
    /// Page number
    #[serde(default = "default_page")]
    page: u64,
    /// Items per page
    #[serde(default = "default_per_page")]
    per_page: u64,
}

/// Request body for partial post update (PATCH)
#[derive(Debug, Deserialize)]
pub struct PatchPostRequest {
    /// Post title
    title: Option<String>,
    /// Post content
    content: Option<String>,
    /// Category ID (null to remove)
    category_id: Option<String>,
    /// Post status: "published" or "draft"
    status: Option<String>,
}

/// Request body for adding a tag to a post
#[derive(Debug, Deserialize)]
pub struct AddTagRequest {
    tag_id: String,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

fn default_status() -> String {
    "published".to_string()
}

/// Create post routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", axum::routing::get(list_posts))
        .route("/search", axum::routing::get(search_posts))
        .route("/{id}", axum::routing::get(get_post))
        .route("/{id}/comments", axum::routing::get(list_post_comments))
        .route("/{id}/tags", axum::routing::get(get_post_tags))
        // Protected routes
        .route("/", axum::routing::post(create_post))
        .route("/{id}", axum::routing::put(update_post))
        .route("/{id}", axum::routing::patch(patch_post))
        .route("/{id}", axum::routing::delete(delete_post))
        .route("/{id}/comments", axum::routing::post(create_comment))
        .route("/{id}/tags", axum::routing::post(add_post_tag))
        .route(
            "/{id}/tags/{tag_id}",
            axum::routing::delete(remove_post_tag),
        )
}

/// GET /posts
/// List posts with optional filtering
///
/// Query parameters:
/// - page: Page number (default: 1)
/// - per_page: Items per page (default: 20)
/// - author: Filter by author UUID
/// - category: Filter by category UUID
/// - tag: Filter by tag UUID
/// - status: "published", "draft", or "all" (admin/owner only)
async fn list_posts(
    State(state): State<AppState>,
    user: Option<Claims>,
    Query(params): Query<ListPostsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // Calculate offset from page
    let _offset = (params.page - 1) * params.per_page;

    // Check if user is admin
    let is_admin = user
        .as_ref()
        .is_some_and(|u| (u.permissions & USER_MANAGE) != 0);

    // Determine which posts to show based on status filter
    let show_drafts = params.status == "draft" || params.status == "all";

    // Get posts based on filters
    let posts = if let Some(author_id) = params.author {
        let author_uuid = Uuid::parse_str(&author_id)
            .map_err(|e| ApiError::Validation(format!("Invalid author ID: {}", e)))?;

        // Check if requesting own posts
        let is_own_posts = user
            .as_ref()
            .is_some_and(|u| Uuid::parse_str(&u.sub) == Ok(author_uuid));

        if show_drafts && (is_own_posts || is_admin) {
            state
                .post_service
                .list_by_user(author_uuid, Some(params.per_page))
                .await
                .map_err(ApiError::Domain)?
        } else {
            state
                .post_service
                .list_published_by_user(author_uuid, Some(params.per_page))
                .await
                .map_err(ApiError::Domain)?
        }
    } else if let Some(category_id) = params.category {
        let category_uuid = Uuid::parse_str(&category_id)
            .map_err(|e| ApiError::Validation(format!("Invalid category ID: {}", e)))?;
        state
            .post_service
            .list_by_category(category_uuid, Some(params.per_page))
            .await
            .map_err(ApiError::Domain)?
    } else if let Some(tag_id) = params.tag {
        let tag_uuid = Uuid::parse_str(&tag_id)
            .map_err(|e| ApiError::Validation(format!("Invalid tag ID: {}", e)))?;
        state
            .post_service
            .list_by_tag(tag_uuid, Some(params.per_page))
            .await
            .map_err(ApiError::Domain)?
    } else {
        // No specific filter - show based on status param and permissions
        match params.status.as_str() {
            "all" if is_admin => state
                .post_service
                .list_all(Some(params.per_page))
                .await
                .map_err(ApiError::Domain)?,
            "draft" if is_admin => state
                .post_service
                .list_all(Some(params.per_page))
                .await
                .map_err(ApiError::Domain)?
                .into_iter()
                .filter(|p| !p.is_published())
                .collect(),
            _ => state
                .post_service
                .list_published(Some(params.per_page))
                .await
                .map_err(ApiError::Domain)?,
        }
    };

    // TODO: Get total count for pagination
    let total = posts.len() as u64;

    let pagination = Pagination::new(params.page, params.per_page, total);
    Ok(resp::list(posts, pagination))
}

/// POST /posts
/// Create a new post (requires authentication)
async fn create_post(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreatePost>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, POST_CREATE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    // Verify user exists before creating post
    state
        .user_service
        .get(user_id)
        .await
        .map_err(|e| match e {
            domain::Error::NotFound(_) => {
                ApiError::Unauthorized("User not found. Please log in again.".to_string())
            }
            _ => ApiError::Domain(e),
        })?;

    let post = state
        .post_service
        .create(user_id, input.title, input.content)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::created(post))
}

/// GET /posts/{id}
/// Get a single post by ID
async fn get_post(
    State(state): State<AppState>,
    user: Option<Claims>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let post = state.post_service.get(id).await.map_err(ApiError::Domain)?;

    // Check visibility
    if post.is_published() {
        return Ok(resp::ok(post));
    }

    // Draft posts: only owner or admin can view
    match user {
        Some(current_user) => {
            let current_user_id = Uuid::parse_str(&current_user.sub)
                .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

            if current_user_id == post.user_id || (current_user.permissions & USER_MANAGE) != 0 {
                Ok(resp::ok(post))
            } else {
                Err(ApiError::NotFound("Post not found".to_string()))
            }
        }
        None => Err(ApiError::NotFound("Post not found".to_string())),
    }
}

/// PUT /posts/{id}
/// Full update of a post
async fn update_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePost>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .update(id, input.title, input.content, user_id, user.permissions)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(post))
}

/// PATCH /posts/{id}
/// Partial update of a post (title, content, category, or status)
async fn patch_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<PatchPostRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let mut post = state.post_service.get(id).await.map_err(ApiError::Domain)?;

    // Check ownership or admin permission
    let is_owner = post.user_id == user_id;
    let is_admin = (user.permissions & USER_MANAGE) != 0;

    if !is_owner && !is_admin {
        return Err(ApiError::Unauthorized(
            "You don't have permission to update this post".to_string(),
        ));
    }

    let has_category_update = input.category_id.is_some();
    let has_title_update = input.title.is_some();
    let has_content_update = input.content.is_some();
    let _has_status_update = input.status.is_some();

    // Handle status change (publish/unpublish)
    if let Some(ref status) = input.status {
        post = match status.as_str() {
            "published" => {
                domain::check_permission(user.permissions, domain::POST_PUBLISH)
                    .map_err(|e| ApiError::Unauthorized(e.to_string()))?;
                state
                    .post_service
                    .publish(id, user_id, user.permissions)
                    .await
                    .map_err(ApiError::Domain)?
            }
            "draft" => {
                domain::check_permission(user.permissions, domain::POST_PUBLISH)
                    .map_err(|e| ApiError::Unauthorized(e.to_string()))?;
                state
                    .post_service
                    .unpublish(id, user_id, user.permissions)
                    .await
                    .map_err(ApiError::Domain)?
            }
            _ => return Err(ApiError::Validation(format!("Invalid status: {}", status))),
        };
    }

    // Handle category change
    if let Some(ref category_id_str) = input.category_id {
        let category_id = if category_id_str.is_empty() {
            None
        } else {
            Some(
                Uuid::parse_str(category_id_str)
                    .map_err(|e| ApiError::Validation(format!("Invalid category ID: {}", e)))?,
            )
        };
        state
            .post_service
            .set_category(id, category_id, user_id, user.permissions)
            .await
            .map_err(ApiError::Domain)?;
    }

    // Handle title/content update
    if has_title_update || has_content_update {
        let title = input.title.unwrap_or_else(|| post.title.clone());
        let content = input.content.unwrap_or_else(|| post.content.clone());
        post = state
            .post_service
            .update(id, Some(title), Some(content), user_id, user.permissions)
            .await
            .map_err(ApiError::Domain)?;
    }

    // Refresh post data if we only changed category
    if has_category_update && !has_title_update && !has_content_update {
        post = state.post_service.get(id).await.map_err(ApiError::Domain)?;
    }

    Ok(resp::ok(post))
}

/// DELETE /posts/{id}
/// Delete a post
async fn delete_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .delete(id, user_id, user.permissions)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::no_content())
}

/// GET /posts/search
/// Search posts
async fn search_posts(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let request = SearchPostsRequest {
        query: params.q,
        limit: Some(params.per_page),
        offset: Some((params.page - 1) * params.per_page),
    };

    let response = state
        .post_service
        .search(request)
        .await
        .map_err(ApiError::Domain)?;

    let pagination = Pagination::new(params.page, params.per_page, response.total);
    Ok(resp::list(response.posts, pagination))
}

/// GET /posts/{id}/tags
/// Get tags for a post
async fn get_post_tags(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let tags = state
        .post_service
        .get_tags(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(tags))
}

/// POST /posts/{id}/tags
/// Add a tag to a post
async fn add_post_tag(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<AddTagRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let tag_id = Uuid::parse_str(&input.tag_id)
        .map_err(|e| ApiError::Validation(format!("Invalid tag ID: {}", e)))?;

    state
        .post_service
        .add_tag(id, tag_id, user_id, user.permissions)
        .await
        .map_err(ApiError::Domain)?;

    let tags = state
        .post_service
        .get_tags(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::created(tags))
}

/// DELETE /posts/{id}/tags/{tag_id}
/// Remove a tag from a post
async fn remove_post_tag(
    State(state): State<AppState>,
    user: Claims,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .remove_tag(id, tag_id, user_id, user.permissions)
        .await
        .map_err(ApiError::Domain)?;

    let tags = state
        .post_service
        .get_tags(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(tags))
}

/// GET /posts/{id}/comments
/// Get comments for a post
async fn list_post_comments(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let comments = state
        .comment_service
        .list_post_comments(id, 100)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(comments))
}

/// POST /posts/{id}/comments
/// Add a comment to a post
async fn create_comment(
    user: Claims,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let content = input
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ApiError::Validation("Content is required".to_string()))?;

    let comment = state
        .comment_service
        .create_comment(
            user_id,
            domain::CreateComment {
                post_id: id,
                content: content.to_string(),
            },
        )
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::created(comment))
}
