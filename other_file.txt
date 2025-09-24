# rust_socket_chat
rust_socket_chat



# Project structure
project_structure = """
Socket Chat Application Structure:
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── models.rs
│   ├── handlers.rs
│   └── state.rs
├── templates/
│   └── index.html
├── static/
│   ├── script.js
│   └── style.css
└── Cargo.toml
"""





# Create models.rs - Data structures for our chat application
models_rs = '''use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub room: String,
    pub socket_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub username: String,
    pub message: String,
    pub room: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRoomData {
    pub room: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageData {
    pub message: String,
    pub room: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserJoinedData {
    pub username: String,
    pub room: String,
    pub user_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLeftData {
    pub username: String,
    pub room: String,
    pub user_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomUsersData {
    pub users: Vec<String>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingData {
    pub username: String,
    pub room: String,
    pub is_typing: bool,
}

impl ChatMessage {
    pub fn new(username: String, message: String, room: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            message,
            room,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl User {
    pub fn new(username: String, room: String, socket_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            room,
            socket_id,
        }
    }
}'''

# Create state.rs - Application state management
state_rs = '''use crate::models::{User, ChatMessage};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::RwLock as TokioRwLock;

#[derive(Debug, Clone)]
pub struct AppState {
    // Users in rooms: room_name -> Vec<User>
    pub rooms: Arc<TokioRwLock<HashMap<String, Vec<User>>>>,
    // Message history: room_name -> Vec<ChatMessage>
    pub messages: Arc<TokioRwLock<HashMap<String, Vec<ChatMessage>>>>,
    // Socket ID to User mapping
    pub socket_users: Arc<TokioRwLock<HashMap<String, User>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(TokioRwLock::new(HashMap::new())),
            messages: Arc::new(TokioRwLock::new(HashMap::new())),
            socket_users: Arc::new(TokioRwLock::new(HashMap::new())),
        }
    }

    // Add user to room
    pub async fn add_user_to_room(&self, user: User) {
        let mut rooms = self.rooms.write().await;
        let mut socket_users = self.socket_users.write().await;
        
        // Remove user from previous room if exists
        if let Some(old_user) = socket_users.get(&user.socket_id) {
            if let Some(room_users) = rooms.get_mut(&old_user.room) {
                room_users.retain(|u| u.socket_id != user.socket_id);
                if room_users.is_empty() {
                    rooms.remove(&old_user.room);
                }
            }
        }
        
        // Add user to new room
        rooms.entry(user.room.clone()).or_insert_with(Vec::new).push(user.clone());
        socket_users.insert(user.socket_id.clone(), user);
    }

    // Remove user by socket ID
    pub async fn remove_user(&self, socket_id: &str) -> Option<User> {
        let mut rooms = self.rooms.write().await;
        let mut socket_users = self.socket_users.write().await;
        
        if let Some(user) = socket_users.remove(socket_id) {
            if let Some(room_users) = rooms.get_mut(&user.room) {
                room_users.retain(|u| u.socket_id != socket_id);
                if room_users.is_empty() {
                    rooms.remove(&user.room);
                }
            }
            Some(user)
        } else {
            None
        }
    }

    // Get users in a room
    pub async fn get_room_users(&self, room_name: &str) -> Vec<User> {
        let rooms = self.rooms.read().await;
        rooms.get(room_name).cloned().unwrap_or_default()
    }

    // Get user by socket ID
    pub async fn get_user(&self, socket_id: &str) -> Option<User> {
        let socket_users = self.socket_users.read().await;
        socket_users.get(socket_id).cloned()
    }

    // Add message to room
    pub async fn add_message(&self, message: ChatMessage) {
        let mut messages = self.messages.write().await;
        messages.entry(message.room.clone()).or_insert_with(Vec::new).push(message);
    }

    // Get messages for a room
    pub async fn get_room_messages(&self, room_name: &str) -> Vec<ChatMessage> {
        let messages = self.messages.read().await;
        messages.get(room_name).cloned().unwrap_or_default()
    }

    // Get all rooms with user counts
    pub async fn get_rooms_info(&self) -> HashMap<String, usize> {
        let rooms = self.rooms.read().await;
        rooms.iter().map(|(name, users)| (name.clone(), users.len())).collect()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}'''

