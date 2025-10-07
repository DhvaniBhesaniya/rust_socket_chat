// use axum::{routing::get, Router};
// use socketioxide::{
//     extract::{Data, SocketRef},
//     SocketIo,
// };
// use std::net::SocketAddr;
// use tower_http::cors::{CorsLayer, Any};
// use hyper::{Method, header}; // ✅ Use `hyper` not `http` crate directly
// use tracing_subscriber::FmtSubscriber;

// #[tokio::main]
// async fn main() {
//     FmtSubscriber::builder().init();

//     // Create a Socket.IO layer and get the controller
//     let (layer, io) = SocketIo::new_layer();

//     // Set up the /chat namespace and handlers
//     io.ns("/chat", |socket| {
//         // Handle 'message' events
//         socket.on("message", |socket: SocketRef, Data::<String>(data)| async move {
//             println!("Received message: {:?}", data);

//             // Emit a reply
//             socket.emit("message", &format!("Echo: {}", data)).ok(); // ✅ `&String` to match expected type
//         });

//         // Handle 'join_room' events
//         socket.on("join_room", |socket: SocketRef, Data::<serde_json::Value>(data)| async move {
//             println!("Join room data: {:?}", data);
//         });
//     });

//     // CORS setup
//     let cors = CorsLayer::new()
//         .allow_origin(Any)
//         .allow_methods([Method::GET, Method::POST])
//         .allow_headers([header::CONTENT_TYPE]);

//     // Define the Axum router with layers
//     let app = Router::new()
//         .route("/", get(|| async { "Server is running." }))
//         .layer(cors)
//         .layer(layer); // Socket.io layer last

//     let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
//     println!("🚀 Server running at http://{}", addr);

//     // Use Axum's server
//     axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
//         .await
//         .unwrap();
// }




//



fn main(){
    println!("Hello, world!");
}