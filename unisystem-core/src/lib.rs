use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{path::PathBuf, sync::Arc};
use std::io::Error;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub trait AudioSample: Copy+Send+Sync+'static {
    fn as_bytes(samples: &[Self]) -> &[u8];
}

impl AudioSample for f32 {
    fn as_bytes(samples: &[Self]) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                samples.as_ptr() as *const u8,
                std::mem::size_of_val(samples),
            )
        }
    }
}

impl AudioSample for i16 {
    fn as_bytes(samples: &[Self]) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                samples.as_ptr() as *const u8,
                std::mem::size_of_val(samples),
            )
        }
    }
}

pub struct AppState<T> where T:AudioSample {
    pub audio_tx: broadcast::Sender<Vec<T>>,
}

pub async fn start_server<T>(audio_tx: broadcast::Sender<Vec<T>>) -> Result<(), Error>
where
    T: AudioSample,
{
    let state = Arc::new(AppState { audio_tx });

    let web_client_path = web_client_path();

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

pub fn web_client_path() -> PathBuf {
    [
        "web-client",
        "../web-client",
        "../../web-client",
        "../../../web-client",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|path| path.join("index.html").exists())
    // .expect("path not found")
    .unwrap_or_else(|| PathBuf::from("web-client"))
}

async fn ws_handler<T>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState<T>>>,
) -> impl IntoResponse
where
    T: AudioSample,
{
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket<T>(mut socket: WebSocket, state: Arc<AppState<T>>)
where
    T:AudioSample,
{
    let mut rx = state.audio_tx.subscribe();
    println!("Client connected to WebSocket");

    loop {
        match rx.recv().await {
            Ok(samples) => {
                let bytes = T::as_bytes(&samples);
                if socket.send(Message::Binary(bytes.to_vec().into())).await.is_err() {
                    println!("client got discommeted!..");
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                eprintln!("socket cleint lagged..skipping audio packs: {}", skipped);
                continue;
            }
            Err(broadcast::error::RecvError::Closed) => {
                println!("udio broadcast closed..");
                break;
            }
        }
    }
}
