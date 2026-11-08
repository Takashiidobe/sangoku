#![no_std]

#[path = "audio/creep.rs"]
mod creep;
#[path = "audio/player.rs"]
mod player;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    player::initialize(&creep::WAVE);
    let mut channels = const {
        [
            player::Channel::new(creep::CH1, creep::TEMPO),
            player::Channel::new(creep::CH2, creep::TEMPO),
            player::Channel::new(creep::CH3, creep::TEMPO),
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
