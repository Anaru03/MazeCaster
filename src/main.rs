mod bmp;
mod framebuffer;
mod map;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use map::{load_maze, Maze};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use player::Player;
use raycaster::{cast_ray, draw_ray, draw_stake};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const BLACK: u32 = 0x000000;
const RED: u32 = 0xFF5757;
const BLUE: u32 = 0x12A8E8;
const WHITE: u32 = 0xFFFFFF;
const GREEN: u32 = 0x00FF00;

const NUM_RAYS_2D: usize = 5;

// Velocidad del jugador
const MOVE_SPEED: f32 = 0.10;

// Velocidad de rotacion
const ROTATION_SPEED: f32 = 0.08;

enum ViewMode {
    Mode2D,
    Mode3D,
}

// VISTA 2D
fn render_2d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: i32,
) {
    // Dibujar laberinto
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match *cell {
                '+' | '-' => Some(RED),
                '|' => Some(BLUE),
                _ => None,
            };

            if let Some(wall_color) = color {
                for py in 0..block_size {
                    for px in 0..block_size {
                        framebuffer.set_pixel(
                            x as i32 * block_size + px,
                            y as i32 * block_size + py,
                            wall_color,
                        );
                    }
                }
            }

            // Dibujar meta
            if *cell == 'g' {
                let center_x = x as i32 * block_size + block_size / 2;
                let center_y = y as i32 * block_size + block_size / 2;

                for py in -5..=5 {
                    for px in -5..=5 {
                        framebuffer.set_pixel(center_x + px, center_y + py, GREEN);
                    }
                }
            }
        }
    }

    // JUGADOR
    let player_x = (player.x * block_size as f32) as i32;
    let player_y = (player.y * block_size as f32) as i32;

    for y in -3..=3 {
        for x in -3..=3 {
            framebuffer.set_pixel(player_x + x, player_y + y, WHITE);
        }
    }

    // RAYOS DEL FOV
    for i in 0..NUM_RAYS_2D {
        let ray_fraction = i as f32 / (NUM_RAYS_2D - 1) as f32;
        let angle = player.angle - player.fov / 2.0 + player.fov * ray_fraction;
        let distance = cast_ray(maze, player, angle);

        draw_ray(framebuffer, player, angle, distance, block_size);
    }
}

// VISTA 3D
fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
) {
    // Un rayo por columna
    for x in 0..WIDTH {
        let ray_fraction = x as f32 / (WIDTH - 1) as f32;
        let angle = player.angle - player.fov / 2.0 + player.fov * ray_fraction;
        let distance = cast_ray(maze, player, angle);

        // Correccion fisheye
        let corrected_distance = distance * (angle - player.angle).cos();

        draw_stake(framebuffer, x as i32, corrected_distance);
    }
}

// CONVERTIR FRAMEBUFFER
fn framebuffer_to_window(framebuffer: &Framebuffer) -> Vec<u32> {
    let mut window_buffer = vec![0u32; framebuffer.width * framebuffer.height];

    for i in 0..framebuffer.width * framebuffer.height {
        let r = framebuffer.buffer[i * 3] as u32;
        let g = framebuffer.buffer[i * 3 + 1] as u32;
        let b = framebuffer.buffer[i * 3 + 2] as u32;

        window_buffer[i] = (r << 16) | (g << 8) | b;
    }

    window_buffer
}

fn main() {
    let maze = load_maze("maze.txt");

    // Ajustar el mapa al tamaño de la ventana
    let maze_width = maze.iter().map(|row| row.len()).max().unwrap_or(1);
    let block_size = (WIDTH / maze_width).min(HEIGHT / maze.len()) as i32;

    println!("Mapa cargado: {} filas", maze.len());

    // Buscar posicion inicial del jugador
    let mut start_x = 1.5;
    let mut start_y = 1.5;

    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'p' {
                start_x = x as f32 + 0.5;
                start_y = y as f32 + 0.5;
            }
        }
    }

    let mut player = Player::new();
    player.x = start_x;
    player.y = start_y;

    println!(
        "Jugador: ({}, {}) - Angulo: {}",
        player.x, player.y, player.angle
    );

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, BLACK);

    let mut window = Window::new(
        "MazeCaster | Flechas: mover | C: vista | ESC: salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    let mut view_mode = ViewMode::Mode2D;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // MOVIMIENTO
        if window.is_key_down(Key::Up) {
            player.move_forward(&maze, MOVE_SPEED);
        }
        if window.is_key_down(Key::Down) {
            player.move_backward(&maze, MOVE_SPEED);
        }
        if window.is_key_down(Key::Left) {
            player.rotate_left(ROTATION_SPEED);
        }
        if window.is_key_down(Key::Right) {
            player.rotate_right(ROTATION_SPEED);
        }

        // CAMBIAR VISTA
        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            view_mode = match view_mode {
                ViewMode::Mode2D => ViewMode::Mode3D,
                ViewMode::Mode3D => ViewMode::Mode2D,
            };
        }

        framebuffer.clear();

        // DIBUJAR VISTA ACTUAL
        match view_mode {
            ViewMode::Mode2D => render_2d(
                &mut framebuffer,
                &maze,
                &player,
                block_size,
            ),
            ViewMode::Mode3D => render_3d(
                &mut framebuffer,
                &maze,
                &player,
            ),
        }

        let window_buffer = framebuffer_to_window(&framebuffer);

        window
            .update_with_buffer(&window_buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}