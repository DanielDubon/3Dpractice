use minifb::Key;
use crate::Player;

pub const MOVE_SPEED: f32 = 2.0; // Reducido para movimiento más lento y preciso
pub const ROTATION_SPEED: f32 = std::f32::consts::PI / 60.0; // Más lento para rotación precisa

pub fn process_events(window: &minifb::Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    let cell_size = 20.0; // Tamaño de cada celda en píxeles

    // Verifica si hay colisión en la próxima posición
    let check_collision = |x: f32, y: f32| -> bool {
        let cell_x = (x / cell_size).floor() as usize;
        let cell_y = (y / cell_size).floor() as usize;

        if cell_x >= maze[0].len() || cell_y >= maze.len() {
            return true; // Fuera de los límites del laberinto
        }

        let cell = maze[cell_y][cell_x];
        cell == '+' || cell == '|' // Si es una pared
    };

    if window.is_key_down(Key::W) {
        let new_x = player.x + player.angle.cos() * MOVE_SPEED;
        let new_y = player.y + player.angle.sin() * MOVE_SPEED;

        if !check_collision(new_x, new_y) {
            player.x = new_x;
            player.y = new_y;
        }
    }
    if window.is_key_down(Key::S) {
        let new_x = player.x - player.angle.cos() * MOVE_SPEED;
        let new_y = player.y - player.angle.sin() * MOVE_SPEED;

        if !check_collision(new_x, new_y) {
            player.x = new_x;
            player.y = new_y;
        }
    }
    if window.is_key_down(Key::A) {
        player.angle -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.angle += ROTATION_SPEED;
    }
}
