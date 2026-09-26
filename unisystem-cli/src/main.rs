use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use unisystem_audio::start_audio_capture;
use unisystem_core::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    println!("Starting UniSystem AudioShare Network Engine...");
    
    let (audio_tx, _) = broadcast::channel::<Vec<f32>>(100);
    
    let stop_flag = Arc::new(Mutex::new(false));
    let stop_flag_clone = stop_flag.clone();

    ctrlc::set_handler(move || {
        println!("\nShutting down...");
        *stop_flag_clone.lock().unwrap() = true;
        std::process::exit(0);
    })?;

    let audio_tx_clone = audio_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(audio_tx_clone).await {
            eprintln!("Web server crashed: {}", e);
        }
    });

    println!("AudioShare Server is Running!");
    println!("Open your phone browser and go to one of these links (pick your Wi-Fi IP):");
    
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
    
    std::thread::sleep(std::time::Duration::from_millis(300));
    
    std::thread::spawn(move || {
        if let Err(err) = start_audio_capture(move |samples: &[f32]| {
            if samples.is_empty() {
                return;
            }

            let mut peak = 0.0_f32;
            for &sample in samples.into_iter() {
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

            let _ = audio_tx.send(samples.to_vec());
        }, stop_flag) {
            eprintln!("Audio capture failed: {}", err);
        }
    });
    
    // Keep the main tokio thread alive
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
