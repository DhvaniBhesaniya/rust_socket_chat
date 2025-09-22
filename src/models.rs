use serde::{Deserialize, Serialize};
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
}
