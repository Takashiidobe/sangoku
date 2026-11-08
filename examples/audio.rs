#![no_std]

#[path = "audio/bike_riding.rs"]
mod bike_riding;
#[path = "audio/player.rs"]
mod player;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    player::initialize(&bike_riding::WAVE);
    let mut channels = const {
        [
            player::Channel::new(bike_riding::CH1, bike_riding::TEMPO),
            player::Channel::new(bike_riding::CH2, bike_riding::TEMPO),
            player::Channel::new(bike_riding::CH3, bike_riding::TEMPO),
            player::Channel::new(bike_riding::CH4, bike_riding::TEMPO),
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
