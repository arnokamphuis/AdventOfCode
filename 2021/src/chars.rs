use super::tools;
use font8x8::{UnicodeFonts, BASIC_FONTS};
use tools::Image;

#[allow(dead_code)]
pub fn print_num(
    img: &mut Image,
    num: u8,
    x: usize,
    y: usize,
    scale: usize,
    color: (u8, u8, u8, u8),
) {
    let off_x = x * 24;
    let off_y = y * 12;

    let c1_index: u8 = num / 10;

    if c1_index != 0 {
        if let Some(glyph) = BASIC_FONTS.get((c1_index + b'0') as char) {
            for (i, x) in glyph.iter().enumerate() {
                for bit in 0..8 {
                    let coor = (off_x + bit, off_y + i);
                    match *x & 1 << bit {
                        0 => img.set_pixel(scale * coor.0, scale * coor.1, (255, 255, 255, 255)),
                        _ => img.set_pixel(scale * coor.0, scale * coor.1, color),
                    }
                }
            }
        }
    }

    let c2_index: u8 = num % 10;
    if let Some(glyph) = BASIC_FONTS.get((c2_index + b'0') as char) {
        for (i, x) in glyph.iter().enumerate() {
            for bit in 0..8 {
                let coor = (off_x + 8 + bit, off_y + i);
                match *x & 1 << bit {
                    0 => img.set_pixel(scale * coor.0, scale * coor.1, (255, 255, 255, 255)),
                    _ => img.set_pixel(scale * coor.0, scale * coor.1, color),
                }
            }
        }
    }
}
