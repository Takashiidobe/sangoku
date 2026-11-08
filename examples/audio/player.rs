use core::ptr::{read_volatile, write_volatile};

#[derive(Clone, Copy)]
pub enum Event {
    Note(u8, u8),
    Rest(u8),
    Octave(u8),
    NoteType(u8, u8, i8),
    Duty(u8),
    Vibrato(u8, u8, u8),
    PerfectPitch,
    Drum(u8),
    Speed(u8),
    Loop(usize),
}

const PITCHES: [i16; 12] = [
    -2004, -1891, -1785, -1685, -1590, -1501, -1417, -1337, -1262, -1192, -1125, -1062,
];
const WAVE: [u8; 16] = [
    0x02, 0x46, 0x8a, 0xcd, 0xef, 0xfe, 0xde, 0xff, 0xee, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
];

fn read(address: usize) -> u8 {
    unsafe { read_volatile(address as *const u8) }
}

fn write(address: usize, value: u8) {
    unsafe { write_volatile(address as *mut u8, value) }
}

fn pan(channel: u8, enabled: bool) {
    let mask = 0x11u8 << channel;
    let value = read(0xff25) & !mask;
    write(0xff25, if enabled { value | mask } else { value });
}

pub fn initialize() {
    write(0xffff, 0);
    write(0xff26, 0);
    write(0xff26, 0x80);
    write(0xff24, 0x77);
    write(0xff25, 0);
    write(0xff10, 0);
    write(0xff1a, 0);
    for (offset, value) in WAVE.iter().enumerate() {
        write(0xff30 + offset, *value);
    }
    write(0xff1a, 0x80);
    write(0xff40, 0x80);
}

pub fn wait_frame() {
    while read(0xff44) >= 144 {}
    while read(0xff44) < 144 {}
}

pub struct Channel {
    score: &'static [Event],
    cursor: usize,
    remaining: u8,
    fraction: u16,
    speed: u8,
    octave: u8,
    envelope: u8,
    duty: u8,
    perfect: bool,
    pitch: u16,
    sounding: bool,
    vibrato_delay: u8,
    vibrato_left: u8,
    vibrato_depth: u8,
    vibrato_rate: u8,
    vibrato_counter: u8,
    vibrato_up: bool,
}

impl Channel {
    pub const fn new(score: &'static [Event]) -> Self {
        Self {
            score,
            cursor: 0,
            remaining: 0,
            fraction: 0,
            speed: 12,
            octave: 4,
            envelope: 0,
            duty: 2,
            perfect: false,
            pitch: 0,
            sounding: false,
            vibrato_delay: 0,
            vibrato_left: 0,
            vibrato_depth: 0,
            vibrato_rate: 0,
            vibrato_counter: 0,
            vibrato_up: false,
        }
    }

    fn duration(&mut self, length: u8) {
        let duration = length as u16 * self.speed as u16 * 144 + self.fraction;
        self.fraction = duration & 255;
        self.remaining = (duration >> 8) as u8;
    }

    fn vibrato(&mut self, channel: u8) {
        if self.vibrato_left != 0 {
            self.vibrato_left -= 1;
        } else if self.sounding && self.vibrato_depth != 0 {
            if self.vibrato_counter != 0 {
                self.vibrato_counter -= 1;
            } else {
                self.vibrato_counter = self.vibrato_rate;
                self.vibrato_up = !self.vibrato_up;
                let low = self.pitch as u8;
                let low = if self.vibrato_up {
                    low.saturating_add(self.vibrato_depth.div_ceil(2))
                } else {
                    low.saturating_sub(self.vibrato_depth / 2)
                };
                write(0xff13 + channel as usize * 5, low);
            }
        }
    }

    pub fn tick(&mut self, channel: u8, drum: &mut Drum) {
        if self.remaining > 1 {
            self.remaining -= 1;
            self.vibrato(channel);
            return;
        }
        self.vibrato_left = self.vibrato_delay;
        loop {
            let event = self.score[self.cursor];
            self.cursor += 1;
            match event {
                Event::Octave(value) => self.octave = value,
                Event::Speed(value) => self.speed = value,
                Event::NoteType(speed, volume, fade) => {
                    self.speed = speed;
                    self.envelope = if channel == 2 {
                        volume << 5
                    } else if fade < 0 {
                        volume << 4 | 8 | (-fade as u8)
                    } else {
                        volume << 4 | fade as u8
                    };
                }
                Event::Duty(value) => self.duty = value,
                Event::Vibrato(delay, depth, rate) => {
                    self.vibrato_delay = delay;
                    self.vibrato_left = delay;
                    self.vibrato_depth = depth;
                    self.vibrato_rate = rate;
                    self.vibrato_counter = rate;
                }
                Event::PerfectPitch => self.perfect = !self.perfect,
                Event::Loop(cursor) => self.cursor = cursor,
                Event::Rest(length) => {
                    if channel != 3 {
                        pan(channel, false);
                    }
                    self.sounding = false;
                    self.duration(length);
                    return;
                }
                Event::Drum(length) => {
                    drum.start();
                    self.duration(length);
                    return;
                }
                Event::Note(note, length) => {
                    let pitch = ((PITCHES[note as usize] >> (self.octave - 1)) + 2048) as u16;
                    self.pitch = pitch + self.perfect as u16;
                    self.sounding = true;
                    let base = 0xff10 + channel as usize * 5;
                    if channel < 2 {
                        write(base + 1, self.duty << 6);
                    }
                    write(base + 2, self.envelope);
                    write(base + 3, self.pitch as u8);
                    write(base + 4, 0x80 | (self.pitch >> 8) as u8);
                    pan(channel, true);
                    self.duration(length);
                    return;
                }
            }
        }
    }
}

pub struct Drum {
    remaining: u8,
    second: bool,
}

impl Drum {
    pub const fn new() -> Self {
        Self {
            remaining: 0,
            second: false,
        }
    }

    fn start(&mut self) {
        write(0xff21, 0x91);
        write(0xff22, 40);
        write(0xff23, 0x80);
        pan(3, true);
        self.remaining = 3;
        self.second = false;
    }

    pub fn tick(&mut self) {
        if self.remaining == 0 {
            return;
        }
        self.remaining -= 1;
        if self.remaining == 0 {
            if self.second {
                pan(3, false);
            } else {
                write(0xff21, 0x71);
                write(0xff22, 24);
                write(0xff23, 0x80);
                self.second = true;
                self.remaining = 1;
            }
        }
    }
}
