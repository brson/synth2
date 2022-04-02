const sampleRate = 32_000; // khz
const channels = 1;
const bufferLength = 256 * 4;

const audioContext = new window.AudioContext({
    latencyHint: "interactive",
    sampleRate: sampleRate,
});

var buffer1 = audioContext.createBuffer(channels, bufferLength, sampleRate);
var buffer2 = audioContext.createBuffer(channels, bufferLength, sampleRate);
