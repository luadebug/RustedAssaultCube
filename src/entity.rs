use crate::offsets::{
    HEAD_X_FROM_LOCAL_PLAYER, HEAD_Y_FROM_LOCAL_PLAYER, HEAD_Z_FROM_LOCAL_PLAYER,
    HEALTH_OFFSET_FROM_LOCAL_PLAYER, NAME_OFFSET_FROM_LOCAL_PLAYER, PITCH_OFFSET,
    POSITION_X_FROM_LOCAL_PLAYER, POSITION_Y_FROM_LOCAL_PLAYER, POSITION_Z_FROM_LOCAL_PLAYER,
    TEAM_OFFSET_FROM_LOCAL_PLAYER, YAW_OFFSET,
};
use crate::utils::{read_memory, read_memory_into_slice, write_memory};
use crate::vec_structures::Vec3;

pub struct Entity {
    pub entity_starts_at_addr: usize,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            entity_starts_at_addr: 0,
        }
    }
    pub fn from_addr(addr: usize) -> Entity {
        Entity {
            entity_starts_at_addr: addr,
        }
    }

    pub fn yaw(&self) -> Result<f32, String> {
        unsafe { read_memory::<f32>(self.entity_starts_at_addr + YAW_OFFSET) }
    }

    pub fn pitch(&self) -> Result<f32, String> {
        unsafe { read_memory::<f32>(self.entity_starts_at_addr + PITCH_OFFSET) }
    }

    pub fn health(&self) -> Result<i32, String> {
        unsafe { read_memory::<i32>(self.entity_starts_at_addr + HEALTH_OFFSET_FROM_LOCAL_PLAYER) }
    }

    pub fn is_alive(&self) -> bool {
        self.health().unwrap_or(0) > 0 // Assuming 0 health means not alive
    }

    pub fn position(&self) -> Result<Vec3, String> {
        unsafe {
            let x = read_memory::<f32>(self.entity_starts_at_addr + POSITION_X_FROM_LOCAL_PLAYER)?;
            let y = read_memory::<f32>(self.entity_starts_at_addr + POSITION_Y_FROM_LOCAL_PLAYER)?;
            let z = read_memory::<f32>(self.entity_starts_at_addr + POSITION_Z_FROM_LOCAL_PLAYER)?;
            Ok(Vec3 { x, y, z })
        }
    }

    pub fn head_position(&self) -> Result<Vec3, String> {
        unsafe {
            let x = read_memory::<f32>(self.entity_starts_at_addr + HEAD_X_FROM_LOCAL_PLAYER)?;
            let y = read_memory::<f32>(self.entity_starts_at_addr + HEAD_Y_FROM_LOCAL_PLAYER)?;
            let z = read_memory::<f32>(self.entity_starts_at_addr + HEAD_Z_FROM_LOCAL_PLAYER)?;
            Ok(Vec3 { x, y, z })
        }
    }

    pub fn team(&self) -> Result<i32, String> {
        unsafe { read_memory::<i32>(self.entity_starts_at_addr + TEAM_OFFSET_FROM_LOCAL_PLAYER) }
    }

    pub fn name(&self) -> Result<String, String> {
        let mut buffer: [u8; 256] = [0; 256];
        let address = self.entity_starts_at_addr + NAME_OFFSET_FROM_LOCAL_PLAYER;
        unsafe {
            if let Ok(()) = read_memory_into_slice(address, &mut buffer) {
                let name_end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
                let name = String::from_utf8_lossy(&buffer[..name_end]).into_owned();
                Ok(name)
            } else {
                Err(format!("Failed to read name at address {:x}", address))
            }
        }
    }

    // Generic read_value function to read a value of type T from an offset
    pub(crate) fn read_value<T>(&self, offset: usize) -> Result<T, String> {
        unsafe { read_memory::<T>(self.entity_starts_at_addr + offset) }
    }

    // Generic write_value function to write a value of type T to an offset
    pub(crate) fn write_value<T>(&mut self, offset: usize, value: T) -> Result<(), String> {
        unsafe { write_memory::<T>(self.entity_starts_at_addr + offset, value) }
    }
}
