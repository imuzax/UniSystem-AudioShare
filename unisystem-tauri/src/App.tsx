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
  // Use a smaller number of bars for a subtle, professional visualization
  const [peaks, setPeaks] = useState<number[]>(new Array(32).fill(0));
  const [copiedIp, setCopiedIp] = useState<string | null>(null);

  useEffect(() => {
    invoke<NetworkInterface[]>("get_network_ips")
      .then((interfaces) => {
        setIps(interfaces);
      })
      .catch(console.error);

    const unlisten = listen<any>("volume-peak", (event) => {
      const vol = event.payload.peak || 0;
      setPeaks(prev => {
        const newPeaks = [...prev];
        newPeaks.shift();
        const variation = vol * (0.8 + Math.random() * 0.4);
        newPeaks.push(Math.min(1.0, variation));
        return newPeaks;
      });
    });

    return () => {
      unlisten.then(f => f());
    };
  }, []);

  const handleCopy = (ip: string) => {
    const url = `http://${ip}:8080`;
    navigator.clipboard.writeText(url).then(() => {
      setCopiedIp(ip);
      setTimeout(() => setCopiedIp(null), 2000);
    });
  };

  return (
    <div className="dashboard-container">
      <header className="dashboard-header">
        <div className="brand-logo-container">
          <img src="/logo.jpg" alt="UniSystem Logo" className="brand-logo" />
        </div>
        <div className="brand-text">
          <h1>UniSystem AudioShare</h1>
          <p className="subtitle">Stream your PC audio to any device on the network in real-time.</p>
        </div>
      </header>

      <main className="dashboard-content">
        <section className="card audio-card">
          <div className="card-header">
            <h2>LIVE AUDIO BROADCAST</h2>
          </div>
          <div className="audio-visualization-wrapper">
            <div className="visualizer-display">
              {peaks.map((p, i) => (
                <div
                  key={i}
                  className="vis-bar"
                  style={{ 
                    height: `${Math.max(4, p * 80)}px`,
                    opacity: 0.2 + p * 0.8
                  }}
                ></div>
              ))}
            </div>
            <div className="audio-status-footer">
              <span className="status-indicator">
                <span className="dot pulse"></span>
                Broadcasting
              </span>
              <span className="tech-specs">48 kHz · Float32 PCM</span>
            </div>
          </div>
        </section>

        <section className="card network-card">
          <div className="card-header">
            <h2>CONNECT YOUR DEVICES</h2>
            <p className="instruction">Open this URL on your phone's browser to listen instantly.</p>
          </div>
          
          <div className="network-rows">
            {ips.length > 0 ? (
              ips.map((item, idx) => (
                <div key={idx} className="network-row">
                  <div className="row-left">
                    <span className="interface-name">{item.name}</span>
                    <span className="status-badge available">
                      <span className="status-dot"></span> Available
                    </span>
                  </div>
                  
                  <div className="row-right">
                    <a 
                      href="#" 
                      onClick={(e) => {
                        e.preventDefault();
                        openUrl(`http://${item.ip}:8080`);
                      }} 
                      className="url-display"
                    >
                      http://{item.ip}:8080
                    </a>
                    <button 
                      className="copy-btn" 
                      onClick={() => handleCopy(item.ip)}
                    >
                      {copiedIp === item.ip ? "Copied" : "Copy"}
                    </button>
                  </div>
                </div>
              ))
            ) : (
              <div className="empty-state">
                Scanning for active network interfaces...
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
