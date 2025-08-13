use axum::{
    routing::{get, post},
    http::{Method, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use log::{info, error};
use serde::{Deserialize, Serialize};

// Import core logic modules and types
use claudia_lib::core_logic::claude::{self as core_claude, Project};

// Define a simple error response struct
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

// Handler for get_home_directory
async fn get_home_directory_handler() -> impl IntoResponse {
    match core_claude::get_home_directory().await {
        Ok(path) => Json(path).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { message: e })).into_response(),
    }
}

// Handler for list_projects
async fn list_projects_handler() -> impl IntoResponse {
    match core_claude::list_projects().await {
        Ok(projects) => Json(projects).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { message: e })).into_response(),
    }
}

// Request body for create_project
#[derive(Deserialize)]
struct CreateProjectRequest {
    path: String,
}

// Handler for create_project
async fn create_project_handler(Json(payload): Json<CreateProjectRequest>) -> impl IntoResponse {
    match core_claude::create_project(payload.path).await {
        Ok(project) => Json(project).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { message: e })).into_response(),
    }
}

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    info!("Starting Axum web server...");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any); // TODO: Restrict origin in production

    let app = Router::new()
        .route("/get_home_directory", get(get_home_directory_handler))
        .route("/list_projects", get(list_projects_handler))
        .route("/create_project", post(create_project_handler))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // Listen on port 3000
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}