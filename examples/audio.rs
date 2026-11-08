#![no_std]

#[path = "audio/bike_riding.rs"]
mod bike_riding;
#[path = "audio/player.rs"]
mod player;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    player::initialize();
    let mut channels = [
        player::Channel::new(bike_riding::CH1),
        player::Channel::new(bike_riding::CH2),
        player::Channel::new(bike_riding::CH3),
        player::Channel::new(bike_riding::CH4),
    ];
    let mut drum = player::Drum::new();
    loop {
        player::wait_frame();
        drum.tick();
        for (index, channel) in channels.iter_mut().enumerate() {
            channel.tick(index as u8, &mut drum);
        }
    }
}
