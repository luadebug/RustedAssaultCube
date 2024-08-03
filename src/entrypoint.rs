use std::ffi::{c_void, CString};
use std::ptr::null;
use std::result::Result::Ok;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

use windows::core::PCSTR;
use windows::Win32::Foundation::{GetLastError, COLORREF, FALSE, TRUE};
use windows::Win32::Graphics::Gdi::{
    AddFontMemResourceEx, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC,
    DeleteObject, GetDC, InvalidateRect, ReleaseDC, SelectObject, TransparentBlt, HBITMAP,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, GetWindowLongA, SetWindowLongA, GWL_EXSTYLE, WS_EX_TRANSPARENT,
};
use windows::Win32::{
    Foundation::RECT,
    Graphics::Gdi::{FillRect, HBRUSH, HDC},
};

use crate::distance;
use crate::draw_utils::{draw_border_box, draw_circle, draw_scaling_bar, draw_text};
use crate::entity::Entity;
use crate::get_window_dimensions::get_window_dimensions;
use crate::misc::init_mem_patches;
use crate::offsets::offsets::{
    ENTITY_LIST_OFFSET, LOCAL_PLAYER_OFFSET, NUMBER_OF_PLAYERS_IN_MATCH_OFFSET, VIEW_MATRIX_ADDR,
};
use crate::utils::{read_memory, read_view_matrix};
use crate::vars::game_vars::LOCAL_PLAYER;
use crate::vars::game_vars::NUM_PLAYERS_IN_MATCH;
use crate::vars::game_vars::{ENTITY_LIST_PTR, FOV, VIEW_MATRIX};
use crate::vars::handles::GAME_WINDOW_DIMENSIONS;
use crate::vars::handles::{AC_CLIENT_EXE_HMODULE, GAME_WINDOW_HANDLE};
use crate::vars::ui_vars::{IS_DRAW_FOV, IS_ESP};
use crate::vec_structures::Vec2;
use crate::world_to_screen::world_to_screen;

