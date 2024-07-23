use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::Foundation::{COLORREF, GetLastError, RECT};
use windows::Win32::Graphics::Gdi::{CreateFontA, CreateSolidBrush, DeleteObject, Ellipse, FillRect, HBRUSH, HDC, Rectangle, SelectObject, SetBkMode, SetTextColor, TextOutA, TRANSPARENT};

use crate::entity::Entity;


pub unsafe fn draw_scaling_bar(
    hdc: HDC,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    width: f32, // Width for the border
    value: f32,
    max: f32,
    color: COLORREF,
) {
    // Calculate dimensions
    let width_diff = (x2 - x1) as i32;
    let height_diff = (y2 - y1) as i32;

    // Calculate the scaled height based on the value and max
    let scaled_height = if max > 0.0 {
        (height_diff as f32 * (value / max)).round() as i32
    } else {
        0 // Avoid division by zero
    };

    // Create a white brush for the background border
    let border_brush = CreateSolidBrush(COLORREF(0x00FFFFFF)); // White color for the border
    let old_brush = SelectObject(hdc, border_brush);

    // Draw the border rectangle
    Rectangle(
        hdc,
        x1 as i32,
        y1 as i32,
        (x1 + width_diff as f32) as i32,
        y2 as i32,
    );

    // Restore the old brush
    SelectObject(hdc, old_brush);
    DeleteObject(border_brush);

    // Create a brush for the filled area
    let fill_brush = CreateSolidBrush(color);
    let old_fill_brush = SelectObject(hdc, fill_brush);

    // Draw the filled rectangle for the scaled height
    if scaled_height > 0 {
        let fill_rect = RECT {
            left: x1 as i32,
            top: (y2 - scaled_height as f32) as i32, // Change top to fill from bottom
            right: x2 as i32,
            bottom: y2 as i32, // Bottom remains the same
        };
        FillRect(hdc, &fill_rect, fill_brush);
    }

    // Clean up: restore the old brush and delete the created brush
    SelectObject(hdc, old_fill_brush);
    DeleteObject(fill_brush);
}

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
    SetTextColor(hdc, COLORREF(0x00FF0000)); // Set text color to blue
    // Select the font into the device context
    let old_font = unsafe { SelectObject(hdc, font) };

    let name_str = ent.name(); // Get the name string
    if !name_str.is_empty() {
        // Set the background mode to transparent
        SetBkMode(hdc, TRANSPARENT);
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