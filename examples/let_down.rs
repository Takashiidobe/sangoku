#![no_std]

#[path = "audio/let_down.rs"]
mod let_down;
#[path = "audio/player.rs"]
mod player;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    player::initialize(&let_down::WAVE);
    let mut channels = const {
        [
            player::Channel::new(let_down::CH1, let_down::TEMPO),
            player::Channel::new(let_down::CH2, let_down::TEMPO),
            player::Channel::new(let_down::CH3, let_down::TEMPO),
            player::Channel::new(let_down::CH4, let_down::TEMPO),
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
