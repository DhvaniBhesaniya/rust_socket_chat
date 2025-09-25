use std::{
    collections::HashMap,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
};

use axum::{body::Body, http::Response, response::IntoResponse, routing::get, Router};
use lazy_static::lazy_static;
use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::Sid,
    SocketIo,
};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

// Shared global state for mapping user connections (not used in this file, but retained)
lazy_static! {
    static ref CONN_USER: Arc<RwLock<HashMap<String, Arc<RwLock<Vec<String>>>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

/// Handles socket events for namespace `/chat`
fn socket_receive_and_send(socket: SocketRef) {
    info!(
        "Socket connected with id: {} and namespace: {}",
        socket.id,
        socket.ns()
    );

    // Authentication event
    socket.on("auth", |socket: SocketRef, Data::<Value>(data)| {
        info!("Auth event received: {:?}", data);
        if let Err(e) = socket.emit("auth", &data) {
            error!("Failed to emit auth: {:?}", e);
        }
    });

    // Join a room
    socket.on("join_room", |socket: SocketRef, Data::<Value>(data)| {
        if let Some(room_name) = data.get("room").and_then(|r| r.as_str()) {
            let room_name = room_name.to_owned();
            info!("Joining room: {}", room_name);
            match socket.join(room_name.clone()) {
                Ok(_) => {
                    info!("Socket {} joined room {}", socket.id, room_name);
                    if let Err(e) = socket.emit("joined_room", "You have joined the ROOM") {
                        error!("Emit error in join_room: {:?}", e);
                    }
                }
                Err(e) => error!("Failed to join room {}: {:?}", room_name, e),
            }
        } else {
            error!("No 'room' key found in join_room data: {:?}", data);
        }
    });

    // Leave a room
    socket.on("leave_room", |socket: SocketRef, Data::<Value>(data)| {
        if let Some(room_name) = data.get("room").and_then(|r| r.as_str()) {
            let room_name = room_name.to_owned();
            match socket.leave(room_name.clone()) {
                Ok(_) => {
                    info!("Socket {} left room {}", socket.id, room_name);
                    if let Err(e) = socket.emit("left_room", &serde_json::json!({ "room": room_name })) {
                        error!("Emit error in leave_room: {:?}", e);
                    }
                }
                Err(e) => error!("Failed to leave room {}: {:?}", room_name, e),
            }
        } else {
            error!("No 'room' key found in leave_room data: {:?}", data);
        }
    });

    // Handle incoming chat messages
    socket.on("message", |socket: SocketRef, Data::<Value>(data)| {
        info!("Received message: {:?}", data);
        if let Some(room) = data.get("room").and_then(|r| r.as_str()) {
            if let Err(e) = socket.to(room.to_owned()).emit("message", &data) {
                error!("Error emitting to room {}: {:?}", room, e);
            }
        } else {
            if let Err(e) = socket.broadcast().emit("message", &data) {
                error!("Error broadcasting message: {:?}", e);
            }
        }
    });

    // Simple broadcast to everyone except sender
    socket.on("test", |socket: SocketRef, Data::<Value>(data)| {
        info!("Broadcasting test message: {:?}", data);
        if let Err(e) = socket.broadcast().emit("test", &data) {
            error!("Broadcast error on test event: {:?}", e);
        }
    });

    // Emit message to all clients in a specific room
    socket.on("emit_to_room", |socket: SocketRef, Data::<Value>(data)| {
        info!("Broadcast to room requested: {:?}", data);
        if let Some(room) = data.get("room").and_then(|r| r.as_str()) {
            if let Err(e) = socket.to(room.to_owned()).broadcast().emit("message", &data) {
                error!("Failed to emit to room {}: {:?}", room, e);
            }
        } else {
            error!("Room not specified in emit_to_room data: {:?}", data);
        }
    });

    // Send message to a specific socket ID
    socket.on("send_to_socket", move |io: SocketIo, Data::<Value>(data)| {
        if let Some(target_socket_id) = data.get("socket_id").and_then(|v| v.as_str()) {
            match Sid::from_str(target_socket_id) {
                Ok(sid) => {
                    let ns = match io.of("/chat") {
                        Some(ns) => ns,
                        None => {
                            error!("Namespace /chat not found");
                            return;
                        }
                    };

                    if let Some(target_socket) = ns.get_socket(sid) {
                        if let Err(e) = target_socket.emit("message", &data) {
                            error!("Emit to socket {} failed: {:?}", target_socket_id, e);
                        }
                    } else {
                        error!("Socket with ID {} not found", target_socket_id);
                    }
                }
                Err(e) => {
                    error!("Invalid socket_id format: {:?}, error: {:?}", target_socket_id, e);
                }
            }
        } else {
            error!("No 'socket_id' field in data: {:?}", data);
        }
    });
}

/// Handles socket events for root namespace `/`
fn socket_setup(socket: SocketRef) {
    info!(
        "Socket connected with id: {} and namespace: {}",
        socket.id,
        socket.ns()
    );

    // Basic authentication echo
    socket.on("auth", |socket: SocketRef, Data::<Value>(data)| {
        info!("Auth event received: {:?}", data);
        if let Err(e) = socket.emit("auth", &data) {
            error!("Emit error in root auth: {:?}", e);
        }
    });

    // Basic test message
    socket.on("message", |socket: SocketRef, Data::<Value>(data)| {
        info!("Message received on root namespace: {:?}", data);
        if let Err(e) = socket.emit("new", "HELLOOO") {
            error!("Emit error on root message: {:?}", e);
        }
    });
}

/// Entry point of the application
#[tokio::main]
async fn main() {
    // Initialize logging subscriber
    tracing::subscriber::set_global_default(FmtSubscriber::default())
        .expect("Failed to set tracing subscriber");

    // Create Socket.io layer and handler
    let (layer, io) = SocketIo::new_layer();

    // Register namespaces and handlers
    io.ns("/", socket_setup);
    io.ns("/chat", socket_receive_and_send);

    // Set up Axum router and apply socket layer
    let app = Router::new().route("/", get(hello)).layer(layer);

    let addr = SocketAddr::from(([192,168,0,24], 3000));
    info!("Server running on: {}", addr);

    // Start TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    // Serve the Axum app
    axum::serve(listener, app)
        .await
        .expect("Axum server crashed");
}

/// Simple test route
#[axum::debug_handler]
async fn hello() -> Response<Body> {
    "Hello".to_string().into_response()
}