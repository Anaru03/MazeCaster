mod audio;
mod bmp;
mod framebuffer;
mod map;
mod player;
mod raycaster;
mod text;
mod texture;

use audio::Audio;
use framebuffer::{Framebuffer, rgb};
use map::{Maze, load_maze};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use player::Player;
use raycaster::{cast_ray, draw_ray, draw_stake};
use std::f32::consts::PI;
use std::time::Instant;
use text::draw_text;
use texture::Texture;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const RENDER_WIDTH: usize = 400;
const RENDER_HEIGHT: usize = 300;

const MINI_WIDTH: usize = 200;
const MINI_HEIGHT: usize = 150;

const BLACK: u32 = rgb(0, 0, 0);
const WHITE: u32 = rgb(255, 255, 255);
const PLAYER_COLOR: u32 = rgb(0, 255, 255);
const GOAL_COLOR: u32 = rgb(0, 255, 0);
const WIN_COLOR: u32 = rgb(180, 25, 25);

const NUM_RAYS_2D: usize = 5;
const MOVE_SPEED: f32 = 6.0;
const ROTATION_SPEED: f32 = 4.0;
const SPRITE_SPEED: f32 = 1.0;

enum ViewMode {
    Mode2D,
    Mode3D,
}

fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    let maze_width = maze.iter().map(|row| row.len()).max().unwrap_or(1);
    let maze_height = maze.len();

    let block_size = (framebuffer.width / maze_width)
        .min(framebuffer.height / maze_height)
        .max(1) as i32;

    let marker_size = (block_size / 3).max(1);

    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if !matches!(*cell, '+' | '-' | '|' | 'x') {
                let size = (block_size / 4).max(1);

                for py in 0..block_size {
                    for px in 0..block_size {
                        let sx = x as i32 * block_size + px;
                        let sy = y as i32 * block_size + py;

                        let color = if (sx / size + sy / size) % 2 == 0 {
                            rgb(232, 232, 232)
                        } else {
                            rgb(17, 17, 17)
                        };

                        framebuffer.set_pixel(sx, sy, color);
                    }
                }
            }

            if *cell == 'g' {
                let cx = x as i32 * block_size + block_size / 2;
                let cy = y as i32 * block_size + block_size / 2;

                for py in -marker_size..=marker_size {
                    for px in -marker_size..=marker_size {
                        framebuffer.set_pixel(cx + px, cy + py, GOAL_COLOR);
                    }
                }
            }
        }
    }

    let player_x = (player.x * block_size as f32) as i32;
    let player_y = (player.y * block_size as f32) as i32;

    for y in -marker_size..=marker_size {
        for x in -marker_size..=marker_size {
            framebuffer.set_pixel(player_x + x, player_y + y, PLAYER_COLOR);
        }
    }

    for i in 0..NUM_RAYS_2D {
        let fraction = i as f32 / (NUM_RAYS_2D - 1) as f32;
        let angle = player.angle - player.fov / 2.0 + player.fov * fraction;

        let (distance, _, _) = cast_ray(maze, player, angle);

        draw_ray(framebuffer, player, angle, distance, block_size);
    }
}

fn render_plane(framebuffer: &mut Framebuffer, player: &Player, texture: &Texture, ceiling: bool) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let horizon = height / 2;

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
        horizon + 1..height
    };

    for y in rows {
        let p = if ceiling {
            horizon as f32 - y as f32
        } else {
            y as f32 - horizon as f32
        };

        let distance = (height as f32 / 2.0) / p;

        let step_x = distance * (ray_x1 - ray_x0) / width as f32;
        let step_y = distance * (ray_y1 - ray_y0) / width as f32;

        let mut world_x = player.x + distance * ray_x0;
        let mut world_y = player.y + distance * ray_y0;

        for x in 0..width {
            let u = world_x.rem_euclid(1.0);
            let v = world_y.rem_euclid(1.0);

            framebuffer.set_pixel(x as i32, y as i32, texture.get_color(u, v));

            world_x += step_x;
            world_y += step_y;
        }
    }
}

fn render_sprite(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    texture: &Texture,
    sprite_x: f32,
    sprite_y: f32,
) {
    let dx = sprite_x - player.x;
    let dy = sprite_y - player.y;

    let distance = (dx * dx + dy * dy).sqrt();
    let sprite_angle = dy.atan2(dx);

    let mut angle_diff = sprite_angle - player.angle;

    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }

    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let (wall_distance, _, _) = cast_ray(maze, player, sprite_angle);

    if distance >= wall_distance {
        return;
    }

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    let center_x = width / 2.0 + (angle_diff / (player.fov / 2.0)) * (width / 2.0);

    let sprite_height = (height / distance.max(0.1)) as i32;

    let sprite_width = (sprite_height as f32 * texture.width as f32 / texture.height as f32) as i32;

    let left = center_x as i32 - sprite_width / 2;
    let top = framebuffer.height as i32 / 2 - sprite_height / 2;

    for y in 0..sprite_height {
        for x in 0..sprite_width {
            let u = x as f32 / sprite_width as f32;
            let v = y as f32 / sprite_height as f32;

            if texture.is_transparent(u, v) {
                continue;
            }

            let color = texture.get_color(u, v);

            framebuffer.set_pixel(left + x, top + y, color);
        }
    }
}

fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    wall: &Texture,
    poster_wall: &Texture,
    exit: &Texture,
    ceiling: &Texture,
    floor: &Texture,
    freddy: &Texture,
    freddys: &[(f32, f32)],
) {
    render_plane(framebuffer, player, ceiling, true);
    render_plane(framebuffer, player, floor, false);

    let width = framebuffer.width;

    for x in 0..width {
        let fraction = x as f32 / (width - 1) as f32;
        let angle = player.angle - player.fov / 2.0 + player.fov * fraction;

        let (distance, texture_u, cell) = cast_ray(maze, player, angle);
        let corrected_distance = distance * (angle - player.angle).cos();

        let texture = match cell {
            'g' => exit,
            'x' => poster_wall,
            _ => wall,
        };

        draw_stake(
            framebuffer,
            x as i32,
            corrected_distance,
            player.fov,
            texture_u,
            texture,
        );
    }

    for &(freddy_x, freddy_y) in freddys {
        render_sprite(framebuffer, maze, player, freddy, freddy_x, freddy_y);
    }
}

fn scale_3d(source: &Framebuffer, target: &mut Framebuffer) {
    for y in 0..source.height {
        for x in 0..source.width {
            let index = (y * source.width + x) * 3;

            let color = rgb(
                source.buffer[index] as u32,
                source.buffer[index + 1] as u32,
                source.buffer[index + 2] as u32,
            );

            let screen_x = (x * 2) as i32;
            let screen_y = (y * 2) as i32;

            target.set_pixel(screen_x, screen_y, color);
            target.set_pixel(screen_x + 1, screen_y, color);
            target.set_pixel(screen_x, screen_y + 1, color);
            target.set_pixel(screen_x + 1, screen_y + 1, color);
        }
    }
}

fn draw_mini(framebuffer: &mut Framebuffer, mini: &Framebuffer) {
    let start_x = WIDTH - MINI_WIDTH - 10;
    let start_y = HEIGHT - MINI_HEIGHT - 10;

    for y in 0..MINI_HEIGHT {
        for x in 0..MINI_WIDTH {
            let index = (y * MINI_WIDTH + x) * 3;

            let color = rgb(
                mini.buffer[index] as u32,
                mini.buffer[index + 1] as u32,
                mini.buffer[index + 2] as u32,
            );

            framebuffer.set_pixel((start_x + x) as i32, (start_y + y) as i32, color);
        }
    }

    for x in start_x..start_x + MINI_WIDTH {
        framebuffer.set_pixel(x as i32, start_y as i32, WHITE);
        framebuffer.set_pixel(x as i32, (start_y + MINI_HEIGHT - 1) as i32, WHITE);
    }

    for y in start_y..start_y + MINI_HEIGHT {
        framebuffer.set_pixel(start_x as i32, y as i32, WHITE);
        framebuffer.set_pixel((start_x + MINI_WIDTH - 1) as i32, y as i32, WHITE);
    }
}

fn render_image(framebuffer: &mut Framebuffer, image: &Texture) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;

            framebuffer.set_pixel(x as i32, y as i32, image.get_color(u, v));
        }
    }
}

fn render_menu(framebuffer: &mut Framebuffer, image: &Texture, option: usize) {
    render_image(framebuffer, image);

    let start = if option == 0 { ">> START" } else { "   START" };

    let taylor = if option == 1 {
        ">> START WITH TAYLOR"
    } else {
        "   START WITH TAYLOR"
    };

    draw_text(framebuffer, start, 90, 300, 3, WHITE);
    draw_text(framebuffer, taylor, 90, 350, 3, WHITE);
}

fn render_win(framebuffer: &mut Framebuffer, image: &Texture) {
    render_image(framebuffer, image);

    draw_text(framebuffer, "HAS SOBREVIVIDO", 245, 270, 3, WIN_COLOR);
    draw_text(framebuffer, "LA PRIMERA NOCHE", 220, 320, 3, WIN_COLOR);
    draw_text(framebuffer, "ENTER - MENU", 300, 430, 2, WIN_COLOR);
}

fn framebuffer_to_window(framebuffer: &Framebuffer) -> Vec<u32> {
    let mut output = vec![0; framebuffer.width * framebuffer.height];

    for (i, pixel) in output.iter_mut().enumerate() {
        *pixel = rgb(
            framebuffer.buffer[i * 3] as u32,
            framebuffer.buffer[i * 3 + 1] as u32,
            framebuffer.buffer[i * 3 + 2] as u32,
        );
    }

    output
}

