use std::arch::x86::{_mm_cvtss_f32, _mm_hadd_ps, _mm_mul_ps, _mm_set_ps, _mm_set_ss, _mm_sqrt_ss, _mm_sub_ps};

use crate::vec_structures::{Vec2, Vec3};

pub fn distance_2d(pos_a: Vec2, pos_b: Vec2) -> f32 {
    unsafe {
        // Load the positions into SSE registers
        let a = _mm_set_ps(0.0, pos_a.y, pos_a.x, 0.0); // Load pos_a
        let b = _mm_set_ps(0.0, pos_b.y, pos_b.x, 0.0); // Load pos_b

        // Calculate the differences
        let diff = _mm_sub_ps(a, b); // diff = pos_a - pos_b

        // Calculate the square of the differences
        let squared = _mm_mul_ps(diff, diff); // squared = diff * diff

        // Sum the squared differences
        let temp = _mm_hadd_ps(squared, squared); // Horizontal add: [x1, y1, x2, y2] -> [x1+y1, x2+y2, 0, 0]
        let sum = _mm_hadd_ps(temp, temp); // Final horizontal add: [x, y, 0, 0] -> [x+y, 0, 0, 0]

        // Extract the result from the SIMD register
        let sum_f32: [f32; 4] = std::mem::transmute(sum); // Convert __m128 to [f32; 4]
        let distance_squared = sum_f32[0]; // Get the sum of squares

        // Fast square root
        fast_sqrt(distance_squared)
    }
}

pub fn distance_3d(pos_a: Vec3, pos_b: Vec3) -> f32 {
    unsafe {
        // Load the positions into SSE registers
        let a = _mm_set_ps(0.0, pos_a.z, pos_a.y, pos_a.x); // Load pos_a
        let b = _mm_set_ps(0.0, pos_b.z, pos_b.y, pos_b.x); // Load pos_b

        // Calculate the differences
        let diff = _mm_sub_ps(a, b); // diff = pos_a - pos_b

        // Calculate the square of the differences
        let squared = _mm_mul_ps(diff, diff); // squared = diff * diff

        // Sum the squared differences
        let temp = _mm_hadd_ps(squared, squared); // Horizontal add: [x1, y1, x2, y2] -> [x1+y1, x2+y2, 0, 0]
        let sum = _mm_hadd_ps(temp, temp); // Final horizontal add: [x, y, 0, 0] -> [x+y, 0, 0, 0]

        // Extract the result from the SIMD register
        let sum_f32: [f32; 4] = std::mem::transmute(sum); // Convert __m128 to [f32; 4]
        let distance_squared = sum_f32[0]; // Get the sum of squares

        // Fast square root
        fast_sqrt(distance_squared)
    }
}

// Fast square root using the x86 intrinsics
fn fast_sqrt(x: f32) -> f32 {
    unsafe {
        let x = _mm_set_ss(x); // Load the value into an SSE register
        let sqrt_x = _mm_sqrt_ss(x); // Compute the square root
        _mm_cvtss_f32(sqrt_x) // Convert back to f32
    }
}

/*
fn calculate_3d_distance(pos_a: Vec3, pos_b: Vec3) -> f32 {
    (((pos_a.x - pos_b.x) * (pos_a.x - pos_b.x))
        + ((pos_a.y - pos_b.y) * (pos_a.y - pos_b.y))
        + ((pos_a.z - pos_b.z) * (pos_a.z - pos_b.z)))
        .sqrt()
}
*/