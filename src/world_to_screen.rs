use crate::vec_structures::{Vec2, Vec3};

pub unsafe fn world_to_screen(
    position: Vec3,
    screen: &mut Vec2,
    view_matrix: [f32; 16],
    window_width: i32,
    window_height: i32,
) -> bool {
    // Calculate the w component directly
    let clip_w = position.x * view_matrix[3] + position.y * view_matrix[7] + position.z * view_matrix[11] + view_matrix[15];

    // Early out if w is too small for perspective division
    if clip_w < 0.001 {
        return false;
    }

    // Calculate the x and y clip coordinates
    let clip_x = position.x * view_matrix[0] + position.y * view_matrix[4] + position.z * view_matrix[8] + view_matrix[12];
    let clip_y = position.x * view_matrix[1] + position.y * view_matrix[5] + position.z * view_matrix[9] + view_matrix[13];

    // Perform perspective division
    let normalized_device_coordinates_x = clip_x / clip_w;
    let normalized_device_coordinates_y = clip_y / clip_w;

    // Precompute half dimensions for screen coordinates
    let half_width = window_width as f32 * 0.5;
    let half_height = window_height as f32 * 0.5;

    // Calculate screen coordinates directly
    screen.x = (1.0 + normalized_device_coordinates_x) * half_width;
    screen.y = (1.0 - normalized_device_coordinates_y) * half_height;

    true
}