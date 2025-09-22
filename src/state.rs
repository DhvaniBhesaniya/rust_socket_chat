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
}
