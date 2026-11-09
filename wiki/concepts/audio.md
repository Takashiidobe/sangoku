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

Its
[original score](https://github.com/pret/pokered/blob/master/audio/music/pokecenter.asm)
uses tempo 144 and wave instrument 0.

Build the opening Let Down arrangement from the supplied MIDI:

```bash
cargo run -- build examples/let_down.rs build/let_down.gb --opt-level z
sameboy build/let_down.gb
```

```bash
cargo run -- build examples/creep.rs build/creep.gb --opt-level z
sameboy build/creep.gb
```

[`audio.rs`](../../examples/audio.rs) initializes the APU and advances the
player once per video frame. The [score](../../examples/audio/bike_riding.rs)
uses Rust events for notes, rests, octaves, duty cycles, envelopes, vibrato, and
loops. Pitches use the shared `Note` enum, including `CSharp`, `FSharp`, and
other semitones, so score entries such as `Note(CSharp, 2)` cannot contain an
invalid numeric pitch. The player is the runtime crate
[`sangoku_audio`](../../runtime/audio/src/lib.rs), linked like `sangoku_console`:
programs and scores write `use sangoku_audio::{Event, Note::*};` and need no
`#[path]` module. It writes the sound registers directly and uses no heap or
floating-point arithmetic. Only the player items a program reaches are linked.

Moving the player out of each example into the crate made all five audio ROMs
196 bytes smaller (for example Surfing 9944 to 9747 used bytes at opt z). The
cause was not investigated.

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
