#![no_std]

const EXPECTED: [char; 8] = [
    '\u{7f}',
    '\u{80}',
    '\u{7ff}',
    '\u{800}',
    '\u{ffff}',
    '\u{10000}',
    '\u{10ffff}',
    'a',
];
const OFFSETS: [u8; 8] = [0, 1, 3, 5, 8, 11, 15, 19];

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let text: &str =
        core::hint::black_box("\u{7f}\u{80}\u{7ff}\u{800}\u{ffff}\u{10000}\u{10ffff}a");
    let mut forward = 0u8;
    for (index, (offset, c)) in text.char_indices().enumerate() {
        if EXPECTED.get(index) == Some(&c) && OFFSETS.get(index) == Some(&(offset as u8)) {
            forward += 1;
        }
    }
    let mut backward = 0u8;
    for (index, (offset, c)) in text.char_indices().rev().enumerate() {
        let expected = 7usize.wrapping_sub(index);
        if EXPECTED.get(expected) == Some(&c) && OFFSETS.get(expected) == Some(&(offset as u8)) {
            backward += 1;
        }
    }
    let boundaries = (0..=text.len())
        .filter(|&i| text.is_char_boundary(i))
        .count() as u8;
    forward + backward + boundaries
}
