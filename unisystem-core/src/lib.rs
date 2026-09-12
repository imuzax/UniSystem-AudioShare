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
use std::{io::Error, path::Path};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for Vec<f32> {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * std::mem::size_of::<f32>(),
            )
        }
    }
}

impl AsBytes for Vec<i16> {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * std::mem::size_of::<i16>(),
            )
        }
    }
}

pub struct AppState<T> {
    pub audio_rx: broadcast::Sender<T>,
}

pub async fn start_server<T>(audio_tx: broadcast::Sender<T>) -> Result<(), Error>
where
    T: Clone + Send + Sync + AsBytes + 'static,
{
    let state = Arc::new(AppState { audio_rx: audio_tx });

    let web_client_path = [
        "web-client",
        "../web-client",
        "../../web-client",
        "../../../web-client",
    ]
    .into_iter()
    .find(|path| Path::new(path).join("index.html").exists())
    .unwrap_or("web-client");

    let app = Router::new()
        .route("/ws", get(ws_handler::<T>))
        .fallback_service(ServeDir::new(web_client_path))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8080";
    println!("Web server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await
}

async fn ws_handler<T>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState<T>>>,
) -> impl IntoResponse
where
    T: Clone + Send + Sync + AsBytes + 'static,
{
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket<T>(mut socket: WebSocket, state: Arc<AppState<T>>)
where
    T: Send + Sync + Clone + AsBytes + 'static,
{
    let mut rx = state.audio_rx.subscribe();
    println!("Client connected to WebSocket");

    while let Ok(samples) = rx.recv().await {
        let byte_data = samples.as_bytes();

        if socket
            .send(Message::Binary(byte_data.to_vec().into()))
            .await
            .is_err()
        {
            println!("Client disconnected");
            break;
        }
    }
}
