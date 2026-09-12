use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use unisystem_audio::start_audio_capture;
use unisystem_core::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting UniSystem AudioShare Network Engine...");
    
    // Create a broadcast channel that can hold up to 100 recent audio chunks
    let (audio_tx, _) = broadcast::channel::<Vec<f32>>(100);
    
    // Setup shutdown flag
    let stop_flag = Arc::new(Mutex::new(false));
    let stop_flag_clone = stop_flag.clone();

    // Setup a handler to stop cleanly on Ctrl+C
    ctrlc::set_handler(move || {
        println!("\nShutting down...");
        *stop_flag_clone.lock().unwrap() = true;
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    // Start the Web Server concurrently
    let audio_tx_clone = audio_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(audio_tx_clone).await {
            eprintln!("Web server crashed: {}", e);
        }
    });

    // Get all local IPs to show the exact links to the user (bypassing docker/vpn confusion)
    println!("============================================================");
    println!("🚀 AudioShare Server is Running!");
    println!("📱 Open your phone browser and go to one of these links (pick your Wi-Fi IP):");
    
    if let Ok(network_interfaces) = local_ip_address::list_afinet_netifas() {
        for (name, ip) in network_interfaces.iter() {
            if ip.is_ipv4() && !ip.is_loopback() {
                println!("🌐 [{}] http://{}:8080", name, ip);
            }
        }
    } else {
        println!("🌐 http://0.0.0.0:8080");
    }
    println!("============================================================");
    println!("Listening to default audio capture stream. Press Ctrl+C to stop.");
    
    // Slight delay so the user can read the message before the volume meter starts spamming
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // We run the audio capture on a separate dedicated OS thread because PipeWire's MainLoop blocks
    std::thread::spawn(move || {
        start_audio_capture(move |samples: &[f32]| {
            if samples.is_empty() {
                return;
            }

            // Calculate peak for the console meter
            let mut peak = 0.0_f32;
            for &sample in samples.iter() {
                let abs_sample = sample.abs();
                if abs_sample > peak {
                    peak = abs_sample;
                }
            }
            
            let max_bars = 40;
            let num_bars = (peak * max_bars as f32).min(max_bars as f32) as usize;
            let meter = "=".repeat(num_bars);
            
            print!("\rVolume: [{:<40}] {:.3} | Broadcast Active", meter, peak);
            use std::io::Write;
            let _ = std::io::stdout().flush();

            // Send the raw PCM float block to all connected WebSockets
            // Ignore errors if no clients are connected yet
            let _ = audio_tx.send(samples.to_vec());
        }, stop_flag).expect("Audio capture failed");
    });
    
    // Keep the main tokio thread alive
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
