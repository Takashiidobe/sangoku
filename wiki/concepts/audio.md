# Playing audio

Build the bike-riding example and open it in SameBoy:

```bash
cargo run -- build examples/audio.rs build/audio.gb --opt-level z
sameboy build/audio.gb
```

Build the three-channel Surfing example with the same player:

```bash
cargo run -- build examples/surfing.rs build/surfing.gb --opt-level z
sameboy build/surfing.gb
```

Surfing uses tempo 160 and wave instrument 0, with no noise percussion. Its
first pulse channel has a 195-frame introduction; the other two have 180-frame
introductions. This preserves the score's delayed first voice. Every repeating
section lasts 1,440 frames. The [Surfing score](../../examples/audio/surfing.rs)
is transcribed from the supplied
[pret/pokered music](https://github.com/pret/pokered/blob/master/audio/music/surfing.asm).

The bike-riding example loops the supplied Pokémon Red bike-riding score using
two pulse channels, a wave channel, and noise percussion. The screen stays
blank; audio plays continuously. Use an external emulator with sound enabled.
The built-in runner does not emulate the APU or frame timing.

[`audio.rs`](../../examples/audio.rs) initializes the APU and advances the
player once per video frame. The [score](../../examples/audio/bike_riding.rs)
uses Rust events for notes, rests, octaves, duty cycles, envelopes, vibrato, and
loops. The [player](../../examples/audio/player.rs) writes the sound registers
directly and uses no heap or floating-point arithmetic.

Note durations carry an 8-bit fractional frame remainder between notes.
`tempo 144` is the original engine's timing parameter, not a BPM value. All four
tracks have a 13.5-frame introduction and a 2,052-frame repeating section. The
drum part expands its repeated patterns and plays instrument 16's two noise
stages. The wave channel uses instrument 3's waveform.

The score was transcribed from the user-supplied
[pret/pokered bike-riding music](https://github.com/pret/pokered/blob/master/audio/music/bikeriding.asm).
Pitch, timing, vibrato, waveform, and drum conventions follow its
[audio engine](https://github.com/pret/pokered/blob/master/audio/engine_1.asm).
This example implements the commands this score uses; it is not a general
Pokémon sound-engine port or a promise of cycle-identical playback.

Run the bounded score checks with:

```bash
cargo nextest run --no-fail-fast --test audio
```
