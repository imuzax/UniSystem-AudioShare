import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface NetworkPayload {
  name: string;
  ip: string;
}

interface VolumePayload {
  peak: f32; // wait, TypeScript uses number
}

function App() {
  const [ips, setIps] = useState<NetworkPayload[]>([]);
  const [volume, setVolume] = useState(0);

  useEffect(() => {
    // Fetch IPs
    invoke<NetworkPayload[]>("get_network_ips")
      .then((res) => setIps(res))
      .catch(console.error);

    // Listen to volume peaks
    const unlisten = listen<{ peak: number }>("volume-peak", (event) => {
      setVolume(event.payload.peak);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="container">
      <header className="header">
        <h1>UniSystem <span>AudioShare</span></h1>
        <p className="subtitle">Ultra-Low Latency Wi-Fi Streaming Engine</p>
      </header>

      <main>
        <section className="glass-card visualizer-section">
          <h2>Live Audio Broadcast</h2>
          <div className="visualizer-container">
            {Array.from({ length: 40 }).map((_, i) => {
              // Map 0.0 -> 1.0 volume to 40 bars
              const isActive = (volume * 40) > i;
              return (
                <div 
                  key={i} 
                  className={`bar ${isActive ? 'active' : ''}`}
                  style={{ height: isActive ? `${Math.max(10, (volume * 100) + (Math.random() * 20))}px` : '10px' }}
                />
              );
            })}
          </div>
          <p className="status">
            <span className="dot pulse"></span>
            Broadcasting 48kHz Float32 PCM
          </p>
        </section>

        <section className="glass-card network-section">
          <h2>Connect Your Devices</h2>
          <p className="instruction">Open this URL on your phone's browser to listen instantly:</p>
          
          <div className="ip-list">
            {ips.length > 0 ? ips.map((item, idx) => (
              <div key={idx} className="ip-card">
                <span className="net-name">{item.name}</span>
                <a 
                  href="#" 
                  onClick={(e) => {
                    e.preventDefault();
                    import('@tauri-apps/plugin-opener').then(opener => {
                      opener.open(`http://${item.ip}:8080`);
                    });
                  }} 
                  className="net-ip"
                >
                  http://{item.ip}:8080
                </a>
              </div>
            )) : (
              <div className="ip-card loading">Scanning network...</div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
