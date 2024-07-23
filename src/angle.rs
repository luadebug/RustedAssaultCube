use std::f32::consts::PI;
use crate::distance::distance_3d;
use crate::vec_structures::Vec3;

/// A struct representing angles in a 3D space.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Angle {
    pub yaw: f32,
    pub pitch: f32,
}
impl Angle {
    pub fn new(yaw: f32, pitch: f32) -> Angle {
        Angle { yaw, pitch }
    }
    pub fn get_angle(player_pos:Vec3, enemy_pos:Vec3) -> Angle {

        let distance = distance_3d(player_pos, enemy_pos);
        // Calculate yaw
        let yaw = -enemy_pos.x.atan2(enemy_pos.y) / PI * 180.0 + 180.0;

        // Calculate pitch
        let pitch = (enemy_pos.z - enemy_pos.z) / distance;
        let pitch = pitch.asin() / PI * 180.0;
        Angle { yaw, pitch }
    }

}

