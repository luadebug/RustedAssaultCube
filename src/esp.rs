use windows::Win32::Foundation::{HWND, TRUE};
use windows::Win32::Graphics::Gdi::{DeleteDC, DeleteObject, ReleaseDC, HBITMAP, HBRUSH, HDC};

#[allow(unused)]
unsafe fn esp_cleanup(
    window_handle_hwnd: HWND,
    hdc: HDC,
    mem_dc: HDC,
    mem_bitmap: HBITMAP,
    red_brush: HBRUSH,
    blue_brush: HBRUSH,
    background_brush: HBRUSH,
) -> Result<(), String> {
    unsafe {
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
    }
    Ok(())
}
