mod bmp;
mod framebuffer;
mod map;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use map::load_maze;
use player::Player;
use raycaster::{cast_ray, draw_ray, draw_stake};

const BLACK: u32 = 0x000000;
const YELLOW: u32 = 0xD1BC2E;
const WHITE: u32 = 0xFFFFFF;

const BLOCK_SIZE: i32 = 20;
const NUM_RAYS: usize = 5;

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
                            YELLOW,
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
                WHITE,
            );
        }
    }

    // Lanzar 5 rayos dentro del FOV
    for i in 0..NUM_RAYS {
        let ray_fraction = i as f32 / (NUM_RAYS - 1) as f32;

        let angle =
            player.angle - player.fov / 2.0
            + player.fov * ray_fraction;

        let distance = cast_ray(
            &maze,
            &player,
            angle,
        );

        draw_ray(
            &mut framebuffer,
            &player,
            angle,
            distance,
            BLOCK_SIZE,
        );
    }

    // Obtener la distancia del rayo central
    let center_distance = cast_ray(
        &maze,
        &player,
        player.angle,
    );

    println!(
        "Distancia del rayo central: {:.2}",
        center_distance
    );

    // Convertir la distancia del rayo central en una estaca
    draw_stake(
        &mut framebuffer,
        600,
        center_distance,
    );

    framebuffer.render_to_file("out.bmp");
}