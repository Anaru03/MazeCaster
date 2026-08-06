mod bmp;
mod framebuffer;

use framebuffer::Framebuffer;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, BLACK);

    framebuffer.set_pixel(400, 300, WHITE);

    framebuffer.render_to_file("out.bmp");
}