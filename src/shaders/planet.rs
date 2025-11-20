use crate::shaders::noise::{fbm_noise};
use nalgebra_glm::{Vec3, Vec4, Vec2};
use raylib::prelude::Color;

#[derive(Debug, Clone, Copy)]
pub enum PlanetType {
    Rocky,
    Cloudy, // Nuevo tipo
    Earth,  // Nuevo tipo
    Glitter,
    Heart,
}

pub struct Planet {
    pub radius: f32,
    pub initial_angle: f32,
    pub position: Vec3,
    pub orbit_center: Vec3,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub planet_type: PlanetType,
    pub base_color: Color,
    pub detail_color: Color,
}

impl Planet {
    pub fn new(planet_type: PlanetType, radius: f32, orbit_radius: f32, orbit_speed: f32, rotation_speed: f32, initial_angle: f32) -> Self {
        let (base_color, detail_color) = match planet_type {
            PlanetType::Rocky => (Color::BROWN, Color::DARKBROWN),
            PlanetType::Cloudy => (Color::LIGHTBLUE, Color::WHITE), // Azul cielo, nubes blancas
            PlanetType::Earth => (Color::DARKGREEN, Color::BLUE),   // Tierra, agua
            PlanetType::Glitter => (Color::PINK, Color::WHITE),
            PlanetType::Heart => (Color::PINK, Color::RED),
        };

        let x = orbit_radius * initial_angle.cos();
        let z = orbit_radius * initial_angle.sin();
        let position = Vec3::new(x, 0.0, z);

        Planet {
            radius,
            initial_angle,
            position,
            orbit_center: Vec3::new(0.0, 0.0, 0.0),
            orbit_radius,
            orbit_speed,
            rotation: 0.0,
            rotation_speed,
            planet_type,
            base_color,
            detail_color,
        }
    }

    pub fn update(&mut self, dt: f32, time: f32) {
        let current_angle = self.initial_angle + self.orbit_speed * time;
        self.position.x = self.orbit_center.x + self.orbit_radius * current_angle.cos();
        self.position.z = self.orbit_center.z + self.orbit_radius * current_angle.sin();
        self.rotation += self.rotation_speed * dt;
    }

    pub fn evaluate_at(&self, world_pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let rl_pos = raylib::prelude::Vector3::new(world_pos.x, world_pos.y, world_pos.z);
        let rl_normal = raylib::prelude::Vector3::new(normal.x, normal.y, normal.z);

        let length = (rl_normal.x * rl_normal.x + rl_normal.y * rl_normal.y + rl_normal.z * rl_normal.z).sqrt();
        let normalized_normal = if length > 0.0 {
            raylib::prelude::Vector3::new(rl_normal.x / length, rl_normal.y / length, rl_normal.z / length)
        } else {
            raylib::prelude::Vector3::new(0.0, 0.0, 1.0)
        };

        let light_dir = raylib::prelude::Vector3::new(1.0, 1.0, 1.0);
        let light_length = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
        let normalized_light_dir = if light_length > 0.0 {
            raylib::prelude::Vector3::new(light_dir.x / light_length, light_dir.y / light_length, light_dir.z / light_length)
        } else {
            raylib::prelude::Vector3::new(1.0, 0.0, 0.0)
        };

        let light_intensity = simulate_lighting(&normalized_normal, &normalized_light_dir);

        let base_color = match self.planet_type {
            PlanetType::Rocky => rocky_planet_color(&rl_pos, time),
            PlanetType::Cloudy => cloudy_planet_color(&rl_pos, time), // Nuevo shader
            PlanetType::Earth => earth_planet_color(&rl_pos, time),   // Nuevo shader
            PlanetType::Glitter => glitter_planet_color(&rl_pos, time),
            PlanetType::Heart => heart_planet_color(&rl_pos, time),
        };

        let final_color = raylib::prelude::Vector3::new(
            base_color.x * light_intensity,
            base_color.y * light_intensity,
            base_color.z * light_intensity
        );

        Color::new(
            (final_color.x * 255.0) as u8,
            (final_color.y * 255.0) as u8,
            (final_color.z * 255.0) as u8,
            255
        )
    }
}

// Funciones auxiliares (mantener las anteriores)
fn simulate_lighting(normal: &raylib::prelude::Vector3, light_dir: &raylib::prelude::Vector3) -> f32 {
    let intensity = normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z;
    intensity.max(0.0).min(1.0) * 0.8 + 0.2
}

