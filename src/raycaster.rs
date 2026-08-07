use crate::map::Maze;
use crate::player::Player;

pub fn cast_ray(
    maze: &Maze,
    player: &Player,
    angle: f32,
) -> f32 {  //Devuelve la distancia que recorrió el rayo antes de encontrar una pared
    let mut distance = 0.0; //Origen
    let step = 0.05; //Avanza en pasos de 0.05 unidades

    //NO USAR FOR, no se sabe cuantos pasos se necesitará el rayo
    loop {
        let ray_x = player.x + angle.cos() * distance;
        let ray_y = player.y + angle.sin() * distance;

        let map_x = ray_x as usize;
        let map_y = ray_y as usize;

        if map_y >= maze.len() || map_x >= maze[map_y].len() {
            break;
        }

        if maze[map_y][map_x] == '#' {
            break;
        }

        distance += step;
    }

    distance
}