#[allow(unused)]
pub unsafe fn init_esp_gdi() -> Result<(HDC, HDC, HBITMAP, HBRUSH, HBRUSH, HBRUSH), Box<String>> {
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

        println!("[esp] going to unwrap assaultcube window handle");
        let assault_cube_cstring = CString::new("AssaultCube").unwrap();
        println!("[esp] going to unwrap FindWindowA");
        let window_handle_result = FindWindowA(
            PCSTR(0 as *const u8),
            PCSTR(assault_cube_cstring.as_ptr() as *const u8),
        );

        if window_handle_result.is_err() {
            let error_code = GetLastError();
            println!("[esp] FindWindowA failed with error code: {:?}", error_code);
        } else {
            GAME_WINDOW_HANDLE = window_handle_result.unwrap();
            println!(
                "[esp] FindWindowA succeeded with window handle: {:?}",
                GAME_WINDOW_HANDLE.0
            );
        }

        // Set window style to WS_EX_TRANSPARENT
        let ex_style = GetWindowLongA(GAME_WINDOW_HANDLE, GWL_EXSTYLE);
        SetWindowLongA(
            GAME_WINDOW_HANDLE,
            GWL_EXSTYLE,
            ex_style | WS_EX_TRANSPARENT.0 as i32,
        );

        println!("[esp] Get device context and create a compatible DC");
        // Get device context for the game window
        let hdc = GetDC(GAME_WINDOW_HANDLE);
        if hdc.0.is_null() {
            return Err(Box::new("Failed to get device context".to_string()));
        }

        // Create a compatible device context (DC)
        let mem_dc = CreateCompatibleDC(hdc);
        if mem_dc.0.is_null() {
            ReleaseDC(GAME_WINDOW_HANDLE, hdc);
            return Err(Box::new("Failed to create compatible DC".to_string()));
        }

        // Get window dimensions
        println!("[esp] Get window dimensions");
        let dimensions = get_window_dimensions(GAME_WINDOW_HANDLE)?;
        GAME_WINDOW_DIMENSIONS.width = dimensions.width;
        GAME_WINDOW_DIMENSIONS.height = dimensions.height;

        // Create a compatible bitmap for double buffering
        println!("[esp] Create a compatible bitmap for double buffering");
        let mut mem_bitmap = CreateCompatibleBitmap(
            hdc,
            GAME_WINDOW_DIMENSIONS.width,
            GAME_WINDOW_DIMENSIONS.height,
        );
        SelectObject(mem_dc, mem_bitmap); // Select the bitmap into the DC

        // Initialize game entity data
        println!("[esp] Initialize game entity data");
        match read_view_matrix(VIEW_MATRIX_ADDR) {
            Ok(matrix) => {
                VIEW_MATRIX.copy_from_slice(&matrix);
            }
            Err(err) => {
                println!("Error reading view matrix: {}", err);
                return Err(Box::new(
                    format!("Error reading view matrix: {}", err).to_string(),
                ));
            }
        };

        println!("[esp] Create brushes for drawing");
        // Create brushes for drawing
        let red_brush = CreateSolidBrush(COLORREF(0x000000FF)); // Red Enemy
        let green_brush = CreateSolidBrush(COLORREF(0x0000FF00)); // Green Ally
        let background_brush = CreateSolidBrush(COLORREF(0x00000000)); // Transparent

        // Check if the brushes are created successfully
        if red_brush.0.is_null() || green_brush.0.is_null() || background_brush.0.is_null() {
            // Cleanup on error
            let _ = DeleteDC(mem_dc);
            ReleaseDC(GAME_WINDOW_HANDLE, hdc);
            return Err(Box::new("Failed to create brushes".to_string()));
        }
        println!("[esp] Create clash font from memory resource");
        // Create a Clash font with the specified size to use later
        let mut n_fonts: u32 = 0;

        let font_handle = AddFontMemResourceEx(
            crate::fonts::clash_font::CLASH.as_ptr() as *const c_void, // Pointer to the font resource
            crate::fonts::clash_font::CLASH.len() as u32,              // Size of the font resource
            Some(null()),                                              // Reserved (must be NULL)
            &mut n_fonts,
        ); // Number of fonts installed

        if font_handle.0.is_null() {
            println!(
                "Failed to add font from memory, error: {:?}",
                GetLastError()
            );
            return Err(Box::new(
                format!(
                    "Failed to add font from memory, error: {:?}",
                    GetLastError()
                )
                .to_string(),
            ));
        }

        // Return the handles and brushes
        Ok((
            hdc,
            mem_dc,
            mem_bitmap,
            red_brush,
            green_brush,
            background_brush,
        ))
    }
}

