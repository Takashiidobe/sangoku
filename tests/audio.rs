#[path = "../examples/audio/bike_riding.rs"]
mod bike_riding;
#[allow(dead_code)]
#[path = "../examples/audio/player.rs"]
mod player;

use player::Event;

#[test]
fn bike_riding_channels_have_matching_intro_and_loop_lengths() {
    for score in [
        bike_riding::CH1,
        bike_riding::CH2,
        bike_riding::CH3,
        bike_riding::CH4,
    ] {
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
            let ticks = length as u32 * speed * 144;
            assert!((256..=65535 - 255).contains(&ticks));
            duration[usize::from(index >= loop_start)] += ticks;
        }
        assert_eq!(duration, [3456, 525312]);
    }
}
