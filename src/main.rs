mod maze;
mod framebuffer;
mod maze_generator;
mod player;
mod cast_ray;

use maze_generator::make_maze;
use framebuffer::Framebuffer;
use player::Player;
use cast_ray::cast_ray;
use std::fs::File;
use std::io::Write;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' => 0x000000, // Black for walls
        '|' => 0x000000, // Black for vertical walls
        '-' => 0x000000, // Black for horizontal walls
        ' ' => 0xFFFFFF, // White for open spaces
        'p' => 0xFF0000, // Red for start
        'g' => 0x00FF00, // Green for goal
        _ => 0xFFFFFF,   // Default to white if unknown
    };
    
    framebuffer.set_current_color(color);
    framebuffer.draw_rect(xo, yo, block_size, block_size);
}

fn render(framebuffer: &mut Framebuffer, maze: &str) {
    let block_size = 20; // Smaller block size for better resolution

    for (row_idx, line) in maze.lines().enumerate() {
        for (col_idx, cell) in line.chars().enumerate() {
            draw_cell(framebuffer, col_idx * block_size, row_idx * block_size, block_size, cell);
        }
    }
}

fn save_maze_to_file(maze: &str, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(maze.as_bytes())?;
    Ok(())
}

fn print_maze(maze: &[Vec<char>]) {
    for row in maze {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn convert_maze_to_vec(maze_str: &str) -> Vec<Vec<char>> {
    maze_str.lines().map(|line| line.chars().collect()).collect()
}

fn find_player_position(maze: &[Vec<char>]) -> (f32, f32) {
    for (row_idx, row) in maze.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == 'p') {
            return (col_idx as f32, row_idx as f32);
        }
    }
    (0.0, 0.0) // Default to (0, 0) if not found
}

fn main() {
    let width = 800;
    let height = 600;

    let maze_width = 10;
    let maze_height = 10;

    let maze_str = make_maze(maze_width, maze_height); // Generate a new maze
    let maze = convert_maze_to_vec(&maze_str); // Convert to Vec<Vec<char>>

    print_maze(&maze); // Print maze to verify content

    let (player_x, player_y) = find_player_position(&maze);
    let mut framebuffer = Framebuffer::new(width, height);

    save_maze_to_file(&maze_str, "maze.txt");
    render(&mut framebuffer, &maze_str);
    
    let player = Player::new(player_x, player_y, std::f32::consts::PI / 4.0); // Set player at the position of 'p'
    
    cast_ray(&mut framebuffer, &maze, &player, 20); // Adjust block_size as needed
    
    framebuffer.display();
}
