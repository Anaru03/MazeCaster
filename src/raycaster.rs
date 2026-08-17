use crate::framebuffer::{Framebuffer, rgb};
use crate::map::Maze;
use crate::player::Player;
use crate::texture::Texture;

fn is_wall(cell: char) -> bool {
    matches!(cell, '+' | '-' | '|')
}

pub fn cast_ray(maze: &Maze, player: &Player, angle: f32) -> (f32, f32, bool) {
    let dir_x = angle.cos();
    let dir_y = angle.sin();

    let mut map_x = player.x as i32;
    let mut map_y = player.y as i32;

    let delta_x = if dir_x == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / dir_x).abs()
    };

    let delta_y = if dir_y == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / dir_y).abs()
    };

    let (step_x, mut side_x) = if dir_x < 0.0 {
        (-1, (player.x - map_x as f32) * delta_x)
    } else {
        (1, (map_x as f32 + 1.0 - player.x) * delta_x)
    };

    let (step_y, mut side_y) = if dir_y < 0.0 {
        (-1, (player.y - map_y as f32) * delta_y)
    } else {
        (1, (map_y as f32 + 1.0 - player.y) * delta_y)
    };

    loop {
        let hit_x = side_x < side_y;

        let distance = if hit_x {
            side_x += delta_x;
            map_x += step_x;
            side_x - delta_x
        } else {
            side_y += delta_y;
            map_y += step_y;
            side_y - delta_y
        };

        if map_y < 0
            || map_x < 0
            || map_y as usize >= maze.len()
            || map_x as usize >= maze[map_y as usize].len()
        {
            return (distance, 0.0, false);
        }

        let cell = maze[map_y as usize][map_x as usize];

        if is_wall(cell) || cell == 'g' {
            let hit_x_pos = player.x + distance * dir_x;
            let hit_y_pos = player.y + distance * dir_y;

            let mut texture_u = if hit_x {
                hit_y_pos.rem_euclid(1.0)
            } else {
                hit_x_pos.rem_euclid(1.0)
            };

            if cell == 'g' {
                texture_u = 1.0 - texture_u;
            }

            return (distance, texture_u, cell == 'g');
        }
    }
}

pub fn draw_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    angle: f32,
    distance: f32,
    block_size: i32,
) {
    let mut current = 0.0;
    let red = rgb(255, 0, 0);

    while current < distance {
        let ray_x = player.x + angle.cos() * current;
        let ray_y = player.y + angle.sin() * current;

        let x = (ray_x * block_size as f32) as i32;
        let y = (ray_y * block_size as f32) as i32;

        framebuffer.set_pixel(x, y, red);
        framebuffer.set_pixel(x + 1, y, red);
        framebuffer.set_pixel(x, y + 1, red);

        current += 0.05;
    }
}

pub fn draw_stake(
    framebuffer: &mut Framebuffer,
    x: i32,
    distance: f32,
    fov: f32,
    texture_u: f32,
    texture: &Texture,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as i32;
    let horizon = height / 2;

    let distance = distance.max(0.01);
    let projection = (width / 2.0) / (fov / 2.0).tan();
    let wall_height = (projection / distance) as i32;

    let wall_top = horizon - wall_height / 2;
    let wall_bottom = horizon + wall_height / 2;

    let top = wall_top.max(0);
    let bottom = wall_bottom.min(height - 1);

    for y in top..=bottom {
        let texture_v = (y - wall_top) as f32 / wall_height as f32;
        let color = texture.get_color(texture_u, texture_v);

        framebuffer.set_pixel(x, y, color);
    }
}
