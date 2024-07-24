use std::ffi::CString;
use std::ptr::null;
use std::result::Result::Ok;
use std::thread;

use windows::core::PCSTR;
use windows::Win32::{Foundation::RECT, Graphics::Gdi::{FillRect, HBRUSH, HDC}};
use windows::Win32::Foundation::{COLORREF, FALSE, GetLastError, HWND, TRUE};
use windows::Win32::Graphics::Gdi::{CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC, DeleteObject, GetDC, HBITMAP, InvalidateRect, ReleaseDC, SelectObject, TransparentBlt};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowLongA, GWL_EXSTYLE, SetWindowLongA, WS_EX_TRANSPARENT};

use crate::aimbot::aimbot;
use crate::distance;
use crate::draw_utils::{draw_border_box, draw_circle, draw_scaling_bar, draw_text};
use crate::entity::Entity;
use crate::get_window_dimensions::get_window_dimensions;
use crate::misc::{init_mem_patches, player_fields_monitor};
use crate::offsets::offsets::{ENTITY_LIST_OFFSET, LOCAL_PLAYER_OFFSET, NUMBER_OF_PLAYERS_IN_MATCH_OFFSET, VIEW_MATRIX_ADDR};
use crate::vars::game_vars::{ENTITY_LIST_PTR, FOV, VIEW_MATRIX};
use crate::vars::game_vars::LOCAL_PLAYER;
use crate::vars::game_vars::NUM_PLAYERS_IN_MATCH;
use crate::vars::handles::AC_CLIENT_EXE_HMODULE;
use crate::vars::handles::GAME_WINDOW_DIMENSIONS;
use crate::vars::handles::GAME_WINDOW_HANDLE;
use crate::vars::ui_vars::{IS_DRAW_FOV, IS_ESP};
use crate::vec_structures::Vec2;
use crate::world_to_screen::world_to_screen;

unsafe fn esp_cleanup(
    window_handle_hwnd: HWND,
    hdc: HDC,
    mem_dc: HDC,
    mem_bitmap: HBITMAP,
    red_brush: HBRUSH,
    blue_brush: HBRUSH,
    background_brush: HBRUSH,
) -> Result<(), Box<String>> {
    // Attempt to delete the memory bitmap and log the result
    if DeleteObject(mem_bitmap) == TRUE {
        println!("[esp] Successfully deallocated mem_bitmap.");
    } else {
        println!("[esp] Failed to deallocate mem_bitmap.");
    }

    // Attempt to delete the memory DC and log the result
    if DeleteDC(mem_dc) == TRUE {
        println!("[esp] Successfully deallocated mem_dc.");
    } else {
        println!("[esp] Failed to deallocate mem_dc.");
    }

    // Release the WINDOW's DC and log the result
    if ReleaseDC(window_handle_hwnd, hdc) != 0 {
        println!("[esp] Successfully released the WINDOW's DC.");
    } else {
        println!("[esp] Failed to release the WINDOW's DC.");
    }

    // Attempt to delete the brushes and log the result
    if DeleteObject(red_brush) == TRUE {
        println!("[esp] Successfully deallocated red_brush.");
    } else {
        println!("[esp] Failed to deallocate red_brush.");
    }

    if DeleteObject(blue_brush) == TRUE {
        println!("[esp] Successfully deallocated blue_brush.");
    } else {
        println!("[esp] Failed to deallocate blue_brush.");
    }

    if DeleteObject(background_brush) == TRUE {
        println!("[esp] Successfully deallocated background_brush.");
    } else {
        println!("[esp] Failed to deallocate background_brush.");
    }

    Ok(())
}


