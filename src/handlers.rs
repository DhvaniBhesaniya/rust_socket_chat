use crate::models::*;
use crate::state::AppState;
use socketioxide::extract::{Data, SocketRef, State};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{error, info, warn};

// Handle client connection
pub async fn on_connect(socket: SocketRef, app_state: State<AppState>) {
    info!("Client connectd: {}", socket.id);

    // send available rooms to the connected client
    let rooms_info = app_state.get_rooms_info().await;
    socket.emit("rooms_list", &rooms_info).ok();

    // Handle join room event
    socket.on("join_room", {
        let app_state = app_state.clone();
        move |socket: SocketRef, Data(data): Data<JoinRoomData>| {
            let app_state = app_state.clone();
            async move {
                handle_join_room(socket, data, socketioxide::extract::State(app_state)).await;
            }
        }
    });

    // Handle send message event
    socket.on("send_message", {
        let app_state = app_state.clone();
        move |socket: SocketRef, Data(data): Data<SendMessageData>| {
            let app_state = app_state.clone();
            async move {
                handle_send_message(socket, data, socketioxide::extract::State(app_state)).await;
            }
        }
    });

    // Handle typing events
    socket.on("typing", {
        let app_state = app_state.clone();
        move |socket: SocketRef| {
            let app_state = app_state.clone();
            async move {
                handle_typing(socket, socketioxide::extract::State(app_state), true).await;
            }
        }
    });

    socket.on("stop_typing", {
        let app_state = app_state.clone();
        move |socket: SocketRef| {
            let app_state = app_state.clone();
            async move {
                handle_typing(socket, socketioxide::extract::State(app_state), false).await;
            }
        }
    });

    // In on_connect, add this after other socket.on handlers:
    socket.on("leave_room", {
        let app_state = app_state.clone();
        move |socket: SocketRef, Data(data): Data<JoinRoomData>| {
            let app_state = app_state.clone();
            async move {
                handle_leave_room(socket, data, socketioxide::extract::State(app_state)).await;
            }
        }
    });

    // Handle disconnect
    socket.on_disconnect({
        let app_state = app_state.clone();
        move |socket: SocketRef| {
            let app_state = app_state.clone();
            async move {
                handle_disconnect(socket, socketioxide::extract::State(app_state)).await;
            }
        }
    });
}

