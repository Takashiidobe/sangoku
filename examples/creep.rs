#![no_std]

#[path = "audio/creep.rs"]
mod creep;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    sangoku_audio::initialize(&creep::WAVE);
    let mut channels = const {
        [
            sangoku_audio::Channel::new(creep::CH1, creep::TEMPO),
            sangoku_audio::Channel::new(creep::CH2, creep::TEMPO),
            sangoku_audio::Channel::new(creep::CH3, creep::TEMPO),
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
