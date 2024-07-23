use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::Foundation::{COLORREF, GetLastError, RECT};
use windows::Win32::Graphics::Gdi::{CreateFontA, CreateSolidBrush, DeleteObject, Ellipse, FillRect, HBRUSH, HDC, SelectObject, TextOutA};

use crate::entity::Entity;

fn draw_filled_rect(hdc: HDC, brush: HBRUSH, x: i32, y: i32, width: i32, height: i32) {
    let rect = RECT {
        left: x,
        top: y,
        right: x + width,
        bottom: y + height,
    };
    unsafe {
        FillRect(hdc, &rect as _, brush);
    }
}

pub unsafe fn draw_border_box(
    hdc: HDC,
    brush: HBRUSH,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    thickness: i32,
) {
    draw_filled_rect(hdc, brush, x, y, width, thickness);

    draw_filled_rect(hdc, brush, x, y, thickness, height);

    draw_filled_rect(hdc, brush, x + width, y, thickness, height);

    draw_filled_rect(hdc, brush, x, y + height, width + thickness, thickness);
}

pub unsafe fn draw_text(hdc: HDC, x: i32, y: i32, ent: &Entity) {

    // Create a font with the specified size
    let font_name = CString::new("Arial").unwrap(); // Choose a font name
    let font = unsafe {
        CreateFontA(
            32,         // Height of the font
            0,                 // Width of the font (0 means default width)
            0,                 // Angle of escapement
            0,                 // Base-line angle
            400,               // Weight (400 is normal)
            0,          // Italic
            0,          // Underline
            0,          // Strikeout
            0,                 // Character Set
            0,                 // Output Precision
            0,                 // Clipping Precision
            0,                 // Quality
            0,                 // Pitch and Family
            PCSTR(font_name.as_ptr() as *const u8) // Font name
        )
    };

    // Select the font into the device context
    let old_font = unsafe { SelectObject(hdc, font) };

    let name_str = ent.name(); // Get the name string
    if !name_str.is_empty() {
        // Draw the name string
        if TextOutA(
            hdc,
            x,
            y,
            name_str.to_bytes(),
        ) == false {
            println!("TextOutA failed {:?}", GetLastError());
        }
    }

    // Clean up: select the old font back into the device context and delete the created font
    unsafe {
        SelectObject(hdc, old_font);
        DeleteObject(font);
    }

}

pub unsafe fn draw_circle(hdc: HDC, center: (f32, f32), radius: f32, color: COLORREF) {
    // Create a brush using the specified color
    let brush = CreateSolidBrush(color);
    let old_brush = SelectObject(hdc, brush);

    // Calculate the rectangle bounding the circle
    let left = (center.0 - radius) as i32;
    let top = (center.1 - radius) as i32;
    let right = (center.0 + radius) as i32;
    let bottom = (center.1 + radius) as i32;

    // Draw the circle (ellipse with equal width and height)
    Ellipse(hdc, left, top, right, bottom);

    // Restore the old brush and delete the created brush
    SelectObject(hdc, old_brush);
    DeleteObject(brush);
}