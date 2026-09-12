const connectBtn = document.getElementById('connectBtn');
const statusText = document.getElementById('status');

let audioCtx;
let nextPlayTime = 0;
let ws;

connectBtn.addEventListener('click', async () => {
    connectBtn.disabled = true;
    
    // Initialize Web Audio API
    audioCtx = new (window.AudioContext || window.webkitAudioContext)({
        sampleRate: 48000,
        latencyHint: 'interactive'
    });
    
    statusText.innerText = "Connecting to AudioShare stream...";
    
    const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${wsProtocol}//${window.location.host}/ws`;
    
    ws = new WebSocket(wsUrl);
    ws.binaryType = "arraybuffer";
    
    ws.onopen = () => {
        statusText.innerText = "Connected! Receiving audio...";
        statusText.style.color = "#4CAF50";
        nextPlayTime = audioCtx.currentTime + 0.1; // 100ms jitter buffer
    };
    
    ws.onmessage = (event) => {
        const float32Array = new Float32Array(event.data);
        // Assuming 2 channels, interleaved
        const numChannels = 2;
        const numSamples = float32Array.length / numChannels;
        
        if (numSamples === 0) return;
        
        const audioBuffer = audioCtx.createBuffer(numChannels, numSamples, 48000);
        const channel0 = audioBuffer.getChannelData(0);
        const channel1 = audioBuffer.getChannelData(1);
        
        for (let i = 0; i < numSamples; i++) {
            channel0[i] = float32Array[i * 2];
            channel1[i] = float32Array[i * 2 + 1];
        }
        
        const source = audioCtx.createBufferSource();
        source.buffer = audioBuffer;
        source.connect(audioCtx.destination);
        
        // Schedule playback
        if (nextPlayTime < audioCtx.currentTime) {
            nextPlayTime = audioCtx.currentTime + 0.05; // Reset if we fell behind
        }
        
        source.start(nextPlayTime);
        nextPlayTime += audioBuffer.duration;
    };
    
    ws.onerror = (e) => {
        statusText.innerText = "Connection error!";
        statusText.style.color = "#f44336";
        connectBtn.disabled = false;
    };
    
    ws.onclose = () => {
        statusText.innerText = "Disconnected.";
        statusText.style.color = "#f44336";
        connectBtn.disabled = false;
    };
});
