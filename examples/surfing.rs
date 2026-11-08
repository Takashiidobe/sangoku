#![no_std]

#[path = "audio/player.rs"]
mod player;
#[path = "audio/surfing.rs"]
mod surfing;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    player::initialize(&surfing::WAVE);
    let mut channels = [
        player::Channel::new(surfing::CH1, surfing::TEMPO),
        player::Channel::new(surfing::CH2, surfing::TEMPO),
        player::Channel::new(surfing::CH3, surfing::TEMPO),
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
