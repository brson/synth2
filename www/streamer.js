const sampleRate = 32_000; // khz
const channels = 1;
const bufferLength = Math.floor(sampleRate / 440) * 20;

const audioContext = new window.AudioContext({
    latencyHint: "interactive",
    sampleRate: sampleRate,
});

var buffer1 = audioContext.createBuffer(channels, bufferLength, sampleRate);
var buffer2 = audioContext.createBuffer(channels, bufferLength, sampleRate);
var buffer3 = audioContext.createBuffer(channels, bufferLength, sampleRate);
var buffer4 = audioContext.createBuffer(channels, bufferLength, sampleRate);

for (buffer of [buffer1, buffer2, buffer3, buffer4]) {
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
            startPlayback();
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
        if (status == "stopped") {
            return;
        }

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

    playBuffer(source1, playTime1, buffer3);
    playBuffer(source2, playTime2, buffer4);
}