fn rotate_planet_position(pos: &raylib::prelude::Vector3, time: f32, rotation_speed: f32) -> raylib::prelude::Vector3 {
    let angle = time * rotation_speed;
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    raylib::prelude::Vector3::new(
        pos.x * cos_a - pos.z * sin_a,
        pos.y,
        pos.x * sin_a + pos.z * cos_a
    )
}

fn noise(pos: &raylib::prelude::Vector3) -> f32 {
    let x = pos.x as i32;
    let y = pos.y as i32;
    let z = pos.z as i32;
    let n = (x.wrapping_add(y.wrapping_mul(57)).wrapping_add(z.wrapping_mul(113))) as f32;
    ((n * n * 41597.5453).sin() * 43758.5453) % 1.0
}

fn fractal_noise(pos: &raylib::prelude::Vector3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    for _ in 0..octaves {
        value += noise(&raylib::prelude::Vector3::new(pos.x * frequency, pos.y * frequency, pos.z * frequency)) * amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    value
}

// --- FUNCIONES DE COLOR ACTUALIZADAS ---
// rocky_planet_color (puedes mantener la anterior o simplificarla)
fn rocky_planet_color(pos: &raylib::prelude::Vector3, time: f32) -> raylib::prelude::Vector3 {
    let rotated_pos = rotate_planet_position(pos, time, 0.3);
    let base_noise = fractal_noise(&rotated_pos, 4);
    let detail_noise = fractal_noise(&raylib::prelude::Vector3::new(rotated_pos.x * 8.0, rotated_pos.y * 8.0, rotated_pos.z * 8.0), 2);

    let base_color = raylib::prelude::Vector3::new(0.8, 0.3, 0.1);  // Rojo intenso
    let rock_color = raylib::prelude::Vector3::new(0.6, 0.2, 0.05); // Marrón oscuro
    let ash_color = raylib::prelude::Vector3::new(0.3, 0.1, 0.05);  // Gris oscuro

    let elevation = (base_noise + detail_noise * 0.3) * 0.5 + 0.5;

    if elevation > 0.7 {
        raylib::prelude::Vector3::new(
            rock_color.x * elevation,
            rock_color.y * elevation,
            rock_color.z * elevation
        )
    } else if elevation < 0.4 {
        raylib::prelude::Vector3::new(
            ash_color.x * (elevation + 0.3),
            ash_color.y * (elevation + 0.3),
            ash_color.z * (elevation + 0.3)
        )
    } else {
        raylib::prelude::Vector3::new(
            base_color.x * elevation,
            base_color.y * elevation,
            base_color.z * elevation
        )
    }
}

// Nuevo shader: Cloudy (nube celeste)
fn cloudy_planet_color(pos: &raylib::prelude::Vector3, time: f32) -> raylib::prelude::Vector3 {
    let rotated_pos = rotate_planet_position(pos, time, 0.2); // Rotación lenta

    // Base azul cielo
    let sky_color = raylib::prelude::Vector3::new(0.5, 0.8, 0.9);
    // Color de las nubes
    let cloud_color = raylib::prelude::Vector3::new(1.0, 1.0, 1.0);

    // Usar ruido para simular nubes
    let cloud_noise = fractal_noise(&raylib::prelude::Vector3::new(
        rotated_pos.x * 5.0 + time * 0.1, // Movimiento lento de las nubes
        rotated_pos.y * 5.0,
        rotated_pos.z * 5.0
    ), 3);

    // Umbral para definir donde hay nubes
    let cloud_threshold = 0.6;

    if cloud_noise > cloud_threshold {
        cloud_color
    } else {
        // Mezcla suave entre el cielo y las nubes en los bordes
        let blend_factor = (cloud_noise - 0.4) / (cloud_threshold - 0.4); // Normalizar entre 0.4 y 0.6
        if blend_factor > 0.0 && blend_factor < 1.0 {
             raylib::prelude::Vector3::new(
                sky_color.x * (1.0 - blend_factor) + cloud_color.x * blend_factor,
                sky_color.y * (1.0 - blend_factor) + cloud_color.y * blend_factor,
                sky_color.z * (1.0 - blend_factor) + cloud_color.z * blend_factor
            )
        } else {
            sky_color
        }
    }
}

// Nuevo shader: Earth (Tierra)
fn earth_planet_color(pos: &raylib::prelude::Vector3, time: f32) -> raylib::prelude::Vector3 {
    let rotated_pos = rotate_planet_position(pos, time, 0.4); // Rotación más rápida como la Tierra

    // Coordenadas esféricas para definir continentes y océanos
    let lat = rotated_pos.y.asin();
    let lon = rotated_pos.x.atan2(rotated_pos.z);

    // Ruido para simular continentes
    let land_noise = fractal_noise(&raylib::prelude::Vector3::new(
        lon * 4.0 + time * 0.05, // Movimiento lento de los continentes
        lat * 3.0,
        0.0 // No usar Z para la forma de los continentes
    ), 4);

    // Colores base
    let ocean_color = raylib::prelude::Vector3::new(0.1, 0.3, 0.6); // Azul oscuro
    let land_color = raylib::prelude::Vector3::new(0.2, 0.6, 0.2);  // Verde oscuro
    let ice_color = raylib::prelude::Vector3::new(0.9, 0.9, 0.9);   // Blanco (polos)

    // Definir regiones polares
    let polar_threshold = 0.8;

    if lat.abs() > polar_threshold {
        // Polos
        ice_color
    } else if land_noise > 0.3 { // Ajustar el umbral para la cantidad de tierra
        // Tierra
        land_color
    } else {
        // Océano
        ocean_color
    }
}

// glitter_planet_color (mantener la anterior)
fn glitter_planet_color(pos: &raylib::prelude::Vector3, time: f32) -> raylib::prelude::Vector3 {
    let rotated_pos = rotate_planet_position(pos, time, 0.35);
    let pattern1 = (rotated_pos.x * 4.0 + time * 0.3).sin();
    let pattern2 = (rotated_pos.y * 4.0 + time * 0.2).cos();

    let base_pink = raylib::prelude::Vector3::new(1.0, 0.8, 0.9);
    let lavender = raylib::prelude::Vector3::new(0.8, 0.8, 1.0);
    let mint = raylib::prelude::Vector3::new(0.8, 1.0, 0.9);
    let peach = raylib::prelude::Vector3::new(1.0, 0.9, 0.8);

    let mix1 = (pattern1 * 0.5 + 0.5).powf(2.0);
    let mix2 = (pattern2 * 0.5 + 0.5).powf(2.0);

    let mut color = if mix1 > 0.6 {
        raylib::prelude::Vector3::new(
            base_pink.x * mix1 + lavender.x * (1.0 - mix1),
            base_pink.y * mix1 + lavender.y * (1.0 - mix1),
            base_pink.z * mix1 + lavender.z * (1.0 - mix1)
        )
    } else {
        raylib::prelude::Vector3::new(
            mint.x * mix2 + peach.x * (1.0 - mix2),
            mint.y * mix2 + peach.y * (1.0 - mix2),
            mint.z * mix2 + peach.z * (1.0 - mix2)
        )
    };

    let glitter = fractal_noise(&raylib::prelude::Vector3::new(
        rotated_pos.x * 40.0 + time * 4.0,
        rotated_pos.y * 40.0,
        rotated_pos.z * 40.0
    ), 1);

    if glitter > 0.95 {
        color = raylib::prelude::Vector3::new(
            color.x + 0.4,
            color.y + 0.4,
            color.z + 0.4
        );
    }

    color
}

// heart_planet_color (mantener la anterior)
fn heart_planet_color(pos: &raylib::prelude::Vector3, time: f32) -> raylib::prelude::Vector3 {
    let rotated_pos = rotate_planet_position(pos, time, 0.45);
    let x = rotated_pos.x;
    let y = rotated_pos.y;
    let z = rotated_pos.z;

    let heart_shape = (x*x + 9.0/4.0 * y*y + z*z - 1.0).powf(3.0) - 
                      (x*x * z.powf(3.0)) - (9.0/80.0 * y*y * z.powf(3.0));

    let main_color = raylib::prelude::Vector3::new(1.0, 0.6, 0.8);
    let accent_color = raylib::prelude::Vector3::new(0.9, 0.5, 0.9);
    let background_color = raylib::prelude::Vector3::new(1.0, 0.9, 0.95);

    let pattern1 = (x * 5.0 + time * 0.5).sin();
    let pattern2 = (y * 5.0 + time * 0.3).cos();
    let pattern = (pattern1 + pattern2) / 2.0;

    let shine = fractal_noise(&raylib::prelude::Vector3::new(
        rotated_pos.x * 30.0 + time * 3.0,
        rotated_pos.y * 30.0,
        rotated_pos.z * 30.0
    ), 1);

    let base_color = if heart_shape < 0.0 {
        if pattern > 0.5 {
            accent_color
        } else {
            main_color
        }
    } else {
        background_color
    };

    raylib::prelude::Vector3::new(
        base_color.x + shine * 0.3,
        base_color.y + shine * 0.3,
        base_color.z + shine * 0.3
    )
}
// --- FIN DE LAS FUNCIONES DE COLOR ---