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

Build Pokémon Center with its shared accompaniment phrases:

```bash
cargo run -- build examples/pokemon_center.rs build/pokemon_center.gb --opt-level z
sameboy build/pokemon_center.gb
```

The [Pokémon Center score](../../examples/audio/pokemon_center.rs) preserves the
supplied `sound_call` subroutines. All three channels loop every 1,728 frames
without an introduction. Its
[original score](https://github.com/pret/pokered/blob/master/audio/music/pokecenter.asm)
uses tempo 144 and wave instrument 0.

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
loops. Pitches use the shared `Note` enum, including `CSharp`, `FSharp`, and
other semitones, so score entries such as `Note(CSharp, 2)` cannot contain an
invalid numeric pitch. The [player](../../examples/audio/player.rs) writes the
sound registers directly and uses no heap or floating-point arithmetic.

Note durations carry an 8-bit fractional frame remainder between notes.
`tempo 144` is the original engine's timing parameter, not a BPM value. All four
tracks have a 13.5-frame introduction and a 2,052-frame repeating section. The
drum part calls three shared patterns and plays instrument 16's two noise
stages. The wave channel uses instrument 3's waveform.

The score was transcribed from the user-supplied
[pret/pokered bike-riding music](https://github.com/pret/pokered/blob/master/audio/music/bikeriding.asm).
Pitch, timing, vibrato, waveform, and drum conventions follow its
[audio engine](https://github.com/pret/pokered/blob/master/audio/engine_1.asm).
This example implements the commands this score uses; it is not a general
Pokémon sound-engine port or a promise of cycle-identical playback.

## Shared phrases

Define each phrase once and call it from a score:

```rust
static PHRASE: &[Event] = &[Note(E, 2), Note(A, 2)];
static MAIN: &[Event] = &[Call(PHRASE), Note(B, 4), Call(PHRASE), Loop(0)];
```

`Call` switches to the referenced array; reaching its end resumes the caller at
the next event. Octave, envelope, speed, vibrato, and fractional timing remain
channel state across calls. `Loop` indexes the current array. The player
supports four nested calls using a fixed return stack; deeper or recursive calls
trap. No score expansion or heap allocation happens during playback.

Phrase data is stored once, with references at call sites. Events are still
unpacked Rust enums, so this is not the original music engine's packed byte
format. Initial channel arrays are constructed in const Rust to avoid repeating
large runtime initialization sequences.

Run the bounded score checks with:

```bash
cargo nextest run --no-fail-fast --test audio
```
