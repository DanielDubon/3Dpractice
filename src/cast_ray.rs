use crate::player::Player;
use crate::framebuffer::Framebuffer;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, block_size: usize) {
    let cos_angle = player.angle().cos();
    let sin_angle = player.angle().sin();

    // Get player position in framebuffer coordinates
    let player_x = (player.x() * block_size as f32) as usize;
    let player_y = (player.y() * block_size as f32) as usize;

    println!("Player Position: ({}, {}), Angle: {}", player_x, player_y, player.angle());

    // Example ray casting logic
    for d in 0..100 { // Example distance range
        let x = (player_x as f32 + d as f32 * cos_angle) as usize;
        let y = (player_y as f32 + d as f32 * sin_angle) as usize;

        // Print ray coordinates for debugging
        println!("Ray Coordinates: ({}, {})", x, y);

        // Ensure coordinates are within framebuffer bounds
        if x < framebuffer.width() && y < framebuffer.height() {
            framebuffer.set_current_color(0xFF0000); // Red for rays
            framebuffer.draw_rect(x, y, 1, 1); // Draw a point for simplicity
        }
    }
}
