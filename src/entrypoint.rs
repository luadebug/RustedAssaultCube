use std::ffi::CString;
use std::result::Result::Ok;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

use windows::core::PCSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;

//use crate::draw_utils::{draw_border_box, draw_circle, draw_scaling_bar, draw_text};
use crate::entity::Entity;
use crate::get_window_dimensions::get_window_dimensions;
use crate::misc::init_mem_patches;
use crate::offsets::{
    ENTITY_LIST_OFFSET, LOCAL_PLAYER_OFFSET, NUMBER_OF_PLAYERS_IN_MATCH_OFFSET, VIEW_MATRIX_ADDR,
};
use crate::utils::{read_memory, read_view_matrix};
use crate::vars::game_vars::{ENTITY_LIST_PTR, VIEW_MATRIX};
use crate::vars::game_vars::LOCAL_PLAYER;
use crate::vars::game_vars::NUM_PLAYERS_IN_MATCH;
use crate::vars::handles::{AC_CLIENT_EXE_HMODULE, GAME_WINDOW_HANDLE};
use crate::vars::handles::GAME_WINDOW_DIMENSIONS;
use crate::vars::ui_vars::IS_ESP;

pub unsafe fn read_game_data() -> Result<(usize, usize, usize, [f32; 16]), String> {
    unsafe {
        // Read the local player address
        let local_player_addr =
            read_memory::<usize>(AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET)
                .map_err(|err| format!("Error reading local player address: {}", err))?;

        // Read the number of players in the match
        let num_players_in_match =
            read_memory::<usize>(AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET)
                .map_err(|err| {
                    format!("Error reading number of players in match: {}", err)
                })? as usize;

        // Read the entity list pointer
        let entity_list_ptr = read_memory::<usize>(AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET)
            .map_err(|err| format!("Error reading entity list pointer: {}", err))?;

        // Read the view matrix
        let view_matrix = read_view_matrix(VIEW_MATRIX_ADDR)
            .map_err(|err| format!("Error reading view matrix: {}", err))?;

        // Return the values as a tuple
        Ok((
            local_player_addr,
            num_players_in_match,
            entity_list_ptr,
            view_matrix,
        ))
    }
}

pub unsafe fn entrypoint() -> Result<(), String> {
    unsafe {
        // Initialize module handle
        AC_CLIENT_EXE_HMODULE = {
            let ac_client_exe_cstring = CString::new("ac_client.exe").unwrap();
            GetModuleHandleA(PCSTR(ac_client_exe_cstring.as_ptr() as *const u8))
                .map(|hinstance| hinstance.0 as usize)
                .expect("[esp] Error getting module handle")
        };

        println!(
            "[esp] Module base addr base_address={:#x}",
            AC_CLIENT_EXE_HMODULE
        );

        /*        let mut hdc;
        let mut mem_dc;
        let mut mem_bitmap;
        let mut red_brush;
        let mut green_brush;
        let mut background_brush;
        (hdc, mem_dc, mem_bitmap, red_brush, green_brush, background_brush) = init_esp_gdi()?;*/
        init_mem_patches();

        loop {
            if !IS_ESP.load(SeqCst) {
                println!("[esp] Turning off ESP");
                thread::sleep(Duration::from_millis(1000));
                continue;
            }

            /*println!("[esp] going to unwrap assaultcube window handle");*/
            let assault_cube_cstring = CString::new("AssaultCube").unwrap();
            /*            println!("[esp] going to unwrap FindWindowA");*/
            let window_handle_result = FindWindowA(
                PCSTR(std::ptr::null::<u8>()),
                PCSTR(assault_cube_cstring.as_ptr() as *const u8),
            );

            if window_handle_result.is_err() {
                let error_code = GetLastError();
                println!("[esp] FindWindowA failed with error code: {:?}", error_code);
            } else {
                GAME_WINDOW_HANDLE = window_handle_result.unwrap();
                /*                println!(
                    "[esp] FindWindowA succeeded with window handle: {:?}",
                    GAME_WINDOW_HANDLE.0
                );*/
            }

            // Get window dimensions
            /*            println!("[esp] Get window dimensions");*/
            let dimensions = get_window_dimensions(GAME_WINDOW_HANDLE)?;
            GAME_WINDOW_DIMENSIONS.width = dimensions.width;
            GAME_WINDOW_DIMENSIONS.height = dimensions.height;

            match read_game_data() {
                Ok((local_player_addr, num_players_in_match, entity_list_ptr, view_matrix)) => {
                    LOCAL_PLAYER = Entity::from_addr(local_player_addr);
                    NUM_PLAYERS_IN_MATCH = num_players_in_match;
                    ENTITY_LIST_PTR = entity_list_ptr;
                    // Copy the view matrix into the global variable
                    VIEW_MATRIX.copy_from_slice(&view_matrix);
                    /*                    render_esp_gdi(hdc, mem_dc,
                    mem_bitmap,
                    red_brush, green_brush, background_brush,
                    local_player_addr, num_players_in_match, entity_list_ptr, view_matrix);*/
                }
                Err(err) => {
                    println!("{}", err); // Handle the error appropriately
                    return Ok(());
                }
            }

            /*            handle_window_resize_gdi(&mut mem_dc, &mut mem_bitmap).unwrap();*/

            // Sleep to reduce CPU usage
            thread::sleep(Duration::from_millis(5));
        }
        /*
        // Cleanup resources at the end of the loop
        esp_cleanup(GAME_WINDOW_HANDLE, hdc, mem_dc, mem_bitmap, red_brush, green_brush, background_brush)
            .expect("[esp] Failed to deallocate hdcs, brushes, and bitmaps.");

        Ok(())*/
    }
}