pub unsafe fn read_game_data() -> Result<(usize, usize, usize, [f32; 16]), Box<String>> {
    unsafe {
        // Read the local player address
        let local_player_addr =
            read_memory::<usize>(AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET)
                .map_err(|err| Box::new(format!("Error reading local player address: {}", err)))?;

        // Read the number of players in the match
        let num_players_in_match =
            read_memory::<usize>(AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET)
                .map_err(|err| {
                    Box::new(format!("Error reading number of players in match: {}", err))
                })? as usize;

        // Read the entity list pointer
        let entity_list_ptr = read_memory::<usize>(AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET)
            .map_err(|err| Box::new(format!("Error reading entity list pointer: {}", err)))?;

        // Read the view matrix
        let view_matrix = read_view_matrix(VIEW_MATRIX_ADDR)
            .map_err(|err| Box::new(format!("Error reading view matrix: {}", err)))?;

        // Return the values as a tuple
        Ok((
            local_player_addr,
            num_players_in_match,
            entity_list_ptr,
            view_matrix,
        ))
    }
}
#[allow(unused)]
pub unsafe fn render_esp_gdi(
    hdc: HDC,
    mem_dc: HDC,
    mem_bitmap: HBITMAP,
    red_brush: HBRUSH,
    green_brush: HBRUSH,
    background_brush: HBRUSH,
    local_player_addr: usize,
    num_players_in_match: usize,
    entity_list_ptr: usize,
    view_matrix: [f32; 16],
) {
    unsafe {
        // Clear the memory DC before drawing (set to transparent)
        FillRect(
            mem_dc,
            &RECT {
                left: 0,
                top: 0,
                right: GAME_WINDOW_DIMENSIONS.width,
                bottom: GAME_WINDOW_DIMENSIONS.height,
            },
            background_brush,
        );

        let mut invalidated_area = RECT {
            left: 0,
            top: 0,
            right: GAME_WINDOW_DIMENSIONS.width,
            bottom: GAME_WINDOW_DIMENSIONS.height,
        };

        // Process each entity
        for i in 1..num_players_in_match {
            let entity_addr = match Entity::from_addr(entity_list_ptr).read_value::<usize>(i * 0x4)
            {
                Ok(addr) => addr,
                Err(err) => {
                    println!("Error reading entity address: {}", err);
                    continue;
                }
            };

            if entity_addr == 0 {
                continue;
            }

            let entity = Entity::from_addr(entity_addr);

            // Check if the entity is alive
            if !entity.is_alive() {
                continue;
            }

            let mut feet_screen_pos = Vec2 { x: 0.0, y: 0.0 };
            let mut head_screen_pos = Vec2 { x: 0.0, y: 0.0 };

            // Use match expressions for error handling with position and head_position
            let entity_position = match entity.position() {
                Ok(pos) => pos,
                Err(err) => {
                    println!("Error reading entity position: {}", err);
                    continue; // Skip to the next entity if there's an error
                }
            };

            let entity_head_position = match entity.head_position() {
                Ok(pos) => pos,
                Err(err) => {
                    println!("Error reading entity head position: {}", err);
                    continue; // Skip to the next entity if there's an error
                }
            };

            // Convert world coordinates to screen coordinates
            if !world_to_screen(
                entity_position,
                &mut feet_screen_pos,
                view_matrix,
                GAME_WINDOW_DIMENSIONS.width,
                GAME_WINDOW_DIMENSIONS.height,
            ) {
                continue;
            }

            if !world_to_screen(
                entity_head_position,
                &mut head_screen_pos,
                view_matrix,
                GAME_WINDOW_DIMENSIONS.width,
                GAME_WINDOW_DIMENSIONS.height,
            ) {
                continue;
            }

            // Draw box around the entity
            let distance = match (LOCAL_PLAYER.position(), entity.position()) {
                (Ok(local_pos), Ok(entity_pos)) => distance::distance_3d(local_pos, entity_pos),
                (Err(err), _) | (_, Err(err)) => {
                    println!("Error reading position: {}", err);
                    continue;
                }
            };

            let box_width = (GAME_WINDOW_DIMENSIONS.width as f32 / distance) as i32;
            let box_height = (GAME_WINDOW_DIMENSIONS.height as f32 / distance * 3.5) as i32;
            let box_left = (feet_screen_pos.x - box_width as f32 / 2.0) as i32;
            let box_top = (feet_screen_pos.y - box_height as f32) as i32;
            let box_brush_color = if LOCAL_PLAYER.team() == entity.team() {
                green_brush
            } else {
                red_brush
            };

            // Draw the FOV circle if enabled
            if IS_DRAW_FOV.load(SeqCst) {
                draw_circle(
                    hdc,
                    (
                        GAME_WINDOW_DIMENSIONS.width as f32 / 2.0,
                        GAME_WINDOW_DIMENSIONS.height as f32 / 2.0,
                    ),
                    FOV.load(SeqCst) as f32,
                    COLORREF(0x00FFFFFF),
                );
            }

            // Draw text and border box for the entity
            draw_text(
                mem_dc,
                feet_screen_pos.x as i32,
                feet_screen_pos.y as i32,
                &entity,
            );
            draw_border_box(
                mem_dc,
                box_brush_color,
                box_left,
                box_top,
                box_width,
                box_height,
                5,
            );

            // Draw health bar
            match entity.health() {
                Ok(health) => unsafe {
                    draw_scaling_bar(
                        mem_dc,
                        head_screen_pos.x - 55.0,
                        head_screen_pos.y,
                        feet_screen_pos.x - 15.0,
                        feet_screen_pos.y,
                        health as f32,
                        100.0,
                        COLORREF(0x0000FF00),
                    );
                },
                Err(err) => {
                    println!("Error reading entity health: {}", err);
                }
            }

            // Update the invalidated area to encompass all drawn entities
            invalidated_area.left = invalidated_area.left.min(box_left);
            invalidated_area.top = invalidated_area.top.min(box_top);
            invalidated_area.right = invalidated_area.right.max(box_left + box_width);
            invalidated_area.bottom = invalidated_area.bottom.max(box_top + box_height);
        }
        unsafe {
            // Invalidate the combined area of all drawn entities
            if InvalidateRect(GAME_WINDOW_HANDLE, Some(&invalidated_area), TRUE) == FALSE {
                println!("[esp] InvalidateRect failed {:?}", GetLastError());
            }

            // Perform the transparent blit
            if TransparentBlt(
                hdc,
                0,
                0,
                GAME_WINDOW_DIMENSIONS.width,
                GAME_WINDOW_DIMENSIONS.height,
                mem_dc,
                0,
                0,
                GAME_WINDOW_DIMENSIONS.width,
                GAME_WINDOW_DIMENSIONS.height,
                0x00000000,
            ) == FALSE
            {
                println!("[esp] TransparentBlt failed {:?}", GetLastError());
            }
        }
    }
}