fn main() {
    let maze = load_maze("maze.txt");

    let wall = Texture::load("assets/wall.jpeg");
    let poster_wall = Texture::load("assets/poster_wall.png");
    let exit = Texture::load("assets/Exit_door.jpg");
    let ceiling = Texture::load("assets/techo.jpg");
    let floor = Texture::load("assets/floor.jpg");
    let inicio = Texture::load("assets/Inicio.jpg");
    let win_screen = Texture::load("assets/Win_screen.jpg");

    let freddy_r = Texture::load("assets/Freddy_R.png");
    let freddy_l = Texture::load("assets/Freddy_L.png");

    let audio = Audio::new();

    let mut start_x = 1.5;
    let mut start_y = 1.5;

    let mut freddys = Vec::new();

    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'p' {
                start_x = x as f32 + 0.5;
                start_y = y as f32 + 0.5;
            }

            if *cell == 'f' {
                freddys.push((x as f32 + 0.5, y as f32 + 0.5));
            }
        }
    }

    let mut player = Player::new();
    player.x = start_x;
    player.y = start_y;

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, BLACK);
    let mut render_buffer = Framebuffer::new(RENDER_WIDTH, RENDER_HEIGHT, BLACK);
    let mut mini = Framebuffer::new(MINI_WIDTH, MINI_HEIGHT, BLACK);

    let mut window = Window::new(
        "MazeCaster | C: vista | M: menu | ESC: salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    let mut playing = false;
    let mut option = 0;
    let mut view = ViewMode::Mode3D;
    let mut won = false;

    let mut last_frame = Instant::now();
    let animation_start = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        let animation_time = animation_start.elapsed().as_secs_f32();

        let freddy = if (animation_time / SPRITE_SPEED) as i32 % 2 == 0 {
            &freddy_r
        } else {
            &freddy_l
        };

        if playing && !won && window.is_key_down(Key::M) {
            audio.stop_music();

            player.x = start_x;
            player.y = start_y;
            player.angle = 0.0;

            playing = false;
            view = ViewMode::Mode3D;
        }

        if !playing {
            if window.is_key_pressed(Key::Up, KeyRepeat::No)
                || window.is_key_pressed(Key::Down, KeyRepeat::No)
            {
                option = 1 - option;
            }

            if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
                if option == 0 {
                    audio.play_music("assets/sounds/efectos_music.mp3");
                } else {
                    audio.play_music("assets/sounds/taylor.mp3");
                }

                playing = true;
                won = false;
                last_frame = Instant::now();
            }

            framebuffer.clear();
            render_menu(&mut framebuffer, &inicio, option);
        } else if won {
            framebuffer.clear();
            render_win(&mut framebuffer, &win_screen);

            if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
                player.x = start_x;
                player.y = start_y;
                player.angle = 0.0;

                playing = false;
                won = false;
                view = ViewMode::Mode3D;
            }
        } else {
            let move_amount = MOVE_SPEED * delta_time;
            let rotation_amount = ROTATION_SPEED * delta_time;

            if window.is_key_down(Key::Up) {
                player.move_forward(&maze, move_amount);
            }

            if window.is_key_down(Key::Down) {
                player.move_backward(&maze, move_amount);
            }

            if window.is_key_down(Key::Left) {
                player.rotate_left(rotation_amount);
            }

            if window.is_key_down(Key::Right) {
                player.rotate_right(rotation_amount);
            }

            if window.is_key_pressed(Key::C, KeyRepeat::No) {
                view = match view {
                    ViewMode::Mode2D => ViewMode::Mode3D,
                    ViewMode::Mode3D => ViewMode::Mode2D,
                };
            }

            if player.reached_goal(&maze) {
                audio.stop_music();
                audio.play_effect("assets/sounds/Win.mp3");
                println!("Has sobrevivido la primera noche");
                won = true;
            }

            framebuffer.clear();
            mini.clear();

            match view {
                ViewMode::Mode3D => {
                    render_buffer.clear();

                    render_3d(
                        &mut render_buffer,
                        &maze,
                        &player,
                        &wall,
                        &poster_wall,
                        &exit,
                        &ceiling,
                        &floor,
                        freddy,
                        &freddys,
                    );

                    scale_3d(&render_buffer, &mut framebuffer);
                    render_2d(&mut mini, &maze, &player);
                }

                ViewMode::Mode2D => {
                    render_2d(&mut framebuffer, &maze, &player);

                    render_3d(
                        &mut mini,
                        &maze,
                        &player,
                        &wall,
                        &poster_wall,
                        &exit,
                        &ceiling,
                        &floor,
                        freddy,
                        &freddys,
                    );
                }
            }

            draw_mini(&mut framebuffer, &mini);
        }

        let output = framebuffer_to_window(&framebuffer);

        window
            .update_with_buffer(&output, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
