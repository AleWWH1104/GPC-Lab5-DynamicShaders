use crate::shaders::noise::{fbm_noise, noise2d};
use nalgebra_glm::Vec3;
use raylib::prelude::Color;

pub struct Star {
    pub radius: f32,
    pub position: Vec3,
    pub rotation: f32,
    pub base_color: Color,
    pub glow_color: Color,
}

impl Star {
    pub fn new(radius: f32, position: Vec3) -> Self {
        Star {
            radius,
            position,
            rotation: 0.0,
            base_color: Color::ORANGE,
            glow_color: Color::YELLOW,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rotation += dt * 0.1; // Rotación lenta
    }

    // Evaluate the star's surface properties at a given world position
    // This simulates the shader logic for calculating color and displacement
    pub fn evaluate_at(&self, world_pos: &Vec3, _normal: &Vec3, time: f32) -> (Color, f32) {
        // Convert world position to a point on the sphere's surface relative to its center
        let local_pos = world_pos - self.position;
        let normalized_pos = local_pos.normalize();
        
        // Use the spherical coordinates for noise sampling
        let theta = normalized_pos.y.asin(); // Latitude (-PI/2 to PI/2)
        let phi = normalized_pos.x.atan2(normalized_pos.z); // Longitude (-PI to PI)

        // --- Noise-based Displacement & Color ---
        // Sample noise on the sphere's surface for turbulence
        let noise_scale = 5.0; // Controls the size of the features
        let turbulence_noise = fbm_noise((phi * noise_scale, theta * noise_scale), time, 4, 0.5);

        // Displace the radius based on noise (for animation effect)
        let displaced_radius = self.radius * (1.0 + turbulence_noise * 0.1); // 10% displacement

        // Calculate distance from the displaced surface
        let distance_to_center = local_pos.magnitude();
        let surface_distance = displaced_radius;

        // --- Color Calculation with Dynamic Turbulence and Pulsation ---

        // 1. Core Glow Effect (intense in the center)
        let core_factor = 1.0 - (distance_to_center / (displaced_radius * 1.2)).min(1.0);
        let core_glow = core_factor * core_factor; // Quadratic falloff

        // 2. Surface Turbulence Effect based on noise
        // Map turbulence_noise from [-1,1] to [0,1] for intensity modulation
        let turbulence_intensity = (turbulence_noise + 1.0) * 0.5; // [0,1]

        // 3. Global Pulsation Effect (cyclic brightness change)
        let pulsation = (time * 2.0).sin() * 0.5 + 0.5; // Oscillates between 0.0 and 1.0

        // 4. Combine effects for final color
        // Base color is orange, but modulated by turbulence and pulsation
        let base_r = self.base_color.r as f32;
        let base_g = self.base_color.g as f32;
        let base_b = self.base_color.b as f32;

        // Create a dynamic color: hotter areas are brighter and whiter
        let hot_factor = turbulence_intensity * 0.8 + core_glow * 0.2; // Hotter where turbulence is high or near core
        let cool_factor = 1.0 - hot_factor;

        // Blend between base color and glow color based on heat
        let r = (base_r * cool_factor + self.glow_color.r as f32 * hot_factor).max(0.0).min(255.0);
        let g = (base_g * cool_factor + self.glow_color.g as f32 * hot_factor).max(0.0).min(255.0);
        let b = (base_b * cool_factor + self.glow_color.b as f32 * hot_factor).max(0.0).min(255.0);

        // Apply global pulsation to overall brightness
        let final_r = (r * pulsation).max(0.0).min(255.0);
        let final_g = (g * pulsation).max(0.0).min(255.0);
        let final_b = (b * pulsation).max(0.0).min(255.0);

        // Create the final color
        let final_color = Color::new(final_r as u8, final_g as u8, final_b as u8, 255);

        // Return the color and the displaced radius (for depth)
        (final_color, surface_distance)
    }
}