# Save to CSV files
df_models = pd.DataFrame([["models.rs", models_rs]])
df_models.columns = ["File", "Content"]
df_models.to_csv("models_rs.csv", index=False)

df_state = pd.DataFrame([["state.rs", state_rs]])
df_state.columns = ["File", "Content"]
df_state.to_csv("state_rs.csv", index=False)

print("✅ Created models.rs and state.rs")
print("📁 Models file contains data structures for User, ChatMessage, and various event data")
print("📁 State file contains AppState for managing users, rooms, and messages")





# Create handlers.rs - Socket event handlers
handlers_rs = '''use crate::models::*;
use crate::state::AppState;
use socketioxide::extract::{Data, SocketRef, State};
use tracing::{info, warn, error};
use std::time::{SystemTime, UNIX_EPOCH};

// Handle client connection
pub async fn on_connect(socket: SocketRef, app_state: State<AppState>) {
    info!("Client connected: {}", socket.id);
    
    // Send available rooms to the connected client
    let rooms_info = app_state.get_rooms_info().await;
    socket.emit("rooms_list", &rooms_info).ok();
    
    // Handle join room event
    socket.on("join_room", {
        let app_state = app_state.clone();
        move |socket: SocketRef, Data(data): Data<JoinRoomData>, app_state: State<AppState>| {
            let app_state = app_state.clone();
            async move {
                handle_join_room(socket, data, app_state).await;
            }
        }
    });
    
    // Handle send message event
    socket.on("send_message", {
        let app_state = app_state.clone();
        move |socket: SocketRef, Data(data): Data<SendMessageData>, app_state: State<AppState>| {
            let app_state = app_state.clone();
            async move {
                handle_send_message(socket, data, app_state).await;
            }
        }
    });
    
    // Handle typing events
    socket.on("typing", {
        let app_state = app_state.clone();
        move |socket: SocketRef, app_state: State<AppState>| {
            let app_state = app_state.clone();
            async move {
                handle_typing(socket, app_state, true).await;
            }
        }
    });
    
    socket.on("stop_typing", {
        let app_state = app_state.clone();
        move |socket: SocketRef, app_state: State<AppState>| {
            let app_state = app_state.clone();
            async move {
                handle_typing(socket, app_state, false).await;
            }
        }
    });
    
    // Handle disconnect
    socket.on_disconnect({
        let app_state = app_state.clone();
        move |socket: SocketRef, app_state: State<AppState>| {
            let app_state = app_state.clone();
            async move {
                handle_disconnect(socket, app_state).await;
            }
        }
    });
}

// Handle user joining a room
async fn handle_join_room(socket: SocketRef, data: JoinRoomData, app_state: State<AppState>) {
    let socket_id = socket.id.to_string();
    
    info!("User {} joining room: {}", data.username, data.room);
    
    // Create new user
    let user = User::new(data.username.clone(), data.room.clone(), socket_id.clone());
    
    // Add user to room
    app_state.add_user_to_room(user.clone()).await;
    
    // Join the socket.io room
    socket.join(&data.room).ok();
    
    // Send room history to the user
    let messages = app_state.get_room_messages(&data.room).await;
    socket.emit("room_messages", &messages).ok();
    
    // Get updated room users
    let room_users = app_state.get_room_users(&data.room).await;
    let user_count = room_users.len();
    let usernames: Vec<String> = room_users.iter().map(|u| u.username.clone()).collect();
    
    // Notify user they joined successfully
    socket.emit("joined_room", &JoinRoomData {
        room: data.room.clone(),
        username: data.username.clone(),
    }).ok();
    
    // Send updated user list to all users in the room
    let room_users_data = RoomUsersData {
        users: usernames,
        count: user_count,
    };
    socket.to(&data.room).emit("room_users_updated", &room_users_data).ok();
    socket.emit("room_users_updated", &room_users_data).ok();
    
    // Notify others in the room that a user joined
    let user_joined_data = UserJoinedData {
        username: data.username.clone(),
        room: data.room.clone(),
        user_count,
    };
    socket.to(&data.room).emit("user_joined", &user_joined_data).ok();
    
    // Create and broadcast system message
    let system_message = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        username: "System".to_string(),
        message: format!("{} joined the room", data.username),
        room: data.room.clone(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    
    app_state.add_message(system_message.clone()).await;
    socket.to(&data.room).emit("new_message", &system_message).ok();
    
    // Update rooms list for all clients
    let rooms_info = app_state.get_rooms_info().await;
    socket.broadcast().emit("rooms_list", &rooms_info).ok();
}

// Handle sending a message
async fn handle_send_message(socket: SocketRef, data: SendMessageData, app_state: State<AppState>) {
    let socket_id = socket.id.to_string();
    
    // Get user info
    if let Some(user) = app_state.get_user(&socket_id).await {
        info!("Message from {}: {}", user.username, data.message);
        
        // Validate that user is in the room they're trying to send to
        if user.room != data.room {
            warn!("User {} tried to send message to room {} but is in room {}", 
                  user.username, data.room, user.room);
            return;
        }
        
        // Create message
        let message = ChatMessage::new(user.username, data.message, data.room.clone());
        
        // Store message
        app_state.add_message(message.clone()).await;
        
        // Broadcast message to all users in the room (including sender)
        socket.within(&data.room).emit("new_message", &message).ok();
    } else {
        error!("Received message from unknown user: {}", socket_id);
    }
}

// Handle typing indicator
async fn handle_typing(socket: SocketRef, app_state: State<AppState>, is_typing: bool) {
    let socket_id = socket.id.to_string();
    
    if let Some(user) = app_state.get_user(&socket_id).await {
        let typing_data = TypingData {
            username: user.username,
            room: user.room.clone(),
            is_typing,
        };
        
        // Broadcast typing status to others in the room (excluding sender)
        socket.to(&user.room).emit("user_typing", &typing_data).ok();
    }
}

// Handle user disconnect
async fn handle_disconnect(socket: SocketRef, app_state: State<AppState>) {
    let socket_id = socket.id.to_string();
    info!("Client disconnected: {}", socket_id);
    
    // Remove user and get their info
    if let Some(user) = app_state.remove_user(&socket_id).await {
        info!("User {} left room: {}", user.username, user.room);
        
        // Get updated room users
        let room_users = app_state.get_room_users(&user.room).await;
        let user_count = room_users.len();
        let usernames: Vec<String> = room_users.iter().map(|u| u.username.clone()).collect();
        
        // Send updated user list to remaining users in the room
        if !room_users.is_empty() {
            let room_users_data = RoomUsersData {
                users: usernames,
                count: user_count,
            };
            socket.to(&user.room).emit("room_users_updated", &room_users_data).ok();
            
            // Notify others that user left
            let user_left_data = UserLeftData {
                username: user.username.clone(),
                room: user.room.clone(),
                user_count,
            };
            socket.to(&user.room).emit("user_left", &user_left_data).ok();
            
            // Create and broadcast system message
            let system_message = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                username: "System".to_string(),
                message: format!("{} left the room", user.username),
                room: user.room.clone(),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            };
            
            app_state.add_message(system_message.clone()).await;
            socket.to(&user.room).emit("new_message", &system_message).ok();
        }
        
        // Update rooms list for all clients
        let rooms_info = app_state.get_rooms_info().await;
        socket.broadcast().emit("rooms_list", &rooms_info).ok();
    }
}

// Handle getting room list
pub async fn get_rooms_list(app_state: State<AppState>) -> Result<axum::Json<HashMap<String, usize>>, axum::http::StatusCode> {
    let rooms = app_state.get_rooms_info().await;
    Ok(axum::Json(rooms))
}'''

