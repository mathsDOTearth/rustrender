// testing our 3d vector functions with minifb
// by Rich of maths.earth 202500308
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

// Import our rendering module.
mod render;
use render::{buffer_to_u32, clear_buffer, draw_line, Pixel};

mod vector;
use vector::{project_point, Vector3};

// Public constants for the window dimensions.
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

fn main() {
    // Create a new window.
    let mut window = Window::new(
        "3D Cube Test: Wireframe Rendering with minifb",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    // Create a 2D pixel buffer initialised to black.
    let mut pixel_buffer: Vec<Vec<Pixel>> = vec![vec![Pixel::new(0, 0, 0, 255); WIDTH]; HEIGHT];

    // Define the cube vertices (a cube centred at the origin, in 3D).
    let cube_vertices: [Vector3; 8] = [
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new( 1.0, -1.0, -1.0),
        Vector3::new( 1.0,  1.0, -1.0),
        Vector3::new(-1.0,  1.0, -1.0),
        Vector3::new(-1.0, -1.0,  1.0),
        Vector3::new( 1.0, -1.0,  1.0),
        Vector3::new( 1.0,  1.0,  1.0),
        Vector3::new(-1.0,  1.0,  1.0),
    ];
    // Define the cube edges as pairs of indices.
    let cube_edges: &[(usize, usize)] = &[
        (0, 1), (1, 2), (2, 3), (3, 0), // back face
        (4, 5), (5, 6), (6, 7), (7, 4), // front face
        (0, 4), (1, 5), (2, 6), (3, 7), // connecting edges
    ];

    let camera_distance = 3.0;
    // Set scale so that a cube with side 2 becomes 200 pixels wide.
    let scale = 100.0;
    let mut angle: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the pixel buffer.
        clear_buffer(&mut pixel_buffer);

        // Rotate the cube about the Y axis.
        let rotated_vertices: Vec<Vector3> = cube_vertices.iter().map(|&v| {
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            Vector3::new(
                v.x * cos_a - v.z * sin_a,
                v.y,
                v.x * sin_a + v.z * cos_a,
            )
        }).collect();

        // Project each rotated vertex into 2D screen space.
        let projected: Vec<Option<(i32, i32)>> = rotated_vertices.iter()
            .map(|&v| project_point(v, camera_distance, WIDTH, HEIGHT, scale))
            .collect();

        // Draw each edge of the cube.
        for &(i0, i1) in cube_edges.iter() {
            if let (Some(p0), Some(p1)) = (projected[i0], projected[i1]) {
                draw_line(&mut pixel_buffer, p0.0, p0.1, p1.0, p1.1, Pixel::new(255, 255, 255, 255));
            }
        }

        // Convert the 2D pixel buffer to a 1D u32 buffer.
        let buffer = buffer_to_u32(&pixel_buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Failed to update window");

        // Increment the rotation angle.
        angle += 0.01;
    }
}