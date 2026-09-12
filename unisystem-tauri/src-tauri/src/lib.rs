use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tauri::{AppHandle, Emitter, Manager};
use unisystem_audio::start_audio_capture;
use unisystem_core::start_server;
use local_ip_address::list_afinet_netifas;

#[derive(Clone, serde::Serialize)]
struct VolumePayload {
    peak: f32,
}

#[derive(Clone, serde::Serialize)]
struct NetworkPayload {
    name: String,
    ip: String,
}

#[tauri::command]
fn get_network_ips() -> Vec<NetworkPayload> {
    let mut ips = Vec::new();
    if let Ok(network_interfaces) = list_afinet_netifas() {
        for (name, ip) in network_interfaces.iter() {
            if ip.is_ipv4() && !ip.is_loopback() {
                ips.push(NetworkPayload {
                    name: name.clone(),
                    ip: ip.to_string(),
                });
            }
        }
    }
    ips
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_network_ips])
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Create broadcast channel for audio
            let (audio_tx, _) = broadcast::channel::<Vec<f32>>(100);
            
            // Start Web Server for streaming
            let audio_tx_clone = audio_tx.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = start_server(audio_tx_clone).await {
                    eprintln!("Web server crashed: {}", e);
                }
            });

            // Start Audio Capture
            let stop_flag = Arc::new(Mutex::new(false));
            std::thread::spawn(move || {
                start_audio_capture(move |samples: &[f32]| {
                    if samples.is_empty() { return; }

                    let mut peak = 0.0_f32;
                    for &sample in samples.iter() {
                        let abs_sample = sample.abs();
                        if abs_sample > peak {
                            peak = abs_sample;
                        }
                    }
                    
                    // Emit peak volume to frontend
                    let _ = app_handle.emit("volume-peak", VolumePayload { peak });

                    // Broadcast to web sockets
                    let _ = audio_tx.send(samples.to_vec());
                }, stop_flag).expect("Audio capture failed");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