# Create lib.rs - Library module declarations
lib_rs = '''pub mod models;
pub mod state;
pub mod handlers;

pub use models::*;
pub use state::*;
pub use handlers::*;'''

# Save to CSV files
df_handlers = pd.DataFrame([["handlers.rs", handlers_rs]])
df_handlers.columns = ["File", "Content"]
df_handlers.to_csv("handlers_rs.csv", index=False)

df_lib = pd.DataFrame([["lib.rs", lib_rs]])
df_lib.columns = ["File", "Content"]
df_lib.to_csv("lib_rs.csv", index=False)

print("✅ Created handlers.rs and lib.rs")
print("📁 Handlers file contains all socket event handlers")
print("📁 Lib file contains module declarations")






# Create main.rs - Main application entry point
main_rs = '''use axum::{
    routing::{get, post},
    Router,
    response::{Html, Response},
    http::StatusCode,
};
use socketioxide::{extract::SocketRef, SocketIo};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, Level};
use tracing_subscriber;

mod models;
mod state;
mod handlers;

use crate::state::AppState;
use crate::handlers::{on_connect, get_rooms_list};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting Rust Socket.IO Chat Server");

    // Create app state
    let app_state = Arc::new(AppState::new());

    // Create SocketIo layer
    let (layer, io) = SocketIo::new_layer();

    // Register the main namespace handler
    io.ns("/", {
        let app_state = app_state.clone();
        move |socket: SocketRef| {
            let app_state = app_state.clone();
            async move {
                on_connect(socket, socketioxide::extract::State(app_state)).await;
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
        .route("/api/rooms", get({
            let app_state = app_state.clone();
            move || get_rooms_list(socketioxide::extract::State(app_state.clone()))
        }))
        .layer(cors)
        .layer(layer)
        .with_state(app_state);

    info!("🌐 Server starting on http://localhost:3000");
    info!("📱 Chat interface available at http://localhost:3000");

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
}'''

