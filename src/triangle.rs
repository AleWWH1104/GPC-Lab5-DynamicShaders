use crate::vertex::Vertex;
use crate::shaders::star::Star;
use crate::framebuffer::Framebuffer;
use crate::shaders::planet::Planet;
use nalgebra_glm::{Mat4, Vec3, Vec4};

// Uniforms struct to pass data to rendering functions
#[derive(Debug, Clone)]
pub struct Uniforms {
    pub time: f32,
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
}

impl Uniforms {
    pub fn new() -> Self {
        Uniforms {
            time: 0.0,
            model_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            projection_matrix: Mat4::identity(),
            viewport_matrix: Mat4::identity(),
        }
    }
}

// Rasterization function - renders a single triangle with the star shader
pub fn triangle_3d_with_star_shader(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: &Uniforms, framebuffer: &mut Framebuffer, star: &Star, render_aura: bool) -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mvp_matrix = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix;
    let clip_v1 = mvp_matrix * Vec4::new(v1.position.x, v1.position.y, v1.position.z, 1.0);
    let clip_v2 = mvp_matrix * Vec4::new(v2.position.x, v2.position.y, v2.position.z, 1.0);
    let clip_v3 = mvp_matrix * Vec4::new(v3.position.x, v3.position.y, v3.position.z, 1.0);

    if clip_v1.w == 0.0 || clip_v2.w == 0.0 || clip_v3.w == 0.0 {
        return Ok(());
    }
    let ndc_v1 = Vec3::new(clip_v1.x / clip_v1.w, clip_v1.y / clip_v1.w, clip_v1.z / clip_v1.w);
    let ndc_v2 = Vec3::new(clip_v2.x / clip_v2.w, clip_v2.y / clip_v2.w, clip_v2.z / clip_v2.w);
    let ndc_v3 = Vec3::new(clip_v3.x / clip_v3.w, clip_v3.y / clip_v3.w, clip_v3.z / clip_v3.w);

    let screen_v1 = uniforms.viewport_matrix * Vec4::new(ndc_v1.x, ndc_v1.y, ndc_v1.z, 1.0);
    let screen_v2 = uniforms.viewport_matrix * Vec4::new(ndc_v2.x, ndc_v2.y, ndc_v2.z, 1.0);
    let screen_v3 = uniforms.viewport_matrix * Vec4::new(ndc_v3.x, ndc_v3.y, ndc_v3.z, 1.0);

    let x1 = screen_v1.x as i32;
    let y1 = screen_v1.y as i32;
    let x2 = screen_v2.x as i32;
    let y2 = screen_v2.y as i32;
    let x3 = screen_v3.x as i32;
    let y3 = screen_v3.y as i32;

    let min_x = x1.min(x2).min(x3).max(0).min(WIDTH as i32 - 1) as usize;
    let max_x = (x1.max(x2).max(x3) + 1).max(0).min(WIDTH as i32) as usize;
    let min_y = y1.min(y2).min(y3).max(0).min(HEIGHT as i32 - 1) as usize;
    let max_y = (y1.max(y2).max(y3) + 1).max(0).min(HEIGHT as i32) as usize;

    if min_x >= max_x || min_y >= max_y {
        return Ok(());
    }

    let det = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    if det == 0 { return Ok(()); }

    for y in min_y..max_y {
        for x in min_x..max_x {
            let px = x as i32;
            let py = y as i32;

            let w1 = ((y2 - y3) * (px - x3) + (x3 - x2) * (py - y3)) as f32 / det as f32;
            let w2 = ((y3 - y1) * (px - x3) + (x1 - x3) * (py - y3)) as f32 / det as f32;
            let w3 = 1.0 - w1 - w2;

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let z = w1 * screen_v1.z + w2 * screen_v2.z + w3 * screen_v3.z;

                let buffer_index = y * WIDTH + x;
                if buffer_index >= framebuffer.buffer.len() {
                    continue;
                }

                if z < framebuffer.zbuffer[buffer_index] {
                    let world_pos = w1 * v1.position + w2 * v2.position + w3 * v3.position;
                    let normal = (w1 * v1.normal + w2 * v2.normal + w3 * v3.normal).normalize();

                    // Usar la función que incluye aura y rayos si el flag está activado
                    let (color_raylib, _) = if render_aura {
                        star.evaluate_at_with_effects(&world_pos, &normal, uniforms.time)
                    } else {
                        star.evaluate_at(&world_pos, &normal, uniforms.time)
                    };

                    let color_u32 = ((color_raylib.r as u32) << 16) | ((color_raylib.g as u32) << 8) | (color_raylib.b as u32);

                    framebuffer.buffer[buffer_index] = color_u32;
                    framebuffer.zbuffer[buffer_index] = z;
                }
            }
        }
    }
    Ok(())
}

