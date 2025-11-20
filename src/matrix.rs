use nalgebra_glm::{Mat4, Vec3};

// Function to create the projection matrix
pub fn create_projection_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov / 2.0).tan();
    let nf = 1.0 / (near - far);

    Mat4::new(
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (far + near) * nf, -1.0,
        0.0, 0.0, (2.0 * far * near) * nf, 0.0,
    )
}

// Function to create the viewport matrix
pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, 0.0,
        0.0, height / 2.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        width / 2.0, height / 2.0, 0.0, 1.0,
    )
}

// Function to create a model matrix (translation, scale, rotation)
pub fn create_model_matrix(position: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let trans = Mat4::new_translation(&position);
    let scale_mat = Mat4::new_nonuniform_scaling(&Vec3::new(scale, scale, scale));

    // Correct way to apply rotations using nalgebra_glm
    let rot_x = nalgebra_glm::rotate_x(&Mat4::identity(), rotation.x);
    let rot_y = nalgebra_glm::rotate_y(&rot_x, rotation.y);
    let rot_z = nalgebra_glm::rotate_z(&rot_y, rotation.z);

    trans * rot_z * scale_mat // Order: Scale -> Rotate -> Translate
}