# Save to CSV file
df_main = pd.DataFrame([["main.rs", main_rs]])
df_main.columns = ["File", "Content"]
df_main.to_csv("main_rs.csv", index=False)

print("✅ Created main.rs")
print("📁 Main file contains the server setup with Axum and Socketioxide")
print("🚀 Server will run on http://localhost:3000")







# Create index.html - Frontend chat interface
index_html = '''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Socket.IO Chat</title>
    <script src="https://cdn.socket.io/4.7.2/socket.io.min.js"></script>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
        }

        .chat-container {
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.2);
            width: 90%;
            max-width: 1200px;
            height: 80vh;
            display: flex;
            overflow: hidden;
        }

        .sidebar {
            width: 300px;
            background: #2c3e50;
            color: white;
            padding: 20px;
            display: flex;
            flex-direction: column;
        }

        .main-chat {
            flex: 1;
            display: flex;
            flex-direction: column;
        }

        .chat-header {
            background: #34495e;
            color: white;
            padding: 20px;
            text-align: center;
        }

        .login-form {
            text-align: center;
            padding: 40px;
        }

        .login-form input, .login-form select, .login-form button {
            width: 100%;
            max-width: 300px;
            padding: 12px;
            margin: 10px 0;
            border: 2px solid #ddd;
            border-radius: 8px;
            font-size: 16px;
        }

        .login-form button {
            background: #3498db;
            color: white;
            border: none;
            cursor: pointer;
            font-weight: bold;
        }

        .login-form button:hover {
            background: #2980b9;
        }

        .messages {
            flex: 1;
            padding: 20px;
            overflow-y: auto;
            background: #f8f9fa;
        }

        .message {
            margin: 10px 0;
            padding: 12px;
            border-radius: 12px;
            max-width: 70%;
            word-wrap: break-word;
        }

        .message.user {
            background: #3498db;
            color: white;
            margin-left: auto;
            text-align: right;
        }

        .message.other {
            background: white;
            border: 1px solid #ddd;
        }

        .message.system {
            background: #f39c12;
            color: white;
            text-align: center;
            margin: 10px auto;
            max-width: 100%;
            font-style: italic;
        }

        .message-author {
            font-weight: bold;
            font-size: 0.9em;
            margin-bottom: 4px;
        }

        .message-time {
            font-size: 0.8em;
            opacity: 0.7;
            margin-top: 4px;
        }

        .input-area {
            padding: 20px;
            background: white;
            border-top: 1px solid #ddd;
            display: flex;
            gap: 10px;
        }

        .input-area input {
            flex: 1;
            padding: 12px;
            border: 2px solid #ddd;
            border-radius: 8px;
            font-size: 16px;
        }

        .input-area button {
            padding: 12px 20px;
            background: #27ae60;
            color: white;
            border: none;
            border-radius: 8px;
            cursor: pointer;
            font-weight: bold;
        }

        .input-area button:hover {
            background: #219a52;
        }

        .users-list {
            margin-top: 20px;
        }

        .users-list h3 {
            margin-bottom: 10px;
            padding-bottom: 10px;
            border-bottom: 1px solid #34495e;
        }

        .user-item {
            padding: 8px;
            margin: 4px 0;
            background: rgba(255,255,255,0.1);
            border-radius: 4px;
        }

        .rooms-list {
            margin-bottom: 20px;
        }

        .room-item {
            padding: 10px;
            margin: 5px 0;
            background: rgba(255,255,255,0.1);
            border-radius: 4px;
            cursor: pointer;
            transition: background 0.2s;
        }

        .room-item:hover {
            background: rgba(255,255,255,0.2);
        }

        .room-item.active {
            background: #3498db;
        }

        .typing-indicator {
            padding: 10px 20px;
            font-style: italic;
            color: #666;
            font-size: 0.9em;
        }

        .hidden {
            display: none;
        }

        .status {
            padding: 10px;
            text-align: center;
            font-weight: bold;
        }

        .status.connected {
            background: #27ae60;
            color: white;
        }

        .status.disconnected {
            background: #e74c3c;
            color: white;
        }

        @media (max-width: 768px) {
            .chat-container {
                width: 100%;
                height: 100vh;
                border-radius: 0;
                flex-direction: column;
            }
            
            .sidebar {
                width: 100%;
                height: 200px;
            }
        }
    </style>
</head>
<body>
    <div class="chat-container">
        <!-- Sidebar -->
        <div class="sidebar">
            <h2>🦀 Rust Chat</h2>
            <div class="status disconnected" id="status">Disconnected</div>
            
            <div class="rooms-list">
                <h3>Rooms</h3>
                <div id="rooms"></div>
            </div>
            
            <div class="users-list">
                <h3>Users in Room</h3>
                <div id="users"></div>
            </div>
        </div>

        <!-- Main Chat Area -->
        <div class="main-chat">
            <!-- Login Form -->
            <div id="login-form" class="login-form">
                <h2>Join Chat</h2>
                <input type="text" id="username" placeholder="Enter your username" maxlength="20" required>
                <select id="room-select">
                    <option value="general">General</option>
                    <option value="random">Random</option>
                    <option value="tech">Tech Talk</option>
                    <option value="rust">Rust Discussion</option>
                </select>
                <input type="text" id="custom-room" placeholder="Or enter custom room name" maxlength="30">
                <button onclick="joinChat()">Join Chat</button>
            </div>

            <!-- Chat Interface -->
            <div id="chat-interface" class="hidden">
                <div class="chat-header">
                    <h3 id="current-room">Room: General</h3>
                </div>
                
                <div class="messages" id="messages"></div>
                
                <div class="typing-indicator" id="typing-indicator"></div>
                
                <div class="input-area">
                    <input type="text" id="message-input" placeholder="Type a message..." maxlength="500">
                    <button onclick="sendMessage()">Send</button>
                </div>
            </div>
        </div>
    </div>

    <script>
        let socket;
        let currentUser = null;
        let currentRoom = null;
        let typingTimer;

        // Initialize socket connection
        function initSocket() {
            socket = io();
            
            socket.on('connect', () => {
                document.getElementById('status').textContent = 'Connected';
                document.getElementById('status').className = 'status connected';
            });
            
            socket.on('disconnect', () => {
                document.getElementById('status').textContent = 'Disconnected';
                document.getElementById('status').className = 'status disconnected';
            });
            
            // Room events
            socket.on('rooms_list', (rooms) => {
                updateRoomsList(rooms);
            });
            
            socket.on('joined_room', (data) => {
                currentRoom = data.room;
                document.getElementById('current-room').textContent = `Room: ${data.room}`;
                document.getElementById('login-form').classList.add('hidden');
                document.getElementById('chat-interface').classList.remove('hidden');
                document.getElementById('message-input').focus();
            });
            
            socket.on('room_messages', (messages) => {
                const messagesDiv = document.getElementById('messages');
                messagesDiv.innerHTML = '';
                messages.forEach(msg => addMessage(msg));
            });
            
            socket.on('new_message', (message) => {
                addMessage(message);
            });
            
            socket.on('user_joined', (data) => {
                showNotification(`${data.username} joined the room`);
            });
            
            socket.on('user_left', (data) => {
                showNotification(`${data.username} left the room`);
            });
            
            socket.on('room_users_updated', (data) => {
                updateUsersList(data.users);
            });
            
            socket.on('user_typing', (data) => {
                if (data.is_typing) {
                    showTypingIndicator(data.username);
                } else {
                    hideTypingIndicator();
                }
            });
        }

        function joinChat() {
            const username = document.getElementById('username').value.trim();
            const selectedRoom = document.getElementById('room-select').value;
            const customRoom = document.getElementById('custom-room').value.trim();
            
            if (!username) {
                alert('Please enter a username');
                return;
            }
            
            const room = customRoom || selectedRoom;
            currentUser = username;
            
            socket.emit('join_room', {
                username: username,
                room: room
            });
        }

        function sendMessage() {
            const input = document.getElementById('message-input');
            const message = input.value.trim();
            
            if (message && currentRoom) {
                socket.emit('send_message', {
                    message: message,
                    room: currentRoom
                });
                input.value = '';
                hideTypingIndicator();
            }
        }

        function addMessage(message) {
            const messagesDiv = document.getElementById('messages');
            const messageEl = document.createElement('div');
            
            let messageClass = 'message ';
            if (message.username === 'System') {
                messageClass += 'system';
            } else if (message.username === currentUser) {
                messageClass += 'user';
            } else {
                messageClass += 'other';
            }
            
            messageEl.className = messageClass;
            
            const time = new Date(message.timestamp * 1000).toLocaleTimeString();
            
            if (message.username !== 'System') {
                messageEl.innerHTML = `
                    <div class="message-author">${message.username}</div>
                    <div>${message.message}</div>
                    <div class="message-time">${time}</div>
                `;
            } else {
                messageEl.innerHTML = `<div>${message.message}</div>`;
            }
            
            messagesDiv.appendChild(messageEl);
            messagesDiv.scrollTop = messagesDiv.scrollHeight;
        }

        function updateRoomsList(rooms) {
            const roomsDiv = document.getElementById('rooms');
            roomsDiv.innerHTML = '';
            
            Object.entries(rooms).forEach(([room, count]) => {
                const roomEl = document.createElement('div');
                roomEl.className = 'room-item';
                roomEl.textContent = `${room} (${count})`;
                roomEl.onclick = () => switchRoom(room);
                roomsDiv.appendChild(roomEl);
            });
        }

        function updateUsersList(users) {
            const usersDiv = document.getElementById('users');
            usersDiv.innerHTML = '';
            
            users.forEach(user => {
                const userEl = document.createElement('div');
                userEl.className = 'user-item';
                userEl.textContent = user;
                if (user === currentUser) {
                    userEl.style.fontWeight = 'bold';
                }
                usersDiv.appendChild(userEl);
            });
        }

        function switchRoom(room) {
            if (currentUser && room !== currentRoom) {
                socket.emit('join_room', {
                    username: currentUser,
                    room: room
                });
            }
        }

        function showNotification(message) {
            // This could be enhanced with better notifications
            console.log('Notification:', message);
        }

        function showTypingIndicator(username) {
            const indicator = document.getElementById('typing-indicator');
            indicator.textContent = `${username} is typing...`;
        }

        function hideTypingIndicator() {
            document.getElementById('typing-indicator').textContent = '';
        }

        // Typing indicator logic
        document.getElementById('message-input')?.addEventListener('input', () => {
            if (currentRoom) {
                socket.emit('typing');
                clearTimeout(typingTimer);
                typingTimer = setTimeout(() => {
                    socket.emit('stop_typing');
                }, 1000);
            }
        });

        // Send message on Enter key
        document.getElementById('message-input')?.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                sendMessage();
            }
        });

        // Send message on Enter key in username field
        document.getElementById('username')?.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                joinChat();
            }
        });

        // Initialize socket when page loads
        document.addEventListener('DOMContentLoaded', () => {
            initSocket();
        });
    </script>
</body>
</html>'''

