#![no_std]

#[path = "audio/let_down.rs"]
mod let_down;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    sangoku_audio::initialize(&let_down::WAVE);
    let mut channels = const {
        [
            sangoku_audio::Channel::new(let_down::CH1, let_down::TEMPO),
            sangoku_audio::Channel::new(let_down::CH2, let_down::TEMPO),
            sangoku_audio::Channel::new(let_down::CH3, let_down::TEMPO),
            sangoku_audio::Channel::new(let_down::CH4, let_down::TEMPO),
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
