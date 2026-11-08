#![no_std]

#[path = "audio/player.rs"]
mod player;
#[path = "audio/pokemon_center.rs"]
mod pokemon_center;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    player::initialize(&pokemon_center::WAVE);
    let mut channels = const {
        [
            player::Channel::new(pokemon_center::CH1, pokemon_center::TEMPO),
            player::Channel::new(pokemon_center::CH2, pokemon_center::TEMPO),
            player::Channel::new(pokemon_center::CH3, pokemon_center::TEMPO),
        ]
    };
    let mut drum = player::Drum::new();
    loop {
        player::wait_frame();
        drum.tick();
        for (index, channel) in channels.iter_mut().enumerate() {
            channel.tick(index as u8, &mut drum);
        }
    }
}