# Save to CSV file
df_html = pd.DataFrame([["index.html", index_html]])
df_html.columns = ["File", "Content"]
df_html.to_csv("index_html.csv", index=False)

print("✅ Created index.html")
print("📁 HTML file contains a complete chat interface with:")
print("  • User login and room selection")
print("  • Real-time messaging")
print("  • Room management")
print("  • User list")
print("  • Typing indicators")
print("  • Responsive design")







//



# Create README.md with comprehensive setup and usage instructions
readme_md = '''# 🦀 Rust Socket.IO Chat Application

A real-time chat application built with Rust, Axum, and Socket.IO that supports multiple rooms, user management, and live messaging.

## ✨ Features

- **Real-time messaging** with Socket.IO
- **Multiple chat rooms** with dynamic room creation
- **User management** with join/leave notifications
- **Typing indicators** to show when users are typing
- **Message history** that persists while server is running
- **Responsive web interface** that works on desktop and mobile
- **Room-based broadcasting** - messages only go to users in the same room
- **System messages** for user join/leave events
- **User lists** showing who's in each room

## 🛠️ Prerequisites

- Rust 1.70+ 
- Cargo

## 📦 Dependencies

The project uses these key dependencies:

- `axum` - Web framework
- `socketioxide` - Socket.IO server implementation for Rust
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `uuid` - Generate unique IDs
- `tracing` - Logging
- `tower-http` - HTTP middleware (CORS)

## 🚀 Quick Start

1. **Clone/Create the project:**
   ```bash
   mkdir rust-socket-chat
   cd rust-socket-chat
   ```

2. **Create the project structure:**
   ```
   rust-socket-chat/
   ├── src/
   │   ├── main.rs
   │   ├── lib.rs
   │   ├── models.rs
   │   ├── handlers.rs
   │   └── state.rs
   ├── templates/
   │   └── index.html
   ├── Cargo.toml
   └── README.md
   ```

3. **Copy all the provided code files into their respective locations**

4. **Build and run:**
   ```bash
   cargo run
   ```

5. **Open your browser:**
   ```
   http://localhost:3000
   ```

## 🎯 Usage

### Starting the Server

```bash
# Development mode
cargo run

# Release mode (production)
cargo build --release
./target/release/rust-socket-chat
```

The server will start on `http://localhost:3000`

### Using the Chat

1. **Open the web interface** at `http://localhost:3000`
2. **Enter a username** (required)
3. **Select a room** from the dropdown or enter a custom room name
4. **Click "Join Chat"** to enter the chat room
5. **Start messaging!** Type messages and press Enter or click Send

### Features in Action

- **Multiple Users**: Open multiple browser tabs/windows to simulate different users
- **Different Rooms**: Users can switch between rooms to have separate conversations
- **Real-time Updates**: See messages, user joins/leaves, and typing indicators in real-time
- **Mobile Friendly**: The interface adapts to different screen sizes

## 🏗️ Architecture

### Backend (Rust)

- **main.rs**: Server setup and routing
- **models.rs**: Data structures (User, ChatMessage, etc.)
- **state.rs**: Application state management with async-safe data structures
- **handlers.rs**: Socket.IO event handlers
- **lib.rs**: Module declarations

### Frontend (HTML/JavaScript)

- **index.html**: Complete web interface with embedded CSS and JavaScript
- Uses Socket.IO client library for real-time communication
- Responsive design that works on desktop and mobile

### Data Flow

1. **Client connects** → Server creates socket connection
2. **User joins room** → Server adds user to room state and Socket.IO room
3. **Message sent** → Server broadcasts to all users in the same room
4. **User leaves** → Server removes user and notifies others

## 📡 Socket.IO Events

### Client → Server

| Event | Data | Description |
|-------|------|-------------|
| `join_room` | `{username, room}` | Join a chat room |
| `send_message` | `{message, room}` | Send a message |
| `typing` | - | Indicate user is typing |
| `stop_typing` | - | Stop typing indicator |

### Server → Client

| Event | Data | Description |
|-------|------|-------------|
| `joined_room` | `{username, room}` | Confirm room join |
| `room_messages` | `[messages]` | Send message history |
| `new_message` | `{message}` | New message received |
| `user_joined` | `{username, room, user_count}` | User joined room |
| `user_left` | `{username, room, user_count}` | User left room |
| `room_users_updated` | `{users, count}` | Updated user list |
| `user_typing` | `{username, room, is_typing}` | Typing indicator |
| `rooms_list` | `{room: count}` | Available rooms |

## 🔧 Configuration

### Server Configuration

You can modify these settings in `main.rs`:

```rust
// Change server port
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

// Configure CORS (in main.rs)
let cors = CorsLayer::new()
    .allow_origin(Any)  // Configure allowed origins
    .allow_methods(Any)
    .allow_headers(Any);
```

### Client Configuration

The client automatically connects to the same host. For production, you might want to configure the Socket.IO connection URL.

## 🐛 Troubleshooting

### Common Issues

1. **Port already in use**:
   ```
   Error: Address already in use
   ```
   - Change the port in `main.rs` or kill the process using port 3000

2. **Connection failed**:
   - Ensure the server is running
   - Check firewall settings
   - Verify the correct URL (http://localhost:3000)

3. **Messages not appearing**:
   - Check browser console for errors
   - Ensure JavaScript is enabled
   - Try refreshing the page

### Debug Mode

The application uses `tracing` for logging. Set log level for more details:

```bash
RUST_LOG=debug cargo run
```

## 🔐 Security Considerations

This is a demo application. For production use, consider:

- **Authentication**: Add user authentication
- **Rate limiting**: Prevent spam/abuse
- **Input validation**: Sanitize user inputs
- **HTTPS**: Use secure connections
- **Room permissions**: Control who can join rooms
- **Message filtering**: Filter inappropriate content

## 🚀 Performance

### Scalability

- **Memory**: Messages are stored in memory - consider Redis for persistence
- **Connections**: Tokio handles thousands of concurrent connections efficiently
- **Horizontal scaling**: Use Redis adapter for multiple server instances

### Optimizations

- Messages are stored in memory and lost on restart
- For production, integrate with a database (PostgreSQL, MongoDB, etc.)
- Implement message persistence and user sessions

## 🔄 Future Enhancements

- [ ] Database integration (PostgreSQL/MongoDB)
- [ ] User authentication and profiles
- [ ] Private messaging
- [ ] File/image sharing
- [ ] Message reactions/emojis
- [ ] Message search and history
- [ ] Admin panel for room management
- [ ] Push notifications
- [ ] Voice/video calling integration

## 📝 License

This project is open source and available under the MIT License.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📞 Support

For questions or issues:
- Check the troubleshooting section
- Review the Socket.IO documentation
- Check Rust/Axum documentation
- Open an issue on GitHub

---

Built with ❤️ using Rust, Axum, and Socket.IO'''

