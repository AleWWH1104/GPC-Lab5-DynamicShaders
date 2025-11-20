// skybox.rs
use crate::framebuffer::Framebuffer;
use crate::triangle::Uniforms;
use nalgebra_glm::{Vec3, Mat4, Vec4};
use rand::Rng;

pub struct Star {
    pub direction: Vec3,
    pub color: u32, // Cambiado a u32 para compatibilidad directa
    pub size: usize, // Cambiado a usize para usarlo en loops
}

pub struct Skybox {
    stars: Vec<Star>,
}

impl Skybox {
    pub fn new(star_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::new();

        for _ in 0..star_count {
            // Generar coordenadas esféricas aleatorias
            let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);

            // Convertir a cartesiano (dirección unitaria)
            let direction = Vec3::new(
                phi.sin() * theta.cos(),
                phi.sin() * theta.sin(),
                phi.cos(),
            ).normalize();

            // Generar color (blanco con variaciones de brillo)
            let brightness = rng.gen_range(100..255);
            let color = ((brightness as u32) << 16) | ((brightness as u32) << 8) | (brightness as u32);
            
            let size = if rng.gen_bool(0.1) { 2 } else { 1 };

            stars.push(Star {
                direction,
                color,
                size,
            });
        }

        Skybox { stars }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms, camera_pos: Vec3) {
        // Truco de Skybox: Crear una matriz de vista que NO tenga translación.
        // Esto hace que las estrellas parezcan estar en el infinito (no se acercan si caminas).
        let mut view_no_translation = uniforms.view_matrix;
        
        // Eliminamos la translación (columna 3, filas 0,1,2)
        view_no_translation.m14 = 0.0;
        view_no_translation.m24 = 0.0;
        view_no_translation.m34 = 0.0;

        for star in &self.stars {
            // Usamos dirección como vector (w=0) o posición relativa (w=1) con matriz sin traslación
            let view_dir = view_no_translation * Vec4::new(star.direction.x, star.direction.y, star.direction.z, 1.0);
            let clip_pos = uniforms.projection_matrix * view_dir;

            if clip_pos.w <= 0.0 { continue; }

            let ndc = Vec3::new(clip_pos.x / clip_pos.w, clip_pos.y / clip_pos.w, clip_pos.z / clip_pos.w);

            // Verificar si está dentro de la pantalla (clipping simple)
            if ndc.x >= -1.0 && ndc.x <= 1.0 && ndc.y >= -1.0 && ndc.y <= 1.0 {
                // Mapear a coordenadas de pantalla
                let screen_pos = uniforms.viewport_matrix * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);
                let cx = screen_pos.x as i32;
                let cy = screen_pos.y as i32;

                // Dibujar el punto (o cuadrado pequeño si size > 1)
                for dy in 0..star.size {
                    for dx in 0..star.size {
                        let px = cx + dx as i32;
                        let py = cy + dy as i32;

                        if px >= 0 && px < framebuffer.width as i32 && py >= 0 && py < framebuffer.height as i32 {
                            // Usamos una profundidad muy alta (ej. 0.999) para que siempre esté al fondo
                            framebuffer.point_with_depth(px as usize, py as usize, 0.999, star.color);
                        }
                    }
                }
            }
        }
    }
}