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

pub struct AppState<T> {
    pub audio_rx: broadcast::Sender<T>,
}

pub async fn start_server<T>(audio_tx: broadcast::Sender<T>) -> Result<(), Error>
where
    T: Clone + Send + Sync + 'static,
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
    T: Clone + Send + Sync + 'static,
{
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket<T>(mut socket: WebSocket, state: Arc<AppState<T>>)
where
    T: Send + Sync + Clone + 'static,
{
    let mut rx = state.audio_rx.subscribe();
    println!("Client connected to WebSocket");

    while let Ok(samples) = rx.recv().await {
        let byte_data = match_f32_bytes(&samples);

        if socket
            .send(Message::Binary(byte_data.into()))
            .await
            .is_err()
        {
            println!("Client disconnected");
            break;
        }
    }
}

fn match_f32_bytes<T>(_samples: &T) -> Vec<u8> {
    Vec::new()
}
