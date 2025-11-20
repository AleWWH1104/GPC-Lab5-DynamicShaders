//vertex.rs
use raylib::prelude::*;
use nalgebra_glm::{Vec2, Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Vertex2D {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl Vertex2D {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Vertex2D { x, y, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
    pub transformed_position: Vec4,
    pub transformed_normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal,
            tex_coords: Vec2::new(0.0, 0.0),
            color,
            transformed_position: Vec4::new(0.0, 0.0, 0.0, 1.0),
            transformed_normal: normal,
        }
    }
}