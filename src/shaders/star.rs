use crate::shaders::noise::fbm_noise;
use nalgebra_glm::Vec3;
use raylib::prelude::Color;

pub struct Star {
    pub radius: f32,
    pub position: Vec3,
    pub rotation: f32,
}

impl Star {
    pub fn new(radius: f32, position: Vec3) -> Self {
        Star {
            radius,
            position,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rotation += dt * 0.05;
    }

    // CORRECCIÓN AQUÍ: Cambiamos el retorno a (Color, f32)
    pub fn evaluate_at(&self, world_pos: &Vec3, _normal: &Vec3, time: f32) -> (Color, f32) {
        let local_pos = world_pos - self.position;
        let normalized_pos = local_pos.normalize();
        
        let theta = normalized_pos.y.asin();
        let phi = normalized_pos.x.atan2(normalized_pos.z);

        // Ruido de magma
        let noise = fbm_noise((phi * 8.0, theta * 8.0), time * 0.3, 5, 0.6);
        let n = (noise + 1.0) * 0.5;

        let color_dark = Vec3::new(0.8, 0.1, 0.0); 
        let color_base = Vec3::new(1.0, 0.5, 0.0);
        let color_hot = Vec3::new(1.0, 1.0, 0.6);

        let final_color_vec = if n < 0.5 {
            let t = n / 0.5;
            lerp(&color_dark, &color_base, t)
        } else {
            let t = (n - 0.5) / 0.5;
            lerp(&color_base, &color_hot, t)
        };

        let color = Color::new(
            (final_color_vec.x * 255.0) as u8,
            (final_color_vec.y * 255.0) as u8,
            (final_color_vec.z * 255.0) as u8,
            255
        );

        // Devolvemos la tupla (Color, 0.0) para satisfacer a triangle.rs
        (color, 0.0) 
    }

    pub fn evaluate_at_with_effects(&self, world_pos: &Vec3, normal: &Vec3, time: f32) -> (Color, f32) {
        // Esta función ya devuelve lo correcto porque llama a la corregida arriba
        self.evaluate_at(world_pos, normal, time)
    }
}

fn lerp(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    Vec3::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
    )
}