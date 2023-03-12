# synth one features

- two oscillators
- sine, tri, saw, square, pulse
- pulse width
- noise oscillator with volume
- lp filter with resonance and keytracking
- amp adsr envelope
- filter adsr envelope
- one lfo
- visualizations
  - osc
  - lfo
- mono or poly voices
- unison
- mod wheel modulation
- velocity modulation
- fixed pitch wheel
- no fx


# sources

https://github.com/chaosprint/glicol
https://www.tone2.com/plugin-formats.html
https://cleveraudio.org/1-feature-overview/_midi/#concurrent-event-queues-and-ambiguity
https://www.musicdsp.org/
https://www.dspguide.com/
https://juce.com/
https://www.elementary.audio/
https://www.musicradar.com/how-to/how-to-recreate-classic-analogue-drum-sounds-in-your-daw-and-with-hardware
https://jamesmunns.com/blog/fixed-point-math/
https://docs.rs/music-note/0.3.0/music_note/
https://www.soundonsound.com/techniques/synthesizing-cowbells-claves
https://github.com/greatest-ape/OctaSine
https://docs.rs/vst/latest/vst/
https://thewolfsound.com/sound-synthesis/wavetable-synthesis-algorithm/
https://github.com/juce-framework/JUCE
https://github.com/mtytel/vital
https://github.com/surge-synthesizer
https://github.com/google/music-synthesizer-for-android
https://github.com/free-audio/clap
https://github.com/robbert-vdh/nih-plug
https://github.com/glowcoil/clap-sys
https://sotrh.github.io/learn-wgpu/showcase/windowless
https://cytomic.com/technical-papers/
https://musicnotation.org/systems/
https://clairnote.org/
https://dsp.stackexchange.com/questions/2349/help-with-algorithm-for-modulating-oscillator-pitch-using-lfo
https://dsp.stackexchange.com/questions/971/how-to-create-a-sine-wave-generator-that-can-smoothly-transition-between-frequen
https://www.reddit.com/r/DSP/comments/vkpw0z/butterworthchebyshev_filters_resources/
https://github.com/MikeCurrington/mkfilter
https://ccrma.stanford.edu/~jos/sasp/sasp.html
https://dspguru.com/dsp/books/
https://github.com/airwindows/airwindows
https://www.reddit.com/r/DSP/comments/wl7dn9/single_pole_iir_more_efficient_than_moving/
  - https://www.wavewalkerdsp.com/2022/08/10/single-pole-iir-filter-substitute-for-moving-average-filter/
https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.0.pdf
https://www.maximintegrated.com/en/design/technical-documents/tutorials/7/733.html
https://github.com/ValdemarOrn/CloudSeed
https://www.researchgate.net/profile/Juhan-Nam/publication/224557976_Alias-Suppressed_Oscillators_Based_on_Differentiated_Polynomial_Waveforms/links/573f274d08ae9ace84133dc9/Alias-Suppressed-Oscillators-Based-on-Differentiated-Polynomial-Waveforms.pdf

# ideas

- more fma
- cheaper approximate powf, expf
- faster fmodf? inline fmodf?
- sleef has simd fmodf
- simd floor / trunc? just use float as int cast
- use unsafe float->int casts?
