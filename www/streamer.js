const sampleRate = 32_000; // khz
const channels = 1;
const bufferLength = Math.floor(sampleRate / 440) * 10;

const audioContext = new window.AudioContext({
    latencyHint: "interactive",
    sampleRate: sampleRate,
});

var buffer1 = audioContext.createBuffer(channels, bufferLength, sampleRate);
var buffer2 = audioContext.createBuffer(channels, bufferLength, sampleRate);

for (buffer of [buffer1, buffer2]) {
    var channel = buffer.getChannelData(0);
    for (var i = 0; i < bufferLength; i++) {
        var run = Math.floor(sampleRate / 440);
        var offset = i % run;
        var rise = -1;
        var slope = rise / run;
        var sample = offset * slope + 1;
        channel[i] = sample;
    }
}

let status = "stopped";

document.addEventListener("keyup", event => {
    if (event.code == "Space") {
        if (status == "running") {
            status = "stopped";
        } else {
            status = "running";
            startPlayback(buffer1, buffer2);
        }
    }
});

function playBuffer(source, nextBuffer) {
    source.start();

    let nextSource = audioContext.createBufferSource();
    nextSource.buffer = nextBuffer;
    nextSource.connect(audioContext.destination);

    source.addEventListener("ended", () => {
        if (status == "stopped") {
            return;
        }
        playBuffer(nextSource, source.buffer);
    });
}

function startPlayback(thisBuffer, nextBuffer) {
    let source = audioContext.createBufferSource();
    source.buffer = thisBuffer;
    source.connect(audioContext.destination);

    playBuffer(source, nextBuffer);
}

//startPlayback(buffer1, buffer2);
