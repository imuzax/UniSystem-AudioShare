use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub audio_rx: broadcast::Sender<Vec<f32>>,
}

pub async fn start_server(audio_tx: broadcast::Sender<Vec<f32>>) -> Result<(), std::io::Error> {
    let state = Arc::new(AppState { audio_rx: audio_tx });

    let mut web_client_path = "web-client".to_string();
    if std::path::Path::new("web-client/index.html").exists() {
        web_client_path = "web-client".to_string();
    } else if std::path::Path::new("../web-client/index.html").exists() {
        web_client_path = "../web-client".to_string();
    } else if std::path::Path::new("../../web-client/index.html").exists() {
        web_client_path = "../../web-client".to_string();
    } else if std::path::Path::new("../../../web-client/index.html").exists() {
        web_client_path = "../../../web-client".to_string();
    }

    let app = Router::new()
        // Serve static web client files via fallback
        .fallback_service(ServeDir::new(web_client_path))
        // WebSocket route
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8080";
    println!("Web server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.audio_rx.subscribe();
    println!("Client connected to WebSocket");

    loop {
        match rx.recv().await {
            Ok(samples) => {
                // Convert &[f32] to &[u8]
                let byte_data: &[u8] = unsafe {
                    std::slice::from_raw_parts(
                        samples.as_ptr() as *const u8,
                        samples.len() * std::mem::size_of::<f32>(),
                    )
                };

                if socket.send(Message::Binary(byte_data.to_vec().into())).await.is_err() {
                    println!("Client disconnected");
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(missed)) => {
                println!("Client lagged behind by {} messages", missed);
            }
            Err(broadcast::error::RecvError::Closed) => {
                break;
            }
        }
    }
}