pub unsafe fn esp_entrypoint() -> Result<(), Box<String>> {
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


    VIEW_MATRIX = VIEW_MATRIX_ADDR as *mut [f32; 16];
    println!("[esp] going to unwrap assaultcube window handle");
    let assault_cube_cstring = CString::new("AssaultCube").unwrap();
    println!("[esp]going to unwrap FindWindowA");
    let window_handle_result = FindWindowA(PCSTR(0 as *const u8), PCSTR(assault_cube_cstring.as_ptr() as *const u8));

    if window_handle_result.is_err() {
        let error_code = GetLastError();
        println!("[esp] FindWindowA failed with error code: {:?}", error_code);
    } else {
        GAME_WINDOW_HANDLE = window_handle_result.unwrap();
        println!("[esp] FindWindowA succeeded with window handle: {:?}", GAME_WINDOW_HANDLE.0);
    }


    // Set window style to WS_EX_TRANSPARENT
    //println!("[esp] Set window style to Getwinlong");
    let ex_style = GetWindowLongA(GAME_WINDOW_HANDLE, GWL_EXSTYLE);
    //println!("[esp] Set window style to Setwinl");
    SetWindowLongA(GAME_WINDOW_HANDLE, GWL_EXSTYLE, ex_style | WS_EX_TRANSPARENT.0 as i32);




/*    OG_WND_PROC = SetWindowLongPtrA(GAME_WINDOW_HANDLE,
                                    WINDOW_LONG_PTR_INDEX(GWL_WNDPROC.0),
                                    (hook_wnd_process as *c_void) as i32);*/
    // Get device context and create a compatible DC
    println!("[esp] Get device context and create a compatible DC");
    let hdc = GetDC(GAME_WINDOW_HANDLE);
    let mem_dc = CreateCompatibleDC(hdc);

    // Get window dimensions
    println!("[esp] Get window dimensions");
    GAME_WINDOW_DIMENSIONS = get_window_dimensions(GAME_WINDOW_HANDLE)?;
    let width = GAME_WINDOW_DIMENSIONS.width;
    let height = GAME_WINDOW_DIMENSIONS.height;

    // Create a compatible bitmap for double buffering
    println!("[esp] Create a compatible bitmap for double buffering)");
    let mem_bitmap = CreateCompatibleBitmap(hdc, width, height);
    SelectObject(mem_dc, mem_bitmap); // Select the bitmap into the DC

    // Initialize game entity data
    println!("[esp] Initialize game entity data");
    LOCAL_PLAYER = Entity::from_addr(*((AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) as *mut usize));
    NUM_PLAYERS_IN_MATCH = *((AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET) as *const i32) as usize;
    ENTITY_LIST_PTR = *((AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) as *const u32);

    // Create brushes for drawing
    println!("[esp] Create brushes for drawing");
    let red_brush = CreateSolidBrush(COLORREF(0x000000FF)); // Red Enemy
    let green_brush = CreateSolidBrush(COLORREF(0x0000FF00)); // Green Ally
    let background_brush = CreateSolidBrush(COLORREF(0x00000000)); // Transparent
    println!("[esp] Getting into the ESP loop");


    init_mem_patches();
    loop {
        if (AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) as *const usize != null() &&
            LOCAL_PLAYER.entity_starts_at_addr !=
                *((AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) as *mut usize) {
            println!("[esp] Local player not found");
            LOCAL_PLAYER = Entity::from_addr(
                *((AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) as *mut usize));
        }

        if (AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET)
            as *const i32 != null() &&
        *((AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET)
            as *const i32) as usize != NUM_PLAYERS_IN_MATCH {
            println!("[esp] Number of players in match not found");
            NUM_PLAYERS_IN_MATCH = *((AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET)
                as *const i32) as usize;
        }

        if (AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) as *const u32 != null() &&
        *((AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) as *const u32) != ENTITY_LIST_PTR {
            println!("[esp] Entity list ptr not found");
            ENTITY_LIST_PTR = *((AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) as *const u32);
        }
        player_fields_monitor();
        if !IS_ESP {
            println!("[esp] Turning off ESP");
            thread::sleep(std::time::Duration::from_millis(1000));
            continue;
        }
        // Clear the memory DC before drawing (set to transparent)
        FillRect(mem_dc, &RECT { left: 0, top: 0, right: width, bottom: height }, background_brush); //Graphics::Gdi::CreatePatternBrush(create_transparent_bitmap(width, height))); // Set as fully transparent

        let mut invalidated_area = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };

        // Process each entity
        for i in 1..NUM_PLAYERS_IN_MATCH {
            let entity_addr = *((ENTITY_LIST_PTR as usize + i * 0x4) as *const usize);
            let entity = Entity::from_addr(entity_addr);
            if !entity.is_alive() {
                continue;
            }
            let mut feet_screen_pos = Vec2 { x: 0.0, y: 0.0 };
            //if !world_to_screen::world_to_screen(entity.position(), &mut screen, *view_matrix, GAME_WINDOW_DIMENSIONS.width, GAME_WINDOW_DIMENSIONS.height) {
            if !world_to_screen(entity.position(), &mut feet_screen_pos, *VIEW_MATRIX, GAME_WINDOW_DIMENSIONS.width, GAME_WINDOW_DIMENSIONS.height) {
                continue;
            }
            let mut head_screen_pos = Vec2 {x: 0.0, y: 0.0};
            if !world_to_screen(entity.head_position(), &mut head_screen_pos, *VIEW_MATRIX, GAME_WINDOW_DIMENSIONS.width, GAME_WINDOW_DIMENSIONS.height) {
                continue;
            }
/*            let mut topLeft: Vec2 = Vec2::new(head_screen_pos.x - width as f32, head_screen_pos.y);
            let mut topRight: Vec2 = Vec2::new(head_screen_pos.x + width as f32, head_screen_pos.y);
            let mut bottomRight: Vec2 = Vec2::new(feet_screen_pos.x + width as f32, feet_screen_pos.y);
            let mut bottomLeft: Vec2 = Vec2::new(feet_screen_pos.x - width as f32, feet_screen_pos.y);*/
            // Draw box
            let distance = distance::distance_3d(LOCAL_PLAYER.position(), entity.position());
            let box_width = (GAME_WINDOW_DIMENSIONS.width as f32 / distance) as i32;
            let box_height = (GAME_WINDOW_DIMENSIONS.height as f32 / distance * 3.5) as i32;
            let box_left = (feet_screen_pos.x - box_width as f32 / 2.0) as i32;
            let box_top = (feet_screen_pos.y - box_height as f32) as i32;
            let box_brush_color = if LOCAL_PLAYER.team() == entity.team() {
                green_brush
            } else {
                red_brush
            };
            //aimbot(mem_dc);
            if IS_DRAW_FOV
            {
                draw_circle(hdc, (GAME_WINDOW_DIMENSIONS.width as f32 / 2.0,
                                              GAME_WINDOW_DIMENSIONS.height as f32 / 2.0),
                                        FOV,
                                        COLORREF(0x00FFFFFF));
            }
            draw_text(mem_dc, feet_screen_pos.x as i32, feet_screen_pos.y as i32, &entity);
            draw_border_box(mem_dc, box_brush_color,
                            box_left, box_top,
                            box_width, box_height, 5);
            draw_scaling_bar(mem_dc,
                             head_screen_pos.x - 55.0, head_screen_pos.y,
                             feet_screen_pos.x - 15.0, feet_screen_pos.y,
                             box_width as f32 * 2.5,
                             entity.health() as f32, 100.0,
                             COLORREF(0x0000FF00));
            // Update the invalidated area to encompass all drawn entities
            invalidated_area.left = invalidated_area.left.min(box_left);
            invalidated_area.top = invalidated_area.top.min(box_top);
            invalidated_area.right = invalidated_area.right.max(box_left + box_width);
            invalidated_area.bottom = invalidated_area.bottom.max(box_top + box_height);




        }


        // Invalidate the combined area of all drawn entities
        if InvalidateRect(GAME_WINDOW_HANDLE, Some(&invalidated_area), TRUE) == FALSE {
            println!("[esp] InvalidateRect failed {:?}", GetLastError());
        }
        if TransparentBlt(hdc, 0, 0, width, height,
                          mem_dc, 0, 0, width, height,
                          0x00000000) == FALSE {
            println!("[esp] TransparentBlt failed {:?}", GetLastError());
        }
        // Sleep to reduce CPU usage
        thread::sleep(std::time::Duration::from_millis(5));
    }


    // Cleanup
/*    esp_cleanup(GAME_WINDOW_HANDLE, hdc, mem_dc, mem_bitmap,
                red_brush, green_brush, background_brush)
        .expect("[esp] Failed to deallocate hdcs, brushes and bitmaps.");
    Ok(())*/
}



