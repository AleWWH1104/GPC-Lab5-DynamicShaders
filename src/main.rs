mod framebuffer;
mod vertex;
mod fragment;
mod camera;
mod shaders;
mod matrix; // Import the new matrix module
mod triangle; // Import the new triangle module

use crate::shaders::star::Star; // Import the Star struct
use crate::vertex::Vertex; // Import Vertex
use crate::triangle::{triangle_3d_with_star_shader, Uniforms}; // Import the rendering function and Uniforms
use crate::matrix::{create_projection_matrix, create_viewport_matrix, create_model_matrix}; // Import matrix functions

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;
use std::f32::consts::PI;

use framebuffer::Framebuffer;
use camera::Camera;
use raylib::prelude::Color; // We still need this for the Star's color types


const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Simple struct to hold OBJ vertex data before processing
#[derive(Debug, Clone)]
struct ObjVertex {
    pos: Vec3,
    norm: Vec3,
}

// Function to load the sphere.obj file
fn load_obj(filename: &str) -> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(filename)?;
    let mut vertices = Vec::new();
    let mut positions = Vec::new();
    let mut normals = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "v" => {
                let x: f32 = parts[1].parse()?;
                let y: f32 = parts[2].parse()?;
                let z: f32 = parts[3].parse()?;
                positions.push(Vec3::new(x, y, z));
            }
            "vn" => {
                let x: f32 = parts[1].parse()?;
                let y: f32 = parts[2].parse()?;
                let z: f32 = parts[3].parse()?;
                normals.push(Vec3::new(x, y, z));
            }
            "f" => {
                // Assume faces are triangles and have vertex/normal indices like f v//vn v//vn v//vn
                for i in 1..=3 {
                    let face_part = parts[i];
                    let sub_parts: Vec<&str> = face_part.split('/').collect();
                    let pos_idx: usize = sub_parts[0].parse()?;
                    let norm_idx: usize = sub_parts[2].parse()?;

                    // OBJ format uses 1-based indexing
                    let pos = positions[pos_idx - 1];
                    let norm = normals[norm_idx - 1];

                    // Create a vertex with a default color, will be shaded later
                    vertices.push(Vertex::new(pos, norm, Color::WHITE));
                }
            }
            _ => {} // Ignore other lines (vt, etc.)
        }
    }

    Ok(vertices)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    
    let mut window = Window::new(
        "Star Dynamic Shaders - Iris Ayala",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    window.set_target_fps(60);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, -15.0), // Move camera closer to see the sphere
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Load the sphere model
    let sphere_mesh = load_obj("models/sphere.obj")?;
    println!("Loaded {} vertices from sphere.obj", sphere_mesh.len());

    let mut star = Star::new(1.5, Vec3::new(0.0, 0.0, 0.0));

    let mut uniforms = Uniforms::new(); // Use Uniforms from triangle.rs
    uniforms.projection_matrix = create_projection_matrix(
        45.0 * PI / 180.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0,
    );
    uniforms.view_matrix = camera.get_view_matrix();
    uniforms.viewport_matrix = create_viewport_matrix(WIDTH as f32, HEIGHT as f32);

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start_time.elapsed().as_secs_f32();
        uniforms.time = elapsed;

        // Camera Controls (WASD to move, QE to move up/down)
        let camera_speed = 0.1;
        if window.is_key_down(Key::W) {
            camera.move_forward(camera_speed);
        }
        if window.is_key_down(Key::S) {
            camera.move_forward(-camera_speed);
        }
        if window.is_key_down(Key::A) {
            camera.move_right(-camera_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(camera_speed);
        }
        if window.is_key_down(Key::Q) {
            camera.move_up(camera_speed);
        }
        if window.is_key_down(Key::E) {
            camera.move_up(-camera_speed);
        }

        uniforms.view_matrix = camera.get_view_matrix();

        star.update(0.016); // Update star rotation and animation state

        framebuffer.clear();

        // --- Render Star ---
        uniforms.model_matrix = create_model_matrix(
            star.position,
            0.5, // Scale
            Vec3::new(star.rotation, star.rotation * 0.5, 0.0), // Rotation
        );

        // Iterate through the loaded mesh triangles (every 3 vertices)
        for i in (0..sphere_mesh.len()).step_by(3) {
            if i + 2 < sphere_mesh.len() {
                let v1 = &sphere_mesh[i];
                let v2 = &sphere_mesh[i + 1];
                let v3 = &sphere_mesh[i + 2];

                triangle_3d_with_star_shader(v1, v2, v3, &uniforms, &mut framebuffer, &star);
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    Ok(())
}