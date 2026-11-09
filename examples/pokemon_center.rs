#![no_std]

#[path = "audio/pokemon_center.rs"]
mod pokemon_center;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    sangoku_audio::initialize(&pokemon_center::WAVE);
    let mut channels = const {
        [
            sangoku_audio::Channel::new(pokemon_center::CH1, pokemon_center::TEMPO),
            sangoku_audio::Channel::new(pokemon_center::CH2, pokemon_center::TEMPO),
            sangoku_audio::Channel::new(pokemon_center::CH3, pokemon_center::TEMPO),
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