pub fn triangle_3d_with_planet_shader(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: &Uniforms, framebuffer: &mut Framebuffer, planet: &Planet) -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    // ... (Transformaciones idénticas a triangle_3d_with_star_shader) ...
    let mvp_matrix = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix;
    let clip_v1 = mvp_matrix * Vec4::new(v1.position.x, v1.position.y, v1.position.z, 1.0);
    let clip_v2 = mvp_matrix * Vec4::new(v2.position.x, v2.position.y, v2.position.z, 1.0);
    let clip_v3 = mvp_matrix * Vec4::new(v3.position.x, v3.position.y, v3.position.z, 1.0);

    if clip_v1.w == 0.0 || clip_v2.w == 0.0 || clip_v3.w == 0.0 {
        return Ok(()); // Salir silenciosamente si hay problemas de división
    }
    let ndc_v1 = Vec3::new(clip_v1.x / clip_v1.w, clip_v1.y / clip_v1.w, clip_v1.z / clip_v1.w);
    let ndc_v2 = Vec3::new(clip_v2.x / clip_v2.w, clip_v2.y / clip_v2.w, clip_v2.z / clip_v2.w);
    let ndc_v3 = Vec3::new(clip_v3.x / clip_v3.w, clip_v3.y / clip_v3.w, clip_v3.z / clip_v3.w);

    let screen_v1 = uniforms.viewport_matrix * Vec4::new(ndc_v1.x, ndc_v1.y, ndc_v1.z, 1.0);
    let screen_v2 = uniforms.viewport_matrix * Vec4::new(ndc_v2.x, ndc_v2.y, ndc_v2.z, 1.0);
    let screen_v3 = uniforms.viewport_matrix * Vec4::new(ndc_v3.x, ndc_v3.y, ndc_v3.z, 1.0);

    let x1 = screen_v1.x as i32;
    let y1 = screen_v1.y as i32;
    let x2 = screen_v2.x as i32;
    let y2 = screen_v2.y as i32;
    let x3 = screen_v3.x as i32;
    let y3 = screen_v3.y as i32;

    let min_x = x1.min(x2).min(x3).max(0).min(WIDTH as i32 - 1) as usize;
    let max_x = (x1.max(x2).max(x3) + 1).max(0).min(WIDTH as i32) as usize;
    let min_y = y1.min(y2).min(y3).max(0).min(HEIGHT as i32 - 1) as usize;
    let max_y = (y1.max(y2).max(y3) + 1).max(0).min(HEIGHT as i32) as usize;

    if min_x >= max_x || min_y >= max_y {
        return Ok(()); // Triángulo fuera de pantalla
    }

    let det = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    if det == 0 { return Ok(()); } // Degenerate triangle

    for y in min_y..max_y {
        for x in min_x..max_x {
            let px = x as i32;
            let py = y as i32;

            let w1 = ((y2 - y3) * (px - x3) + (x3 - x2) * (py - y3)) as f32 / det as f32;
            let w2 = ((y3 - y1) * (px - x3) + (x1 - x3) * (py - y3)) as f32 / det as f32;
            let w3 = 1.0 - w1 - w2;

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let z = w1 * screen_v1.z + w2 * screen_v2.z + w3 * screen_v3.z;

                let buffer_index = y * WIDTH + x;
                if buffer_index >= framebuffer.buffer.len() {
                    continue;
                }

                if z < framebuffer.zbuffer[buffer_index] {
                    let world_pos = w1 * v1.position + w2 * v2.position + w3 * v3.position;
                    let normal = (w1 * v1.normal + w2 * v2.normal + w3 * v3.normal).normalize();

                    // Usar el shader del planeta
                    let color_raylib = planet.evaluate_at(&world_pos, &normal, uniforms.time);
                    let color_u32 = ((color_raylib.r as u32) << 16) | ((color_raylib.g as u32) << 8) | (color_raylib.b as u32);

                    framebuffer.buffer[buffer_index] = color_u32;
                    framebuffer.zbuffer[buffer_index] = z;
                }
            }
        }
    }
    Ok(())
}