mod bmp;
mod framebuffer;
mod map;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use map::load_maze;
use player::Player;
use raycaster::cast_ray;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

fn main() {
    let maze = load_maze("maze.txt");

    println!(
        "Mapa cargado: {} filas x {} columnas",
        maze.len(),
        maze[0].len()
    );

    let player = Player::new();

    println!(
        "Jugador: ({}, {}) - Angulo: {}",
        player.x,
        player.y,
        player.angle
    );

    let distance = cast_ray(
        &maze,
        &player,
        player.angle,
    );

println!("Distancia del primer rayo: {:.2}", distance);
    let mut framebuffer = Framebuffer::new(800, 600, BLACK);

    framebuffer.set_pixel(400, 300, WHITE);

    framebuffer.render_to_file("out.bmp");
}