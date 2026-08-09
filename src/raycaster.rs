use crate::framebuffer::Framebuffer;
use crate::map::Maze;
use crate::player::Player;

// Determina si un caracter representa una pared
fn is_wall(cell: char) -> bool {
    cell == '+'
        || cell == '-'
        || cell == '|'
}

// Lanzar un rayo y devolver la distancia
pub fn cast_ray(
    maze: &Maze,
    player: &Player,
    angle: f32,
) -> f32 {
    let mut distance = 0.0;
    let step = 0.05;

    // No usamos for porque no sabemos
    // cuantos pasos necesitara el rayo
    loop {
        let ray_x =
            player.x + angle.cos() * distance;

        let ray_y =
            player.y + angle.sin() * distance;

        let map_x = ray_x as usize;
        let map_y = ray_y as usize;

        // Evitar salir del mapa
        if map_y >= maze.len()
            || map_x >= maze[map_y].len()
        {
            break;
        }

        let cell = maze[map_y][map_x];

        // Detener el rayo cuando encuentre pared
        if is_wall(cell) {
            break;
        }

        distance += step;
    }

    distance
}

// Dibujar un rayo en la vista 2D
pub fn draw_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    angle: f32,
    distance: f32,
    block_size: i32,
) {
    let step = 0.05;
    let mut current_distance = 0.0;

    while current_distance < distance {
        let ray_x =
            player.x
            + angle.cos() * current_distance;

        let ray_y =
            player.y
            + angle.sin() * current_distance;

        let screen_x =
            (ray_x * block_size as f32) as i32;

        let screen_y =
            (ray_y * block_size as f32) as i32;

        framebuffer.set_pixel(
            screen_x,
            screen_y,
            0xFFD6D6,
        );

        current_distance += step;
    }
}

// Dibujar una estaca en la vista 3D
pub fn draw_stake(
    framebuffer: &mut Framebuffer,
    x: i32,
    distance: f32,
) {
    let screen_height =
        framebuffer.height as i32;

    // Horizonte = H / 2
    let horizon =
        screen_height / 2;

    let safe_distance =
        distance.max(0.01);

    // Cerca = pared grande
    // Lejos = pared pequena
    let wall_height =
        (screen_height as f32 / safe_distance) as i32;

    let top =
        horizon - wall_height / 2;

    let bottom =
        horizon + wall_height / 2;

    for y in top..=bottom {
        framebuffer.set_pixel(
            x,
            y,
            0xD1BC2E,
        );
    }
}