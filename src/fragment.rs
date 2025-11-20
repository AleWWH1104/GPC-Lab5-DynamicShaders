// fragment.rs
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Fragment {
    pub position: (usize, usize),
    pub depth: f32,
    pub color: Color,
    pub normal: Vector3,
    pub intensity: f32,
}

impl Fragment {
    pub fn new(x: usize, y: usize, depth: f32) -> Self {
        Fragment {
            position: (x, y),
            depth,
            color: Color::BLACK,
            normal: Vector3::new(0.0, 0.0, 0.0),
            intensity: 1.0,
        }
    }
}