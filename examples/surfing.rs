#![no_std]

#[path = "audio/surfing.rs"]
mod surfing;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    sangoku_audio::initialize(&surfing::WAVE);
    let mut channels = const {
        [
            sangoku_audio::Channel::new(surfing::CH1, surfing::TEMPO),
            sangoku_audio::Channel::new(surfing::CH2, surfing::TEMPO),
            sangoku_audio::Channel::new(surfing::CH3, surfing::TEMPO),
        ]
    };
    let mut drum = sangoku_audio::Drum::new();
    loop {
        sangoku_audio::wait_frame();
        drum.tick();
        for (index, channel) in channels.iter_mut().enumerate() {
            channel.tick(index as u8, &mut drum);
        }
    }
}