// Function to handle window resize
#[allow(unused)]
pub unsafe fn handle_window_resize_gdi(
    mem_dc: &mut HDC,         // Mutable pointer to the device context
    mem_bitmap: &mut HBITMAP, // Mutable pointer to the bitmap
) -> Result<(), &'static str> {
    unsafe {
        // Check for window resize
        if GAME_WINDOW_DIMENSIONS.width != get_window_dimensions(GAME_WINDOW_HANDLE).unwrap().width
            || GAME_WINDOW_DIMENSIONS.height
                != get_window_dimensions(GAME_WINDOW_HANDLE).unwrap().height
        {
            let new_dimensions = get_window_dimensions(GAME_WINDOW_HANDLE).unwrap();
            let new_width = new_dimensions.width;
            let new_height = new_dimensions.height;

            // Only recreate if dimensions have actually changed
            if new_width != GAME_WINDOW_DIMENSIONS.width
                || new_height != GAME_WINDOW_DIMENSIONS.height
            {
                println!("[esp] Window resized to: {}x{}", new_width, new_height);

                // Cleanup old resources
                let _ = DeleteObject(*mem_bitmap);
                let _ = DeleteDC(*mem_dc);

                // Create a new compatible DC and bitmap with the new dimensions
                let hdc = GetDC(GAME_WINDOW_HANDLE);
                *mem_dc = CreateCompatibleDC(hdc);
                *mem_bitmap = CreateCompatibleBitmap(hdc, new_width, new_height);
                SelectObject(*mem_dc, *mem_bitmap); // Select the new bitmap into the DC

                // Update current dimensions
                GAME_WINDOW_DIMENSIONS.width = new_width;
                GAME_WINDOW_DIMENSIONS.height = new_height;

                // Release the device context
                ReleaseDC(GAME_WINDOW_HANDLE, hdc);
            }
        }
    }
    Ok(())
}

pub unsafe fn entrypoint() -> Result<(), Box<String>> {
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
                PCSTR(0 as *const u8),
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