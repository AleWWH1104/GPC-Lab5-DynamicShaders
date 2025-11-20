mod framebuffer;
mod vertex;
mod fragment;
mod camera;
mod shaders;
mod matrix; // Import the new matrix module
mod triangle; // Import the new triangle module

use crate::shaders::star::Star; // Import the Star struct
use crate::shaders::planet::{Planet, PlanetType};
use crate::vertex::Vertex; // Import Vertex
use crate::triangle::{triangle_3d_with_star_shader,triangle_3d_with_planet_shader, Uniforms}; // Import the rendering function and Uniforms
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

fn render_orbit(framebuffer: &mut Framebuffer, center: &Vec3, radius: f32, color: u32) {
    let steps = 100;
    let angle_step = 2.0 * PI / steps as f32;

    for i in 0..steps {
        let angle1 = i as f32 * angle_step;
        let angle2 = (i + 1) as f32 * angle_step;

        let x1 = center.x + radius * angle1.cos();
        let z1 = center.z + radius * angle1.sin();
        let x2 = center.x + radius * angle2.cos();
        let z2 = center.z + radius * angle2.sin();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    
    let mut window = Window::new(
        "Solar System - Software Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    window.set_target_fps(60);

    // Cámara 2D en el plano XZ
    let mut camera = Camera::new(
        Vec3::new(0.0, 10.0, -100.0), // Comienza más lejos en Z, ligeramente elevada
        Vec3::new(0.0, 0.0, 0.0),    // Mira hacia el origen (el Sol)
        Vec3::new(0.0, 1.0, 0.0),
    );

    let sphere_mesh = load_obj("models/sphere.obj")?;
    println!("Loaded {} vertices from sphere.obj", sphere_mesh.len());

    let mut sun = Star::new(1.5, Vec3::new(0.0, 0.0, 0.0));

    let mut planets = vec![
        Planet::new(PlanetType::Rocky, 0.4, 4.0, 0.8, 0.6, 0.0),
        Planet::new(PlanetType::Cloudy, 0.7, 6.5, 0.6, 0.3, PI / 2.0), // Planeta nube celeste
        Planet::new(PlanetType::Earth, 0.6, 9.0, 0.5, 0.7, PI),        // Planeta Tierra
        Planet::new(PlanetType::Glitter, 0.6, 11.5, 0.3, 0.5, 3.0 * PI / 2.0),
        Planet::new(PlanetType::Heart, 0.7, 14.0, 0.2, 0.3, PI / 4.0),
    ];

    let mut uniforms = Uniforms::new();
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

        // Controles de Cámara 2D (WASD para movimiento en XZ, QE para altura)
        let camera_speed = 0.1;
         let zoom_speed = 0.5;
        if window.is_key_down(Key::W) {
            camera.eye.z -= camera_speed; // Mover en -Z (hacia adelante)
            camera.center.z -= camera_speed;
        }
        if window.is_key_down(Key::S) {
            camera.eye.z += camera_speed; // Mover en +Z (hacia atrás)
            camera.center.z += camera_speed;
        }
        if window.is_key_down(Key::A) {
            camera.eye.x -= camera_speed; // Mover en -X (izquierda)
            camera.center.x -= camera_speed;
        }
        if window.is_key_down(Key::D) {
            camera.eye.x += camera_speed; // Mover en +X (derecha)
            camera.center.x += camera_speed;
        }
        // Opcional: Controles de altura
        if window.is_key_down(Key::Q) {
            camera.eye.y += camera_speed;
            camera.center.y += camera_speed;
        }
        if window.is_key_down(Key::E) {
            camera.eye.y -= camera_speed;
            camera.center.y -= camera_speed;
        }
        if window.is_key_down(Key::R) { // Zoom In (acercar)
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::F) { // Zoom Out (alejar)
            camera.zoom(-zoom_speed);
        }
    
        uniforms.view_matrix = camera.get_view_matrix();

        sun.update(0.016);

        for planet in &mut planets {
            planet.update(0.016, elapsed);
        }

        framebuffer.clear();

        // --- Render Skybox (si se implementa) ---
        // Renderizar el modelo skybox con una matriz de vista donde la translación es cero (siempre centrado en la cámara)

        // --- Render Orbit Lines (Opcional) ---
        for planet in &planets {
            let orbit_color = 0x404040; // Gris oscuro
            render_orbit(&mut framebuffer, &planet.orbit_center, planet.orbit_radius, orbit_color);
        }

        // --- Render Sun ---
        uniforms.model_matrix = create_model_matrix(
            sun.position,
            1.0, // Escala
            Vec3::new(sun.rotation, sun.rotation * 0.5, 0.0), // Rotación
        );
        for i in (0..sphere_mesh.len()).step_by(3) {
            if i + 2 < sphere_mesh.len() {
                let v1 = &sphere_mesh[i];
                let v2 = &sphere_mesh[i + 1];
                let v3 = &sphere_mesh[i + 2];
                triangle_3d_with_star_shader(v1, v2, v3, &uniforms, &mut framebuffer, &sun);
            }
        }

        // --- Render Planets ---
        for planet in &planets {
            uniforms.model_matrix = create_model_matrix(
                planet.position,
                1.0, // Escala
                Vec3::new(planet.rotation, planet.rotation * 0.7, 0.0), // Rotación
            );
            for i in (0..sphere_mesh.len()).step_by(3) {
                if i + 2 < sphere_mesh.len() {
                    let v1 = &sphere_mesh[i];
                    let v2 = &sphere_mesh[i + 1];
                    let v3 = &sphere_mesh[i + 2];
                    // Pasar el planeta en lugar del sol
                    triangle_3d_with_planet_shader(v1, v2, v3, &uniforms, &mut framebuffer, planet)?;
                }
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    Ok(())
}