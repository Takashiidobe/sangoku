#![no_std]

use core::convert::Infallible;
use core::ptr::{read_volatile, write_volatile};
use sangoku_console::{finish, uWrite, writeln};

const LCDC: *mut u8 = 0xff40 as *mut u8;
const LY: *const u8 = 0xff44 as *const u8;

static FONT: [[u8; 7]; 40] = [
    [0, 0, 0, 0, 0, 0, 0],
    [14, 17, 17, 31, 17, 17, 17],
    [30, 17, 17, 30, 17, 17, 30],
    [14, 17, 16, 16, 16, 17, 14],
    [30, 17, 17, 17, 17, 17, 30],
    [31, 16, 16, 30, 16, 16, 31],
    [31, 16, 16, 30, 16, 16, 16],
    [14, 17, 16, 23, 17, 17, 15],
    [17, 17, 17, 31, 17, 17, 17],
    [14, 4, 4, 4, 4, 4, 14],
    [7, 2, 2, 2, 18, 18, 12],
    [17, 18, 20, 24, 20, 18, 17],
    [16, 16, 16, 16, 16, 16, 31],
    [17, 27, 21, 21, 17, 17, 17],
    [17, 25, 21, 19, 17, 17, 17],
    [14, 17, 17, 17, 17, 17, 14],
    [30, 17, 17, 30, 16, 16, 16],
    [14, 17, 17, 17, 21, 18, 13],
    [30, 17, 17, 30, 20, 18, 17],
    [15, 16, 16, 14, 1, 1, 30],
    [31, 4, 4, 4, 4, 4, 4],
    [17, 17, 17, 17, 17, 17, 14],
    [17, 17, 17, 17, 17, 10, 4],
    [17, 17, 17, 21, 21, 21, 10],
    [17, 17, 10, 4, 10, 17, 17],
    [17, 17, 10, 4, 4, 4, 4],
    [31, 1, 2, 4, 8, 16, 31],
    [14, 17, 19, 21, 25, 17, 14],
    [4, 12, 4, 4, 4, 4, 14],
    [14, 17, 1, 2, 4, 8, 31],
    [30, 1, 1, 14, 1, 1, 30],
    [2, 6, 10, 18, 31, 2, 2],
    [31, 16, 16, 30, 1, 1, 30],
    [14, 16, 16, 30, 17, 17, 14],
    [31, 1, 2, 4, 8, 8, 8],
    [14, 17, 17, 14, 17, 17, 14],
    [14, 17, 17, 15, 1, 1, 14],
    [4, 4, 4, 4, 4, 0, 4],
    [0, 21, 14, 31, 14, 21, 0],
    [0, 0, 31, 0, 31, 0, 0],
];

fn tile(byte: u8) -> u8 {
    match byte {
        b'A'..=b'Z' => byte - b'A' + 1,
        b'0'..=b'9' => byte - b'0' + 27,
        b'!' => 37,
        b'*' => 38,
        b'=' => 39,
        _ => 0,
    }
}

struct Screen {
    column: u8,
    row: u8,
}

impl uWrite for Screen {
    type Error = Infallible;

    fn write_str(&mut self, text: &str) -> Result<(), Infallible> {
        for byte in text.bytes() {
            if self.row >= 18 {
                break;
            }
            if byte == b'\n' {
                self.column = 3;
                self.row += 1;
                continue;
            }
            let address = 0x9800 + self.row as usize * 32 + self.column as usize;
            unsafe { write_volatile(address as *mut u8, tile(byte)) };
            self.column += 1;
            if self.column == 20 {
                self.column = 3;
                self.row += 1;
            }
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    unsafe {
        write_volatile(0xffff as *mut u8, 0);
        if read_volatile(LCDC) & 0x80 != 0 {
            // start at a fresh vblank so setup cannot overrun its last scanline.
            while read_volatile(LY) >= 144 {}
            while read_volatile(LY) < 144 {}
        }
        write_volatile(LCDC, 0);
        for (index, glyph) in FONT.iter().enumerate() {
            let address = 0x8000 + index * 16;
            write_volatile(address as *mut u8, 0);
            write_volatile((address + 1) as *mut u8, 0);
            for (row, bits) in glyph.iter().enumerate() {
                let address = address + 2 + row * 2;
                write_volatile(address as *mut u8, bits << 1);
                write_volatile((address + 1) as *mut u8, bits << 1);
            }
        }
        for offset in 0..1024 {
            write_volatile((0x9800 + offset) as *mut u8, 0);
        }
        write_volatile(0xff42 as *mut u8, 0);
        write_volatile(0xff43 as *mut u8, 0);
        write_volatile(0xff47 as *mut u8, 0xe4);
    }
    let mut screen = Screen { column: 3, row: 4 };
    finish(writeln!(
        &mut screen,
        "SANGOKU\n\nHELLO GAME BOY!\n\nRUST ON SCREEN\n\n6 * 7 = {}",
        6u8 * 7
    ));
    unsafe { write_volatile(LCDC, 0x91) };
    0
}
