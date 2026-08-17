use crate::framebuffer::Framebuffer;
use font8x8::{BASIC_FONTS, UnicodeFonts};

pub fn draw_text(
    framebuffer: &mut Framebuffer,
    text: &str,
    x: i32,
    y: i32,
    scale: i32,
    color: u32,
) {
    let mut cursor_x = x;

    for character in text.chars() {
        if let Some(glyph) = BASIC_FONTS.get(character) {
            for (row, bits) in glyph.iter().enumerate() {
                for column in 0..8 {
                    if bits & (1 << column) != 0 {
                        for py in 0..scale {
                            for px in 0..scale {
                                framebuffer.set_pixel(
                                    cursor_x + column * scale + px,
                                    y + row as i32 * scale + py,
                                    color,
                                );
                            }
                        }
                    }
                }
            }
        }

        cursor_x += 9 * scale;
    }
}