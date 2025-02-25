// Add an RGBA pixel buffer to minifb
// by Rich of maths.earth 20250223
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

// Define the window dimensions.
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// A struct to represent an RGBA pixel.
#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    /// Create a new Pixel with the given red, green, blue and alpha values.
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert this pixel into a 32-bit colour in 0xAARRGGBB format.
    fn to_u32(self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

fn main() {
    // Create a new window.
    let mut window = Window::new(
        "Lesson One: 2D Pixel Buffer Rendering with minifb (Using Pixel struct)",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    // Create a 2D pixel buffer initialised to black.
    // This buffer is a vector of rows, each row being a vector of Pixel structs.
    let mut pixel_buffer: Vec<Vec<Pixel>> = vec![vec![Pixel::new(0, 0, 0, 255); WIDTH]; HEIGHT];

    // Define the dimensions of the square.
    let square_width = 200;
    let square_height = 200;
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
        // Check if the Space key is currently pressed.
        let current_space = window.is_key_down(Key::Space);
        // If space was just pressed (transition from not pressed to pressed)
        if current_space && !prev_space_down {
            // Cycle the square colour.
            color_state = (color_state + 1) % 3;
            square_color = match color_state {
                0 => Pixel::new(255, 0, 0, 255),   // Red
                1 => Pixel::new(0, 255, 0, 255),   // Green
                2 => Pixel::new(0, 0, 255, 255),   // Blue
                _ => Pixel::new(255, 0, 0, 255),   // Fallback to red
            };
        }
        // Store the current state for debouncing.
        prev_space_down = current_space;

        // Clear the pixel buffer by filling it with black.
        for row in pixel_buffer.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = Pixel::new(0, 0, 0, 255);
            }
        }

        // Draw the square into the pixel buffer using the current square colour.
        for y in square_y..(square_y + square_height) {
            for x in square_x..(square_x + square_width) {
                pixel_buffer[y][x] = square_color;
            }
        }

        // Convert the 2D pixel buffer to a 1D u32 buffer.
        // Each pixel is converted into 0xAARRGGBB format.
        let mut buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);
        for row in &pixel_buffer {
            for &pixel in row {
                buffer.push(pixel.to_u32());
            }
        }

        // Update the window with the new pixel buffer.
        // minifb manages double buffering internally.
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}