# Create a comprehensive project summary
project_summary = {
    "File": [
        "Cargo.toml", "src/main.rs", "src/lib.rs", "src/models.rs", 
        "src/state.rs", "src/handlers.rs", "templates/index.html", "README.md"
    ],
    "Purpose": [
        "Project dependencies and metadata",
        "Server entry point with Axum and Socketioxide setup",
        "Module declarations",
        "Data structures (User, ChatMessage, room data, etc.)",
        "Application state management with async-safe structures",
        "Socket.IO event handlers for chat functionality",
        "Complete web interface with real-time chat UI",
        "Documentation and setup instructions"
    ],
    "Key Features": [
        "All required dependencies including socketioxide 0.15.0",
        "HTTP server, CORS, health check, Socket.IO integration",
        "Clean module organization",
        "Type-safe message and user structures with UUID generation",
        "Thread-safe room and user management with Tokio async locks",
        "Join/leave rooms, send messages, typing indicators, broadcast system",
        "Responsive design, multiple rooms, user lists, real-time updates",
        "Complete setup guide and API documentation"
    ]
}

df_summary = pd.DataFrame(project_summary)
df_summary.to_csv("project_summary.csv", index=False)

# Save README
df_readme = pd.DataFrame([["README.md", readme_md]])
df_readme.columns = ["File", "Content"]
df_readme.to_csv("readme_md.csv", index=False)

print("✅ Created README.md and project summary")
print("📋 Project Summary:")
for i, row in df_summary.iterrows():
    print(f"  • {row['File']}: {row['Key Features']}")
    
print("\n🎯 The complete chat application includes:")
print("  • Real-time messaging with Socket.IO")
print("  • Multiple chat rooms")
print("  • User management and presence")
print("  • Typing indicators") 
print("  • Message broadcasting")
print("  • Responsive web interface")
print("  • Complete documentation")