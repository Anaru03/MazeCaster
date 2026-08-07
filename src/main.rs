mod bmp;
mod framebuffer;
mod map;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use map::load_maze;
use player::Player;
use raycaster::{cast_ray, draw_ray};

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;
const YELLOW: u32 = 0xFFFF00;

const BLOCK_SIZE: i32 = 20;

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

    let mut framebuffer = Framebuffer::new(800, 600, BLACK);

    // Dibujar las paredes del laberinto
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '#' {
                for py in 0..BLOCK_SIZE {
                    for px in 0..BLOCK_SIZE {
                        framebuffer.set_pixel(
                            x as i32 * BLOCK_SIZE + px,
                            y as i32 * BLOCK_SIZE + py,
                            WHITE,
                        );
                    }
                }
            }
        }
    }

    // Convertir la posicion del jugador a pixeles
    let player_screen_x = (player.x * BLOCK_SIZE as f32) as i32;
    let player_screen_y = (player.y * BLOCK_SIZE as f32) as i32;

    // Dibujar al jugador
    for y in -3..=3 {
        for x in -3..=3 {
            framebuffer.set_pixel(
                player_screen_x + x,
                player_screen_y + y,
                YELLOW,
            );
        }
    }

    // Lanzar el primer rayo
    let distance = cast_ray(
        &maze,
        &player,
        player.angle,
    );

    println!(
        "Distancia del primer rayo: {:.2}",
        distance
    );

    // Dibujar el rayo
    draw_ray(
        &mut framebuffer,
        &player,
        player.angle,
        distance,
        BLOCK_SIZE,
    );

    framebuffer.render_to_file("out.bmp");
}