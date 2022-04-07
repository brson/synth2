const sampleRate = 32_000; // khz
const channels = 1;
const bufferLength = 256 * 4;

const audioContext = new window.AudioContext({
    latencyHint: "interactive",
    sampleRate: sampleRate,
});


let incomingBufferQueue = [];

let buffer1 = audioContext.createBuffer(channels, bufferLength, sampleRate);
let buffer2 = audioContext.createBuffer(channels, bufferLength, sampleRate);
let buffer3 = audioContext.createBuffer(channels, bufferLength, sampleRate);
let buffer4 = audioContext.createBuffer(channels, bufferLength, sampleRate);

/*for (buffer of [buffer1, buffer2, buffer3, buffer4]) {
    let channel = buffer.getChannelData(0);
    for (let i = 0; i < bufferLength; i++) {
        let run = Math.floor(sampleRate / 440);
        let offset = i % run;
        let rise = -1;
        let slope = rise / run;
        let sample = offset * slope + 1;
        channel[i] = sample;
    }
}*/

function fillBuffer(buffer) {
    let channel = buffer.getChannelData(0);
    let newData = incomingBufferQueue.shift();
    if (status == "stopped" || !newData) {
        for (let i = 0; i < bufferLength; i++) {
            channel[i] = 0;
        }
    } else {
        console.assert(newData.length == bufferLength);
        for (let i = 0; i < bufferLength; i++) {
            channel[i] = newData[i];
        }
    }
}

let status = "running";

document.addEventListener("keyup", event => {
    if (event.code == "Space") {
        if (status == "running") {
            status = "stopped";
        } else {
            status = "running";
            //startPlayback();
        }
    }
});

function playBuffer(source, playTime, nextBuffer) {
    source.start(playTime);

    let nextSource = audioContext.createBufferSource();
    nextSource.buffer = nextBuffer;
    nextSource.connect(audioContext.destination);
    let nextPlayTime = playTime + bufferLength / sampleRate * 2;

    source.addEventListener("ended", () => {
        fillBuffer(source.buffer);

        playBuffer(nextSource, nextPlayTime, source.buffer);
    });
}

function startPlayback() {
    let source1 = audioContext.createBufferSource();
    source1.buffer = buffer1;
    source1.connect(audioContext.destination);

    let source2 = audioContext.createBufferSource();
    source2.buffer = buffer2;
    source2.connect(audioContext.destination);

    let playTime1 = audioContext.currentTime;
    let playTime2 = playTime1 + bufferLength / sampleRate;

    fillBuffer(source1.buffer);
    fillBuffer(source2.buffer);
    playBuffer(source1, playTime1, buffer3);
    playBuffer(source2, playTime2, buffer4);
}

startPlayback();


const websocketUrl = "ws://127.0.0.1:9110";

function connect() {
    let ws = new WebSocket(websocketUrl);

    ws.addEventListener("message", (event) => {
        var data = JSON.parse(event.data);
        console.log(data.length);
        incomingBufferQueue.push(data);
    })
}

connect();
