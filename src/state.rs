use crate::models::{ChatMessage, User};
use std::{collections::HashMap, sync::Arc};
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
    // add user to room
    pub async fn add_user_to_room(&self, user: User) {
        let mut rooms = self.rooms.write().await;
        let mut socket_user = self.socket_users.write().await;

        // remove user from previous room if exists
        if let Some(old_user) = socket_user.get(&user.socket_id) {
            if let Some(room_user) = rooms.get_mut(&old_user.room) {
                room_user.retain(|u| u.socket_id != user.socket_id);
                if room_user.is_empty() {
                    rooms.remove(&old_user.room);
                }
            }
        }
        // add user to the new room
        rooms
            .entry(user.room.clone())
            .or_insert_with(Vec::new)
            .push(user.clone());
        socket_user.insert(user.socket_id.clone(), user);
    }

    // remove user by socket id.
    pub async fn remove_user(&self, socket_id: &str) -> Option<User> {
        let mut rooms = self.rooms.write().await;
        let mut socket_user = self.socket_users.write().await;

        if let Some(user) = socket_user.remove(socket_id) {
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

    // get users in a room
    pub async fn get_room_users(&self, room_name: &str) -> Vec<User> {
        let rooms = self.rooms.read().await;
        rooms.get(room_name).cloned().unwrap_or_default()
    }

    // get user by socket id
    pub async fn get_user_by_socket_id(&self, socket_id: &str) -> Option<User> {
        let socket_user = self.socket_users.read().await;
        socket_user.get(socket_id).cloned()
    }

    // add message to room
    pub async fn add_message(&self, message: ChatMessage) {
        let mut messages = self.messages.write().await;
        messages
            .entry(message.room.clone())
            .or_insert_with(Vec::new)
            .push(message);
    }

    // get message from a room
    pub async fn get_room_messages(&self, room_name: &str) -> Vec<ChatMessage> {
        let messages = self.messages.read().await;
        messages.get(room_name).cloned().unwrap_or_default()
    }

    // get all rooms with user counts
    pub async fn get_rooms_info(&self) -> HashMap<String, usize> {
        let rooms = self.rooms.read().await;
        rooms
            .iter()
            .map(|(name, users)| (name.clone(), users.len()))
            .collect()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
