use std::ffi::{c_void, CString};
use windows::Win32::Globalization::{CP_UTF8, MULTI_BYTE_TO_WIDE_CHAR_FLAGS, MultiByteToWideChar, WideCharToMultiByte};
use crate::offsets::offsets::{HEAD_X_FROM_LOCAL_PLAYER, HEAD_Y_FROM_LOCAL_PLAYER, HEAD_Z_FROM_LOCAL_PLAYER, HEALTH_OFFSET_FROM_LOCAL_PLAYER, NAME_OFFSET_FROM_LOCAL_PLAYER, POSITION_X_FROM_LOCAL_PLAYER, POSITION_Y_FROM_LOCAL_PLAYER, POSITION_Z_FROM_LOCAL_PLAYER, TEAM_OFFSET_FROM_LOCAL_PLAYER};
use crate::utils::read_memory;
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

    pub fn health(&self) -> i32 {
        unsafe { *((self.entity_starts_at_addr + HEALTH_OFFSET_FROM_LOCAL_PLAYER) as *const i32) }
    }

    pub fn is_alive(&self) -> bool {
        self.health() > 0
    }

    pub fn position(&self) -> Vec3 {
        unsafe {
            Vec3 {
                x: *((self.entity_starts_at_addr + POSITION_X_FROM_LOCAL_PLAYER) as *const f32),
                y: *((self.entity_starts_at_addr + POSITION_Y_FROM_LOCAL_PLAYER) as *const f32),
                z: *((self.entity_starts_at_addr + POSITION_Z_FROM_LOCAL_PLAYER) as *const f32),
            }
        }
    }

    pub fn head_position(&self) -> Vec3 {
        unsafe {
            Vec3 {
                x: *((self.entity_starts_at_addr + HEAD_X_FROM_LOCAL_PLAYER) as *const f32),
                y: *((self.entity_starts_at_addr + HEAD_Y_FROM_LOCAL_PLAYER) as *const f32),
                z: *((self.entity_starts_at_addr + HEAD_Z_FROM_LOCAL_PLAYER) as *const f32),
            }
        }
    }

    pub fn team(&self) -> i32 {
        unsafe { *((self.entity_starts_at_addr + TEAM_OFFSET_FROM_LOCAL_PLAYER) as *const i32) }
    }

    pub fn name(&self) -> CString {
        // Buffer to hold the name (adjust size if necessary)
        let mut buffer: [u8; 256] = [0; 256];

        // Calculate the address to read from
        let address = (self.entity_starts_at_addr + NAME_OFFSET_FROM_LOCAL_PLAYER) as *const c_void;

        // Read memory into the buffer
        if unsafe { !read_memory(address, buffer.as_mut_ptr() as *mut c_void, buffer.len()) } {
            return CString::new("").unwrap(); // Return empty string if reading fails
        }

        // Finding valid length until the first null byte
        let length = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());



        // Convert buffer to a valid UTF-8 string
        let utf8_str = std::str::from_utf8(&buffer[..length])
            .unwrap_or_else(|_| {
                eprintln!("Error converting bytes to UTF-8");
                ""
            });

        // Create a CString from the UTF-8 string
        CString::new(utf8_str).unwrap_or_else(|_| {
            eprintln!("Error creating CString from UTF-8 string");
            CString::new("").unwrap() // Return empty string on error
        })
    }
}
