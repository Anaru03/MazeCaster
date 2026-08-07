use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename)
        .expect("No se pudo abrir el archivo del laberinto");

    let reader = BufReader::new(file);
    let mut maze: Maze = Vec::new();

    for line in reader.lines() {
        let line = line.expect("No se pudo leer una linea del laberinto");
        maze.push(line.chars().collect());
    }

    maze
}