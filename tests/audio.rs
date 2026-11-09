#[allow(dead_code)]
#[path = "../examples/audio/bike_riding.rs"]
mod bike_riding;
#[allow(dead_code)]
#[path = "../examples/audio/surfing.rs"]
mod surfing;

#[allow(dead_code)]
#[path = "../examples/audio/pokemon_center.rs"]
mod pokemon_center;

#[allow(dead_code)]
#[path = "../examples/audio/let_down.rs"]
mod let_down;

#[allow(dead_code)]
#[path = "../examples/audio/creep.rs"]
mod creep;

use sangoku_audio::{Event, Note, Sequence};

fn expand(events: &[Event], depth: usize) -> Vec<Event> {
    assert!(depth <= 4);
    let mut expanded = Vec::new();
    for &event in events {
        if let Event::Call(phrase) = event {
            expanded.extend(expand(phrase, depth + 1));
        } else {
            expanded.push(event);
        }
    }
    expanded
}

fn score_lengths(score: &[Event], tempo: u8) -> [u32; 2] {
    let Event::Loop(loop_start) = score[score.len() - 1] else {
        panic!("score must end in a loop");
    };
    assert!(loop_start < score.len() - 1);
    let mut speed = 12u32;
    let mut duration = [0u32; 2];
    let intro = expand(&score[..loop_start], 0);
    let repeated = expand(&score[loop_start..score.len() - 1], 0);
    for (index, event) in intro.iter().chain(&repeated).enumerate() {
        let length = match *event {
            Event::NoteType(value, volume, fade) => {
                assert!(volume <= 15 && (-7..=7).contains(&fade));
                speed = value as u32;
                continue;
            }
            Event::Speed(value) => {
                speed = value as u32;
                continue;
            }
            Event::Note(_, length) | Event::Rest(length) | Event::Drum(length) => length,
            Event::Octave(value) => {
                assert!((1..=8).contains(&value));
                continue;
            }
            Event::Duty(value) => {
                assert!(value < 4);
                continue;
            }
            Event::Loop(_) => panic!("unexpected nested loop"),
            _ => continue,
        };
        assert!((1..=16).contains(&length));
        let ticks = length as u32 * speed * tempo as u32;
        assert!((256..=65535 - 255).contains(&ticks));
        duration[usize::from(index >= intro.len())] += ticks;
    }
    duration
}

#[test]
fn bike_riding_channels_have_matching_intro_and_loop_lengths() {
    for score in [
        bike_riding::CH1,
        bike_riding::CH2,
        bike_riding::CH3,
        bike_riding::CH4,
    ] {
        assert_eq!(score_lengths(score, bike_riding::TEMPO), [3456, 525312]);
    }
}

#[test]
fn surfing_preserves_the_delayed_first_voice_and_matching_loops() {
    assert_eq!(score_lengths(surfing::CH1, surfing::TEMPO), [49920, 368640]);
    for score in [surfing::CH2, surfing::CH3] {
        assert_eq!(score_lengths(score, surfing::TEMPO), [46080, 368640]);
    }
}

#[test]
fn pokemon_center_channels_have_matching_loop_lengths() {
    for score in [
        pokemon_center::CH1,
        pokemon_center::CH2,
        pokemon_center::CH3,
    ] {
        assert_eq!(score_lengths(score, pokemon_center::TEMPO), [0, 442368]);
    }
}

#[test]
fn song_calls_expand_to_the_same_event_stream_across_loop_boundaries() {
    for score in [
        bike_riding::CH1,
        bike_riding::CH2,
        bike_riding::CH3,
        bike_riding::CH4,
        surfing::CH1,
        surfing::CH2,
        surfing::CH3,
        pokemon_center::CH1,
        pokemon_center::CH2,
        pokemon_center::CH3,
    ] {
        let Event::Loop(loop_start) = score[score.len() - 1] else {
            panic!("missing loop")
        };
        let first = expand(&score[..score.len() - 1], 0);
        let repeated = expand(&score[loop_start..score.len() - 1], 0);
        let mut sequence = Sequence::new(score);
        for event in first.iter().chain(&repeated) {
            assert!(
                sequence.next_event() == Some(*event),
                "call/return changed the event stream"
            );
        }
    }
}

