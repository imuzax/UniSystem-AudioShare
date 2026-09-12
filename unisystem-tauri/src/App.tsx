import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openUrl } from "@tauri-apps/plugin-opener";
import "./App.css";

interface NetworkInterface {
  name: string;
  ip: string;
}

function App() {
  const [ips, setIps] = useState<NetworkInterface[]>([]);
  // Create an array of 64 visualizer bars for a huge display
  const [peaks, setPeaks] = useState<number[]>(new Array(64).fill(0));

  useEffect(() => {
    // Get network interfaces
    invoke<NetworkInterface[]>("get_network_ips")
      .then((interfaces) => {
        setIps(interfaces);
      })
      .catch(console.error);

    // Listen for volume peaks
    const unlisten = listen<any>("volume-peak", (event) => {
      const vol = event.payload.peak || 0;
      setPeaks(prev => {
        const newPeaks = [...prev];
        // Shift left
        newPeaks.shift();
        // Add new volume with some random variation for aesthetics
        const variation = vol * (0.8 + Math.random() * 0.4);
        newPeaks.push(Math.min(1.0, variation));
        return newPeaks;
      });
    });

    return () => {
      unlisten.then(f => f());
    };
  }, []);

  return (
    <div className="dashboard-wrapper">
      {/* Dynamic Animated Background Orbs */}
      <div className="aura-orb orb-primary"></div>
      <div className="aura-orb orb-secondary"></div>

      <div className="glass-dashboard">
        <header className="dashboard-header">
          <div className="brand-title">
            <img src="/logo.jpg" alt="UniSystem Logo" className="brand-logo" />
            <div>
              <h1>UniSystem <span>AudioShare</span></h1>
              <p className="status-badge">
                <span className="dot pulse"></span>
                Engine Active & Streaming
              </p>
            </div>
          </div>
        </header>

        <main className="dashboard-content">
          {/* Hero Visualizer */}
          <section className="hero-visualizer-section">
            <div className="visualizer-display">
              {peaks.map((p, i) => (
                <div
                  key={i}
                  className={`vis-bar ${p > 0.05 ? 'active' : ''}`}
                  style={{ 
                    height: `${Math.max(4, p * 200)}px`,
                    opacity: 0.3 + p * 0.7
                  }}
                ></div>
              ))}
            </div>
            <div className="db-meter">
              <span>-60 dB</span>
              <span>-30 dB</span>
              <span>0 dB</span>
            </div>
          </section>

          {/* Network Links */}
          <section className="network-section">
            <h2 className="section-title">Available Connectors</h2>
            <p className="instruction-text">
              Open these URLs on any device connected to the same Wi-Fi.
            </p>
            
            <div className="devices-grid">
              {ips.length > 0 ? ips.map((item, idx) => (
                <div key={idx} className="device-card">
                  <div className="device-info">
                    <span className="device-icon">🌐</span>
                    <span className="device-name">{item.name}</span>
                  </div>
                  <a 
                    href="#" 
                    onClick={(e) => {
                      e.preventDefault();
                      openUrl(`http://${item.ip}:8080`);
                    }} 
                    className="device-link"
                  >
                    http://{item.ip}:8080
                  </a>
                </div>
              )) : (
                <div className="loading-state">Scanning Network...</div>
              )}
            </div>
          </section>
        </main>
      </div>
    </div>
  );
}

export default App;
