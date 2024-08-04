use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowInfo, WINDOWINFO};

use crate::window_dimensions::WindowDimensions;

pub(crate) fn get_window_dimensions(window: HWND) -> Result<WindowDimensions, String> {
    let mut window_info = WINDOWINFO::default();
    window_info.cbSize = std::mem::size_of_val(&window_info) as u32;

    unsafe {
        match GetWindowInfo(window, &mut window_info) {
            Ok(_) => Ok(WindowDimensions {
                width: window_info.rcClient.right - window_info.rcClient.left,
                height: window_info.rcClient.bottom - window_info.rcClient.top,
            }),
            Err(_) => Err(format!(
                "Error getting screen dimensions. last_error={:?}",
                GetLastError()
            )),
        }
    }
}
