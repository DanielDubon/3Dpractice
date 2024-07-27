mod player;
mod cast_ray;
mod events;

use cast_ray::cast_ray;
use player::Player;
use events::process_events;
use minifb::{Window, WindowOptions};
use rand::seq::SliceRandom;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn generate_maze(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut vis = vec![vec![0; width]; height];
    let mut hor = vec![vec!['+'; width + 1]; height + 1];
    let mut ver = vec![vec!['|'; width + 1]; height];

    fn walk(
        x: usize,
        y: usize,
        vis: &mut Vec<Vec<u8>>,
        hor: &mut Vec<Vec<char>>,
        ver: &mut Vec<Vec<char>>,
    ) {
        vis[y][x] = 1;
        let mut rng = rand::thread_rng();
        let mut dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        dirs.shuffle(&mut rng);

        for (dx, dy) in dirs {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if ny < vis.len() && nx < vis[0].len() && vis[ny][nx] == 0 {
                if dx == 0 {
                    hor[y.max(ny)][x] = ' ';
                } else {
                    ver[y][x.max(nx)] = ' ';
                }
                walk(nx, ny, vis, hor, ver);
            }
        }
    }

    walk(rand::thread_rng().gen_range(0..width), rand::thread_rng().gen_range(0..height), &mut vis, &mut hor, &mut ver);

    let mut maze = vec![vec!['+'; width * 2 + 1]; height * 2 + 1];

    for y in 0..height {
        for x in 0..width {
            maze[y * 2][x * 2 + 1] = hor[y][x];
            maze[y * 2 + 1][x * 2] = ver[y][x];
            maze[y * 2 + 1][x * 2 + 1] = ' ';
        }
    }

    maze[0][1] = 'p'; // Punto de inicio
    maze[height * 2][width * 2 - 1] = 'g'; // Punto final

    maze
}

fn render(maze: &Vec<Vec<char>>, window: &mut Window, player: &Player) {
    let width = window.get_size().0;
    let height = window.get_size().1;
    let cell_size = 20; // Tamaño de cada celda en píxeles

    let mut buffer: Vec<u32> = vec![0; width * height];

    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '|' => 0x800080, // Morado para paredes
                ' ' => 0xFFFFFF, // Blanco para espacio libre
                'p' => 0xF00FFF, // Lila para punto de inicio
                'g' => 0x00FF00, // Verde para objetivo
                _ => 0x000000,   // Negro para caracteres desconocidos
            };

            let pixel_x = x * cell_size;
            let pixel_y = y * cell_size;

            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    if pixel_x + dx < width && pixel_y + dy < height {
                        buffer[(pixel_y + dy) * width + (pixel_x + dx)] = color;
                    }
                }
            }
        }
    }

    // Dibuja el jugador como un pixel rojo más pequeño
    let player_size = cell_size / 2; // Tamaño reducido del jugador
    let player_x = (player.x / cell_size as f32).floor() as usize;
    let player_y = (player.y / cell_size as f32).floor() as usize;
    let px = player_x * cell_size + (cell_size - player_size) / 2;
    let py = player_y * cell_size + (cell_size - player_size) / 2;

    for dy in 0..player_size {
        for dx in 0..player_size {
            if px + dx < width && py + dy < height {
                buffer[(py + dy) * width + (px + dx)] = 0xFF0000; // Color rojo para el jugador
            }
        }
    }

    // Render del rayo
    cast_ray(&maze, player, &mut buffer, width, height, cell_size);

    window.update_with_buffer(&buffer, width, height).unwrap();
}



fn save_maze_to_file(maze: &Vec<Vec<char>>, filename: &str) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    for row in maze {
        writeln!(writer, "{}", row.iter().collect::<String>()).expect("Unable to write data");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let width = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(16);
    let height = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(8);

    let maze = generate_maze(width, height);
    let cell_size = 20;
    let window_width = (width * 2 + 1) * cell_size; // Ajuste el tamaño de la ventana según el laberinto
    let window_height = (height * 2 + 1) * cell_size; // Ajuste el tamaño de la ventana según el laberinto
    let mut window = Window::new("Maze Renderer", window_width, window_height, WindowOptions::default()).unwrap();

    // Crear el jugador en la posición inicial (donde 'p' se encuentra)
    let mut player = Player::new(1.5 * cell_size as f32, 0.5 * cell_size as f32, std::f32::consts::PI / 3.0);

    // Guardar el laberinto en el archivo al iniciar
    save_maze_to_file(&maze, "maze.txt");

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        process_events(&window, &mut player, &maze); // Procesar los eventos de entrada
        render(&maze, &mut window, &player);
    }
}
