use crate::framebuffer::Framebuffer;
use crate::map::Maze;
use crate::player::Player;

// Verifica si una celda es pared
fn is_wall(cell: char) -> bool {
    matches!(cell, '+' | '-' | '|')
}

// Calcula la distancia hasta una pared con DDA
pub fn cast_ray(maze: &Maze, player: &Player, angle: f32) -> f32 {
    let dir_x = angle.cos();
    let dir_y = angle.sin();

    // Celda inicial
    let mut map_x = player.x as i32;
    let mut map_y = player.y as i32;

    // Distancia entre lineas de la cuadricula
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

    // Direccion y distancia inicial
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

    // Recorrer la cuadricula
    loop {
        let hit_x = side_x < side_y;

        if hit_x {
            side_x += delta_x;
            map_x += step_x;
        } else {
            side_y += delta_y;
            map_y += step_y;
        }

        // Evitar salir del mapa
        if map_y < 0
            || map_x < 0
            || map_y as usize >= maze.len()
            || map_x as usize >= maze[map_y as usize].len()
        {
            return if hit_x {
                side_x - delta_x
            } else {
                side_y - delta_y
            };
        }

        // Impacto con pared
        if is_wall(maze[map_y as usize][map_x as usize]) {
            return if hit_x {
                side_x - delta_x
            } else {
                side_y - delta_y
            };
        }
    }
}

// Dibuja el rayo en 2D
pub fn draw_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    angle: f32,
    distance: f32,
    block_size: i32,
) {
    let mut current = 0.0;

    while current < distance {
        let ray_x = player.x + angle.cos() * current;
        let ray_y = player.y + angle.sin() * current;

        let x = (ray_x * block_size as f32) as i32;
        let y = (ray_y * block_size as f32) as i32;

        framebuffer.set_pixel(x, y, 0xFFD6D6);
        current += 0.05;
    }
}

// Dibuja una estaca en 3D
pub fn draw_stake(framebuffer: &mut Framebuffer, x: i32, distance: f32, fov: f32, color: u32) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as i32;
    let horizon = height / 2;

    // Proyeccion de la pared
    let distance = distance.max(0.01);
    let projection = (width / 2.0) / (fov / 2.0).tan();
    let wall_height = (projection / distance) as i32;

    let top = (horizon - wall_height / 2).max(0);
    let bottom = (horizon + wall_height / 2).min(height - 1);

    for y in top..=bottom {
        framebuffer.set_pixel(x, y, color);
    }
}
