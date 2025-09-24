# 🦀 Rust Socket.IO Chat Application

A real-time chat application built with Rust, Axum, and Socket.IO, supporting multiple rooms, user management, and live messaging with a modern web interface.

---

## 📚 Table of Contents

- [🦀 Rust Socket.IO Chat Application](#-rust-socketio-chat-application)
  - [📚 Table of Contents](#-table-of-contents)
  - [📝 Overview](#-overview)
  - [✨ Features](#-features)
  - [📁 Project Structure](#-project-structure)
  - [🚀 Getting Started](#-getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation \& Running](#installation--running)
  - [💡 Usage](#-usage)
  - [🏗️ Architecture](#️-architecture)
    - [Backend (Rust)](#backend-rust)
    - [Frontend (HTML/JavaScript)](#frontend-htmljavascript)
  - [📡 Socket.IO Events](#-socketio-events)
    - [Client → Server](#client--server)
    - [Server → Client](#server--client)
  - [📜 License](#-license)

---

## 📝 Overview

This project is a full-stack real-time chat application. The backend is written in Rust using [Axum](https://github.com/tokio-rs/axum) for HTTP and [Socketioxide](https://github.com/1c3t3a/socketioxide) for Socket.IO support. The frontend is a responsive HTML/JavaScript interface served from the `templates/` directory.

Users can join chat rooms, send messages, see who is online, and view typing indicators—all in real time.

---

## ✨ Features

- **Real-time messaging** with Socket.IO
- **Multiple chat rooms** (dynamic creation and joining)
- **User management** (join/leave notifications, user lists)
- **Typing indicators** to show when users are typing
- **Message history** (persists while server is running)
- **Responsive web interface** (desktop & mobile)
- **Room-based broadcasting** (messages only go to users in the same room)
- **System messages** for user join/leave events

---

## 📁 Project Structure

```
rust-socket-chat/
├── src/
│   ├── main.rs          # Server setup and routing
│   ├── lib.rs           # Module declarations
│   ├── models.rs        # Data structures (User, ChatMessage, etc.)
│   ├── handlers.rs      # Socket.IO event handlers
│   └── state.rs         # Application state management
├── templates/
│   └── index.html       # Frontend chat interface
├── static/
│   ├── script.js        # (Optional) Separate JS if needed
│   └── style.css        # (Optional) Separate CSS if needed
├── Cargo.toml           # Rust dependencies and metadata
└── Readme.md            # Project documentation
```

> **Note:** The `src/bin/` directory contains experimental or alternative binaries and is not part of the main application.

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- Cargo (comes with Rust)

### Installation & Running

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd rust_socket_chat
   ```

2. **Build and run the server:**
   ```sh
   cargo run --bin rust-socket-chat
   ```

3. **Open your browser:**
   ```
   http://localhost:3000
   ```

You should see the chat interface and be able to join rooms, send messages, and interact in real time.

---

## 💡 Usage

- **Open the web interface** at [http://localhost:3000](http://localhost:3000)
- **Enter a username** (required)
- **Select a room** from the dropdown or enter a custom room name
- **Click "Join Chat"** to enter the chat room
- **Start messaging!** Type messages and press Enter or click Send

You can open multiple browser tabs/windows to simulate different users and rooms.

---

## 🏗️ Architecture

### Backend (Rust)

- `src/main.rs`: Server setup, routing, and Socket.IO integration
- `src/models.rs`: Data structures for users, messages, and events
- `src/state.rs`: Application state management (rooms, users, messages)
- `src/handlers.rs`: All Socket.IO event handlers (join, leave, message, typing, etc.)
- `src/lib.rs`: Module declarations

### Frontend (HTML/JavaScript)

- `templates/index.html`: Complete chat interface with embedded CSS and JavaScript
- Uses Socket.IO client library for real-time communication
- Responsive design for desktop and mobile

---

## 📡 Socket.IO Events

### Client → Server

| Event         | Data                | Description                        |
|---------------|---------------------|------------------------------------|
| `join_room`   | `{room, username}`  | Join a chat room                   |
| `send_message`| `{room, message}`   | Send a message to the room         |
| `typing`      |                     | Notify others user is typing       |
| `stop_typing` |                     | Notify others user stopped typing  |
| `leave_room`  | `{room, username}`  | Leave the current room             |

### Server → Client

| Event           | Data                       | Description                        |
|-----------------|----------------------------|------------------------------------|
| `rooms_list`    | `{room: user_count, ...}`  | List of available rooms            |
| `room_messages` | `[ChatMessage, ...]`       | Message history for the room       |
| `joined_room`   | `{room, username, ...}`    | Confirmation of joining a room     |
| `user_joined`   | `{username, room, ...}`    | Notification when a user joins     |
| `user_left`     | `{username, room, ...}`    | Notification when a user leaves    |
| `room_users`    | `{users, count}`           | List of users in the room          |
| `new_message`   | `ChatMessage`              | New message in the room            |
| `typing`        | `{username, room, ...}`    | User is typing indicator           |
| `stop_typing`   | `{username, room, ...}`    | User stopped typing indicator      |

---

## 📜 License

This project is open source and available under the MIT License.

---

Built with ❤️ using Rust, Axum, and