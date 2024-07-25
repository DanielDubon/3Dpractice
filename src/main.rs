// src/main.rs

mod maze;
mod framebuffer;
mod maze_generator;

use maze_generator::make_maze;
use maze::load_maze;
use framebuffer::Framebuffer;
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
    let rows = maze.lines().count();
    let cols = maze.lines().next().unwrap_or("").chars().count();

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

fn main() {
    let width = 800; // Adjust width and height as needed
    let height = 600;
    
    let maze_width = 10; // Adjust maze width and height as needed
    let maze_height = 10;
    
    let maze = make_maze(maze_width, maze_height); // Generate a new maze
    let mut framebuffer = Framebuffer::new(width, height);

    save_maze_to_file(&maze, "maze.txt");
    render(&mut framebuffer, &maze);
    
    framebuffer.display(); // Show the rendered image
}