// Handle user joining a room
async fn handle_join_room(socket: SocketRef, data: JoinRoomData, app_state: State<AppState>) {
    let socket_id = socket.id.to_string();
    info!("User {} joining room: {}", socket_id, data.room);

    // create a new user
    let user = User::new(data.username.clone(), data.room.clone(), socket_id.clone());

    //  add user to  room
    app_state.add_user_to_room(user.clone()).await;

    // join the socket.io room
    socket.join(data.room.clone()).ok();

    // send room history to the user
    let messages = app_state.get_room_messages(&data.room).await;
    socket.emit("room_messages", &messages).ok();

    // get updated room users
    let room_users = app_state.get_room_users(&data.room).await;
    let user_count = room_users.len();
    let usernames: Vec<String> = room_users.iter().map(|u| u.username.clone()).collect();

    // notify user they joined successfully
    socket
        .emit(
            "joined_room",
            &JoinRoomData {
                room: data.room.clone(),
                username: data.username.clone(),
            },
        )
        .ok();

    // send the updated uer list to all users in th room
    let room_users_data = RoomUsersData {
        users: usernames,
        count: user_count,
    };

    socket
        .to(data.room.clone())
        .emit("room_users_updated", &room_users_data)
        .ok();
    socket.emit("room_users_updated", &room_users_data).ok();

    // notify all users in the room that a new user has joined

    let user_joined_data = UserJoinedData {
        username: data.username.clone(),
        room: data.room.clone(),
        user_count,
    };

    socket
        .to(data.room.clone())
        .emit("user_joined", &user_joined_data)
        .ok();

    // create and broadcast system message
    let system_message = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        username: "System".to_string(),
        message: format!("{} has joined the room.", data.username),
        room: data.room.clone(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    app_state.add_message(system_message.clone()).await;
    socket
        .to(data.room.clone())
        .emit("new_message", &system_message)
        .ok();

    // update the room list for all the client.
    let rooms_info = app_state.get_rooms_info().await;
    socket.broadcast().emit("rooms_list", &rooms_info).ok();
}

// handle sending a message
pub async fn handle_send_message(
    socket: SocketRef,
    data: SendMessageData,
    app_state: State<AppState>,
) {
    let socket_id = socket.id.to_string();

    // get user info
    if let Some(user) = app_state.get_user_by_socket_id(&socket_id).await {
        info!(
            "User {} sending message to room {}: {}",
            user.username, user.room, data.message
        );

        // Validate that user is in the room they're trying to send to
        if user.room != data.room {
            warn!(
                "User {} tried to send message to room {} but is in room {}",
                user.username, data.room, user.room
            );
            return;
        }

        // create message
        let message = ChatMessage::new(user.username, data.message, data.room.clone());
        // store message
        app_state.add_message(message.clone()).await;

        // broadcast message to all users in the room (including sender)
        socket.within(data.room).emit("new_message", &message).ok();
    } else {
        error!("received message from unknown user : {}", socket_id);
    }
}

// handle  typing indicator

async fn handle_typing(socket: SocketRef, app_state: State<AppState>, is_typing: bool) {
    let socket_id = socket.id.to_string();

    if let Some(user) = app_state.get_user_by_socket_id(&socket_id).await {
        let typing_data = TypingData {
            username: user.username.clone(),
            room: user.room.clone(),
            is_typing,
        };

        // broadcast typing status to others in the room  (excluding sender)
        socket.to(user.room).emit("user_typing", &typing_data).ok();
    }
}

// handle user leaving the room
async fn handle_leave_room(socket: SocketRef, data: JoinRoomData, app_state: State<AppState>) {
    let socket_id = socket.id.to_string();
    info!("User {} requested to leave room: {}", data.username, data.room);

    // Remove user and get their info
    if let Some(user) = app_state.remove_user(&socket_id).await {
        info!("User {} left room: {}", user.username, user.room);

        // get updated room users
        let room_users = app_state.get_room_users(&user.room).await;
        let user_count = room_users.len();
        let usernames: Vec<String> = room_users.iter().map(|u| u.username.clone()).collect();

        // send updated user list to remaining users in the room
        if !room_users.is_empty() {
            let room_users_data = RoomUsersData {
                users: usernames,
                count: user_count,
            };

            socket
                .to(user.room.to_owned())
                .emit("room_users_updated", &room_users_data)
                .ok();

            // notify other users that user left
            let user_left_data = UserLeftData {
                username: user.username.clone(),
                room: user.room.clone(),
                user_count,
            };

            socket
                .to(user.room.clone())
                .emit("user_left", &user_left_data)
                .ok();

            // create and broadcast system message
            let system_message = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                username: "System".to_string(),
                message: format!("{} has left the room.", user.username),
                room: user.room.clone(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            app_state.add_message(system_message.clone()).await;
            socket
                .to(user.room.to_owned())
                .emit("new_message", &system_message)
                .ok();
        }

        // Update rooms list for all clients
        let rooms_info = app_state.get_rooms_info().await;
        socket.broadcast().emit("rooms_list", &rooms_info).ok();
    }
}

// Handle user disconnect

async fn handle_disconnect(socket: SocketRef, app_state: State<AppState>) {
    let socket_id = socket.id.to_string();
    info!("Client disconnected: {}", socket_id);

    // Remove user and get their info
    if let Some(user) = app_state.remove_user(&socket_id).await {
        info!("User {} left room: {}", user.username, user.room);

        // get updated room users
        let room_users = app_state.get_room_users(&user.room).await;
        let user_count = room_users.len();
        let usernames: Vec<String> = room_users.iter().map(|u| u.username.clone()).collect();

        // send updated  user list to  remaining users in the room
        if !room_users.is_empty() {
            let room_users_data = RoomUsersData {
                users: usernames,
                count: user_count,
            };

            socket
                .to(user.room.to_owned())
                .emit("room_users_updated", &room_users_data)
                .ok();

            // notify other user that user left
            let user_left_data = UserLeftData {
                username: user.username.clone(),
                room: user.room.clone(),
                user_count,
            };

            socket
                .to(user.room.clone())
                .emit("user_left", &user_left_data)
                .ok();

            //create and broadcast system message
            let system_message = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                username: "System".to_string(),
                message: format!("{} has left the room.", user.username),
                room: user.room.clone(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            app_state.add_message(system_message.clone()).await;
            socket
                .to(user.room.to_owned())
                .emit("new_message", &system_message)
                .ok();
        }

        // Update rooms list for all clients
        let rooms_info = app_state.get_rooms_info().await;
        socket.broadcast().emit("rooms_list", &rooms_info).ok();
    }
}

// Handle getting room list
pub async fn get_rooms_list(
    app_state: State<AppState>,
) -> Result<axum::Json<HashMap<String, usize>>, axum::http::StatusCode> {
    let rooms = app_state.get_rooms_info().await;
    Ok(axum::Json(rooms))
}
