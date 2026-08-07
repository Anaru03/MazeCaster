use crate::framebuffer::Framebuffer;
use crate::map::Maze;
use crate::player::Player;

//RAYO
pub fn cast_ray(
    maze: &Maze,
    player: &Player,
    angle: f32,
) -> f32 { // Devuelve la distancia que recorrio el rayo antes de encontrar una pared
    let mut distance = 0.0; // Origen
    let step = 0.05; // Avanza en pasos de 0.05 unidades
    //Buscar la pared

    // NO USAR FOR, no se sabe cuantos pasos necesitara el rayo
    loop {
        let ray_x = player.x + angle.cos() * distance;
        let ray_y = player.y + angle.sin() * distance;

        let map_x = ray_x as usize;
        let map_y = ray_y as usize;

        if map_y >= maze.len() || map_x >= maze[map_y].len() {
            break;
        }

        if maze[map_y][map_x] == '#' {
            println!(
                "Pared encontrada en ({}, {}) - distancia: {:.2}",
                map_x,
                map_y,
                distance
            );

            break;
        }

        distance += step;
    }

    distance
}

//DIBUJO 
pub fn draw_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    angle: f32, //Hacia dónde apunta el rayo
    distance: f32,
    block_size: i32,
) {
    let step = 0.05;
    let mut current_distance = 0.0; //Dibujar el camino hasta esa pared

    //Saber desde donde comienza el rayo
    while current_distance < distance {
        let ray_x = player.x + angle.cos() * current_distance;
        let ray_y = player.y + angle.sin() * current_distance;

        let screen_x = (ray_x * block_size as f32) as i32;
        let screen_y = (ray_y * block_size as f32) as i32;

        //Aparece el rayo
        framebuffer.set_pixel(
            screen_x,
            screen_y,
            0xFF0000,
        );

        current_distance += step;
    }
}

pub fn draw_stake(
    framebuffer: &mut Framebuffer,
    x: i32,
    distance: f32,
) {
    let screen_height = framebuffer.height as i32;
    let horizon = screen_height / 2;

    let wall_height = (screen_height as f32 / distance) as i32;

    let top = horizon - wall_height / 2;
    let bottom = horizon + wall_height / 2;

    for y in top..=bottom {
        framebuffer.set_pixel(
            x,
            y,
            0xD1BC2E,
        );
    }
}