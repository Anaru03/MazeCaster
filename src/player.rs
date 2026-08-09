use crate::map::Maze;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 1.5,
            y: 1.5,
            angle: 0.0,
            fov: std::f32::consts::PI / 3.0,
        }
    }

    // Verifica si puede moverse
    fn can_move(&self, maze: &Maze, new_x: f32, new_y: f32) -> bool {
        let map_x = new_x as usize;
        let map_y = new_y as usize;

        if map_y >= maze.len() || map_x >= maze[map_y].len() {
            return false;
        }

        !matches!(maze[map_y][map_x], '+' | '-' | '|')
    }

    // Avanzar
    pub fn move_forward(&mut self, maze: &Maze, speed: f32) {
        let new_x = self.x + self.angle.cos() * speed;
        let new_y = self.y + self.angle.sin() * speed;

        if self.can_move(maze, new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    // Retroceder
    pub fn move_backward(&mut self, maze: &Maze, speed: f32) {
        let new_x = self.x - self.angle.cos() * speed;
        let new_y = self.y - self.angle.sin() * speed;

        if self.can_move(maze, new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    // Girar
    pub fn rotate_left(&mut self, speed: f32) {
        self.angle -= speed;
    }

    pub fn rotate_right(&mut self, speed: f32) {
        self.angle += speed;
    }
}
