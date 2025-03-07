extern crate minifb;

use minifb::{Key, Window, WindowOptions};

// Import our rendering module.
mod render;
use render::{buffer_to_u32, clear_buffer, draw_square, draw_pixel, draw_line, draw_triangle, draw_rect, Pixel};

// Public constants for the window dimensions.
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

fn main() {
    // Create a new window.
    let mut window = Window::new(
        "Lesson One: 2D Pixel Buffer Rendering with minifb (Library Example)",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    // Create a 2D pixel buffer initialised to black.
    // Each pixel is stored as a Pixel struct.
    let mut pixel_buffer: Vec<Vec<Pixel>> = vec![vec![Pixel::new(0, 0, 0, 255); WIDTH]; HEIGHT];

    // Define the dimensions of the square.
    let square_width = 100;
    let square_height = 100;
    // Calculate the top-left corner so that the square is centred.
    let square_x = (WIDTH - square_width) / 2;
    let square_y = (HEIGHT - square_height) / 2;

    // Set the initial square colour to red.
    let mut square_color = Pixel::new(255, 0, 0, 255);
    // Colour state: 0 = red, 1 = green, 2 = blue.
    let mut color_state: u32 = 0;

    // Variable to detect key transitions for the space bar.
    let mut prev_space_down = false;

    // Main loop.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Check the current state of the space bar.
        let current_space = window.is_key_down(Key::Space);
        // If space has just been pressed (transition from not pressed to pressed)...
        if current_space && !prev_space_down {
            // Cycle the square colour.
            color_state = (color_state + 1) % 3;
            square_color = match color_state {
                0 => Pixel::new(255, 0, 0, 255),   // Red
                1 => Pixel::new(0, 255, 0, 255),   // Green
                2 => Pixel::new(0, 0, 255, 255),   // Blue
                _ => Pixel::new(255, 0, 0, 255),   // Fallback to red (should not occur)
            };
        }
        // Save the current space state for the next iteration.
        prev_space_down = current_space;

        // Clear the pixel buffer (fill with black).
        clear_buffer(&mut pixel_buffer);

        // Draw the square into the pixel buffer using the current square colour.
        draw_square(&mut pixel_buffer, square_x, square_y, square_width, square_height, square_color);

        // Draw a single pixel into the pixel buffer.
        draw_pixel(&mut pixel_buffer, 10, 10, Pixel::new(255, 255, 255, 255));

        // Draw a line into the pixel buffer.
        draw_line(&mut pixel_buffer, 100, 100, 200, 200, Pixel::new(255, 255, 255, 255));
        draw_line(&mut pixel_buffer, 180, 100, 280, 280, Pixel::new(255, 0, 0, 255));

        // Draw a triangle into the pixel buffer.
        draw_triangle(&mut pixel_buffer, 400, 400, 500, 400, 450, 500, Pixel::new(0, 255, 0, 255));

        // Draw a rectangle into the pixel buffer.
        draw_rect(&mut pixel_buffer, 600, 400, 100, 100, Pixel::new(0, 0, 255, 255));
        
        // Convert the 2D pixel buffer to a 1D u32 buffer.
        let buffer = buffer_to_u32(&pixel_buffer);

        // Update the window with the new 1D pixel buffer.
        // minifb handles double buffering internally.
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}
