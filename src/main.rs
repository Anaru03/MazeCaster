mod bmp;
mod framebuffer;
mod player;

use framebuffer::Framebuffer;
use player::Player;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, BLACK);

    framebuffer.set_pixel(400, 300, WHITE);

    framebuffer.render_to_file("out.bmp");

    let player = Player::new();

    println!(
        "Jugador: ({}, {}) - Ángulo: {}",
        player.x,
        player.y,
        player.angle
    );
}