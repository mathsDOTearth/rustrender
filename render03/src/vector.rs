// This module contains 3d vector helper functions that extend minifb.
// by Rich of maths.earth 202500308

/// A simple 3D vector struct.
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Creates a new `Vector3` with the given components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the sum of two vectors.
    pub fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Returns the difference between two vectors.
    pub fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Returns this vector multiplied by a scalar.
    pub fn scale(self, factor: f32) -> Vector3 {
        Vector3::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// Returns the dot product of this vector and another.
    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of this vector and another.
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}


/// Projects a 3D point into 2D screen space using a simple perspective projection.
/// 
/// The idea here is to first perform a perspective division, then scale the resulting
/// coordinates so that an object with side length 2 (i.e. spanning from –1 to +1) becomes
/// 200 pixels wide. Finally, the coordinates are translated so that (0,0) corresponds
/// to the centre of the window.
/// 
/// # Arguments
/// * `p` - The 3D point in world space.
/// * `d` - The camera distance (a positive value).
/// * `width` and `height` - The dimensions of the window.
/// * `scale` - A scale factor to convert from the projected unit coordinates to pixels.
/// 
pub fn project_point(p: Vector3, d: f32, width: usize, height: usize, scale: f32) -> Option<(i32, i32)> {
    // Shift the z coordinate by the camera distance.
    let z = p.z + d;
    if z <= 0.0 {
        return None;
    }
    // Perspective division.
    let proj_x = p.x * d / z;
    let proj_y = p.y * d / z;
    // Map the projected coordinates to screen space.
    // Here we assume the projected coordinates are in a roughly [-1, 1] range.
    let screen_x = (proj_x * scale + width as f32 / 2.0).round() as i32;
    let screen_y = (height as f32 / 2.0 - proj_y * scale).round() as i32;
    Some((screen_x, screen_y))
}