#[test]
fn nested_and_empty_phrases_return_to_the_next_event() {
    static LEAF: &[Event] = &[Event::NoteType(6, 10, 3), Event::Note(Note::E, 2)];
    static PHRASE: &[Event] = &[
        Event::Call(&[]),
        Event::Octave(3),
        Event::Call(LEAF),
        Event::Note(Note::A, 2),
    ];
    static MAIN: &[Event] = &[
        Event::Call(PHRASE),
        Event::Note(Note::B, 4),
        Event::Call(PHRASE),
    ];
    let mut sequence = Sequence::new(MAIN);
    for event in [
        Event::Octave(3),
        Event::NoteType(6, 10, 3),
        Event::Note(Note::E, 2),
        Event::Note(Note::A, 2),
        Event::Note(Note::B, 4),
        Event::Octave(3),
        Event::NoteType(6, 10, 3),
        Event::Note(Note::E, 2),
        Event::Note(Note::A, 2),
    ] {
        assert!(sequence.next_event() == Some(event));
    }
    assert!(sequence.next_event().is_none());
    assert!(sequence.next_event().is_none());
}

#[test]
fn loops_inside_a_phrase_are_relative_to_that_phrase() {
    static PHRASE: &[Event] = &[Event::Rest(2), Event::Loop(0)];
    static MAIN: &[Event] = &[
        Event::Note(Note::C, 1),
        Event::Call(PHRASE),
        Event::Note(Note::D, 1),
    ];
    let mut sequence = Sequence::new(MAIN);
    assert!(sequence.next_event() == Some(Event::Note(Note::C, 1)));
    for _ in 0..3 {
        assert!(sequence.next_event() == Some(Event::Rest(2)));
    }
}

static LEAF: &[Event] = &[Event::Rest(1)];
static CALL1: &[Event] = &[Event::Call(LEAF)];
static CALL2: &[Event] = &[Event::Call(CALL1)];
static CALL3: &[Event] = &[Event::Call(CALL2)];
static CALL4: &[Event] = &[Event::Call(CALL3)];
static CALL5: &[Event] = &[Event::Call(CALL4)];

#[test]
fn four_nested_phrase_calls_can_return() {
    let mut sequence = Sequence::new(CALL4);
    assert!(sequence.next_event() == Some(Event::Rest(1)));
    assert!(sequence.next_event().is_none());
}

#[test]
#[should_panic]
fn excessive_phrase_nesting_traps_at_the_bounded_depth() {
    Sequence::new(CALL5).next_event();
}

fn excerpt_ticks(score: &'static [Event], tempo: u8) -> u32 {
    let expected = expand(score, 0);
    let mut sequence = Sequence::new(score);
    let mut speed = 12;
    let mut duration = 0u32;
    for event in expected {
        assert!(sequence.next_event() == Some(event));
        match event {
            Event::NoteType(value, _, _) | Event::Speed(value) => speed = value,
            Event::Note(_, length) | Event::Rest(length) | Event::Drum(length) => {
                assert!((1..=16).contains(&length));
                duration += length as u32 * speed as u32 * tempo as u32;
            }
            _ => {}
        }
    }
    assert!(sequence.next_event().is_none());
    duration
}

#[test]
fn let_down_excerpt_ends_with_all_four_channels_synchronized() {
    for score in [let_down::CH1, let_down::CH2, let_down::CH3, let_down::CH4] {
        assert_eq!(excerpt_ticks(score, let_down::TEMPO), 305016);
    }
}

#[test]
fn creep_vocal_and_guitar_parts_cover_the_full_song() {
    for score in [creep::CH1, creep::CH2, creep::CH3] {
        assert_eq!(excerpt_ticks(score, creep::TEMPO), 3551040);
    }
}
