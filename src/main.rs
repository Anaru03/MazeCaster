mod bmp;
mod framebuffer;
mod map;
mod player;
mod raycaster;
mod texture;

use framebuffer::Framebuffer;
use map::{Maze, load_maze};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use player::Player;
use raycaster::{cast_ray, draw_ray, draw_stake};
use texture::Texture;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;
const GREEN: u32 = 0x4FAE4F;

const NUM_RAYS_2D: usize = 5;
const MOVE_SPEED: f32 = 0.4;
const ROTATION_SPEED: f32 = 0.3;

enum ViewMode {
    Mode2D,
    Mode3D,
}

fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: i32) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if !matches!(*cell, '+' | '-' | '|') {
                let size = (block_size / 4).max(1);

                for py in 0..block_size {
                    for px in 0..block_size {
                        let screen_x = x as i32 * block_size + px;
                        let screen_y = y as i32 * block_size + py;

                        let checker_x = screen_x / size;
                        let checker_y = screen_y / size;

                        let color = if (checker_x + checker_y) % 2 == 0 {
                            0xE8E8E8
                        } else {
                            0x111111
                        };

                        framebuffer.set_pixel(screen_x, screen_y, color);
                    }
                }
            }

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

    let player_x = (player.x * block_size as f32) as i32;
    let player_y = (player.y * block_size as f32) as i32;

    for y in -3..=3 {
        for x in -3..=3 {
            framebuffer.set_pixel(player_x + x, player_y + y, WHITE);
        }
    }

    for i in 0..NUM_RAYS_2D {
        let fraction = i as f32 / (NUM_RAYS_2D - 1) as f32;
        let angle = player.angle - player.fov / 2.0 + player.fov * fraction;

        let (distance, _) = cast_ray(maze, player, angle);

        draw_ray(framebuffer, player, angle, distance, block_size);
    }
}

fn render_plane(framebuffer: &mut Framebuffer, player: &Player, texture: &Texture, ceiling: bool) {
    let horizon = HEIGHT / 2;

    let dir_x = player.angle.cos();
    let dir_y = player.angle.sin();

    let plane = (player.fov / 2.0).tan();
    let plane_x = -dir_y * plane;
    let plane_y = dir_x * plane;

    let ray_x0 = dir_x - plane_x;
    let ray_y0 = dir_y - plane_y;
    let ray_x1 = dir_x + plane_x;
    let ray_y1 = dir_y + plane_y;

    let rows = if ceiling {
        0..horizon
    } else {
        (horizon + 1)..HEIGHT
    };

    for y in rows {
        let p = if ceiling {
            horizon as f32 - y as f32
        } else {
            y as f32 - horizon as f32
        };

        let distance = (HEIGHT as f32 / 2.0) / p;

        let step_x = distance * (ray_x1 - ray_x0) / WIDTH as f32;
        let step_y = distance * (ray_y1 - ray_y0) / WIDTH as f32;

        let mut world_x = player.x + distance * ray_x0;
        let mut world_y = player.y + distance * ray_y0;

        for x in 0..WIDTH {
            let u = world_x.rem_euclid(1.0);
            let v = world_y.rem_euclid(1.0);

            let color = texture.get_color(u, v);

            framebuffer.set_pixel(x as i32, y as i32, color);

            world_x += step_x;
            world_y += step_y;
        }
    }
}

fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    wall: &Texture,
    ceiling: &Texture,
    floor: &Texture,
) {
    render_plane(framebuffer, player, ceiling, true);
    render_plane(framebuffer, player, floor, false);

    for x in 0..WIDTH {
        let fraction = x as f32 / (WIDTH - 1) as f32;
        let angle = player.angle - player.fov / 2.0 + player.fov * fraction;

        let (distance, texture_u) = cast_ray(maze, player, angle);

        let distance = distance * (angle - player.angle).cos();

        draw_stake(framebuffer, x as i32, distance, player.fov, texture_u, wall);
    }
}

fn framebuffer_to_window(framebuffer: &Framebuffer) -> Vec<u32> {
    let mut output = vec![0; framebuffer.width * framebuffer.height];

    for (i, pixel) in output.iter_mut().enumerate() {
        let r = framebuffer.buffer[i * 3] as u32;
        let g = framebuffer.buffer[i * 3 + 1] as u32;
        let b = framebuffer.buffer[i * 3 + 2] as u32;

        *pixel = (r << 16) | (g << 8) | b;
    }

    output
}

fn main() {
    let maze = load_maze("maze.txt");

    let wall = Texture::load("assets/wall.jpeg");
    let ceiling = Texture::load("assets/techo.jpg");
    let floor = Texture::load("assets/floor.jpg");

    let maze_width = maze.iter().map(|row| row.len()).max().unwrap_or(1);
    let block_size = (WIDTH / maze_width).min(HEIGHT / maze.len()) as i32;

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

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, BLACK);

    let mut window = Window::new(
        "MazeCaster | Flechas: mover | C: vista | ESC: salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    let mut view = ViewMode::Mode2D;
    let mut won = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
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

        if player.reached_goal(&maze) && !won {
            println!("Has sobrevivido la primera noche");
            won = true;
        }

        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            view = match view {
                ViewMode::Mode2D => ViewMode::Mode3D,
                ViewMode::Mode3D => ViewMode::Mode2D,
            };
        }

        framebuffer.clear();

        match view {
            ViewMode::Mode2D => {
                render_2d(&mut framebuffer, &maze, &player, block_size);
            }

            ViewMode::Mode3D => {
                render_3d(&mut framebuffer, &maze, &player, &wall, &ceiling, &floor);
            }
        }

        let output = framebuffer_to_window(&framebuffer);

        window
            .update_with_buffer(&output, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
