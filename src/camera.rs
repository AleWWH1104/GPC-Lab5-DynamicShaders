//camera.rs
use nalgebra_glm::{Vec3, Mat4, look_at};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }

    pub fn forward(&self) -> Vec3 {
        (self.center - self.eye).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(&self.up).normalize()
    }

    pub fn move_forward(&mut self, distance: f32) {
        let direction = self.forward();
        self.eye += direction * distance;
        self.center += direction * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        let direction = self.right();
        self.eye += direction * distance;
        self.center += direction * distance;
    }

    pub fn move_up(&mut self, distance: f32) {
        self.eye.y += distance;
        self.center.y += distance;
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius = (self.eye - self.center).magnitude();

        let current_direction = (self.eye - self.center).normalize();

        let right = current_direction.cross(&self.up).normalize();
        let rotated_yaw = nalgebra_glm::rotate_vec3(&current_direction, delta_yaw, &self.up);
        let rotated = nalgebra_glm::rotate_vec3(&rotated_yaw, delta_pitch, &right);

        self.eye = self.center + rotated * radius;
    }
}