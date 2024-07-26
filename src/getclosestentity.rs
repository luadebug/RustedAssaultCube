use std::ffi::CString;

use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::distance::distance_2d;
use crate::entity::Entity;
use crate::offsets::offsets::{ENTITY_LIST_OFFSET, LOCAL_PLAYER_OFFSET, NUMBER_OF_PLAYERS_IN_MATCH_OFFSET, VIEW_MATRIX_ADDR};
use crate::vars::game_vars::{ENTITY_LIST_PTR, FOV, LOCAL_PLAYER, NUM_PLAYERS_IN_MATCH, VIEW_MATRIX};
use crate::vars::handles::{AC_CLIENT_EXE_HMODULE, GAME_WINDOW_DIMENSIONS};
use crate::vec_structures::Vec2;
use crate::world_to_screen::world_to_screen;

pub unsafe fn get_closest_entity() -> Entity {
    let mut target = Entity::new();
    AC_CLIENT_EXE_HMODULE = {
        let ac_client_exe_cstring = CString::new("ac_client.exe").unwrap();
        GetModuleHandleA(PCSTR(ac_client_exe_cstring.as_ptr() as *const u8))
            .map(|hinstance| hinstance.0 as usize)
            .expect("[getcloseentity] Error getting module handle")
    };

    LOCAL_PLAYER = Entity::from_addr(*((AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) as *mut usize));
    VIEW_MATRIX = VIEW_MATRIX_ADDR as *mut [f32; 16];
    NUM_PLAYERS_IN_MATCH = *((AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET) as *const i32) as usize;
    ENTITY_LIST_PTR = *((AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) as *const u32);

    // Process each entity
    for i in 1..NUM_PLAYERS_IN_MATCH {
        if ENTITY_LIST_PTR == 0
        {
            continue;
        }
        let entity_addr = *((ENTITY_LIST_PTR as usize + i * 0x4) as *const usize);
        if entity_addr == 0
        {
            continue;
        }
        let enemy = Entity::from_addr(entity_addr);
        if LOCAL_PLAYER.entity_starts_at_addr == 0
        {
            continue;
        }
        if !enemy.is_alive() {
            continue;  // Skip dead enemies
        }
        if LOCAL_PLAYER.team() == enemy.team() {
            continue;   // Skip allies
        }
        let mut screen = Vec2 { x: 0.0, y: 0.0 };
        if !world_to_screen(enemy.head_position(), &mut screen, *VIEW_MATRIX, GAME_WINDOW_DIMENSIONS.width, GAME_WINDOW_DIMENSIONS.height) {
            continue;
        }
        let distance_to_fov:f32 = distance_2d(Vec2
                                                { x: GAME_WINDOW_DIMENSIONS.width as f32 / 2.0,
                                                  y: GAME_WINDOW_DIMENSIONS.height as f32 / 2.0
                                                }, screen);
        if distance_to_fov < FOV
        {
            target = enemy;
        }
    }

    target
}