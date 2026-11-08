#[allow(dead_code)]
#[path = "../examples/audio/bike_riding.rs"]
mod bike_riding;
#[allow(dead_code)]
#[path = "../examples/audio/player.rs"]
mod player;

#[allow(dead_code)]
#[path = "../examples/audio/surfing.rs"]
mod surfing;

use player::Event;

fn score_lengths(score: &[Event], tempo: u8) -> [u32; 2] {
    let Event::Loop(loop_start) = score[score.len() - 1] else {
        panic!("score must end in a loop");
    };
    assert!(loop_start < score.len() - 1);
    let mut speed = 12u32;
    let mut duration = [0u32; 2];
    for (index, event) in score[..score.len() - 1].iter().enumerate() {
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
            Event::Note(pitch, length) => {
                assert!(pitch < 12);
                length
            }
            Event::Rest(length) | Event::Drum(length) => length,
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
        duration[usize::from(index >= loop_start)] += ticks;
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
