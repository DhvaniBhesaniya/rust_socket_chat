use std::sync::Arc;

use axum::{response::Html, routing::get, Router};
use hyper::StatusCode;
use rust_socket_chat::{get_rooms_list, on_connect, AppState};
use socketioxide::{
    extract::{SocketRef, State},
    SocketIo,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Starting rust socket.io chat server...");

    // create app state
    let app_state = Arc::new(AppState::new());

    // create socket io layer
    let (layer, io) = SocketIo::new_layer();

    // Register the main namespace handler
    io.ns("/", {
        let app_state = app_state.clone(); // typically Arc<AppState>
        move |socket: SocketRef| {
            let app_state = app_state.clone();
            async move {
                on_connect(socket, State((*app_state).clone())).await; // <-- Correct type
            }
        }
    });

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the Axum app
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/health", get(health_check))
        .route(
            "/api/rooms",
            get({
                let app_state = app_state.clone();
                move || get_rooms_list(State((*app_state).clone()))
            }),
        )
        .layer(cors)
        .layer(layer)
        .with_state(app_state);

    info!("Server starting on http://localhost:3000");
    info!("Chat interface available at http://localhost:3000");

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Serve the main HTML page
async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

// Health check endpoint
async fn health_check() -> Result<axum::Json<serde_json::Value>, StatusCode> {
    Ok(axum::Json(serde_json::json!({
        "status": "ok",
        "message": "Chat server is running"
    })))
}
