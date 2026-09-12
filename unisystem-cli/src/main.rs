use std::sync::{Arc, Mutex};
use std::time::Duration;
use unisystem_audio::start_audio_capture;

fn main() -> anyhow::Result<()> {
    println!("Starting UniSystem AudioShare CLI...");
    
    // Flag to control when to stop capturing
    let stop_flag = Arc::new(Mutex::new(false));
    let stop_flag_clone = stop_flag.clone();

    // Setup a handler to stop cleanly on Ctrl+C
    ctrlc::set_handler(move || {
        println!("\nShutting down...");
        *stop_flag_clone.lock().unwrap() = true;
    }).expect("Error setting Ctrl-C handler");

    println!("Listening to default audio capture stream. Press Ctrl+C to stop.");
    
    // We'll calculate simple peak volume per buffer and print a basic level meter
    start_audio_capture(move |samples: &[f32]| {
        if samples.is_empty() {
            return;
        }

        // Calculate max amplitude (peak)
        let mut peak = 0.0_f32;
        for &sample in samples.iter() {
            let abs_sample = sample.abs();
            if abs_sample > peak {
                peak = abs_sample;
            }
        }
        
        // Convert to a simple progress bar/meter
        let max_bars = 50;
        let num_bars = (peak * max_bars as f32).min(max_bars as f32) as usize;
        let meter = "=".repeat(num_bars);
        
        // Use carriage return to overwrite the same line
        print!("\rVolume: [{:<50}] {:.3}", meter, peak);
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }, stop_flag)?;

    Ok(())
}
