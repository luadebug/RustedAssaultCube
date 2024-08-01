use std::ffi::c_void;

use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

use crate::offsets::offsets::{BRIGHTNESS, SET_BRIGHTNESS};
use crate::utils::write_memory;
use crate::vars::handles::AC_CLIENT_EXE_HMODULE;

pub unsafe fn c_brightness() -> *mut usize {
    unsafe { (AC_CLIENT_EXE_HMODULE + BRIGHTNESS) as *mut usize }
}

pub unsafe fn set_brightness() -> *mut usize {
    unsafe { (AC_CLIENT_EXE_HMODULE + SET_BRIGHTNESS) as *mut usize }
}

pub unsafe fn set_brightness_toggle(is_on: bool) {
    if is_on {
        unsafe {
            if let Err(e) = write_memory(c_brightness() as usize, 100) {
                println!("Error writing brightness: {}", e);
            }
        }
    } else {
        unsafe {
            if let Err(e) = write_memory(c_brightness() as usize, 40) {
                println!("Error writing brightness: {}", e);
            }
        }
    }
    // Get the function pointer after setting the brightness
    unsafe {
        let set_brightness_func = set_brightness();

        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        if VirtualProtect(
            set_brightness_func as *mut c_void,
            512,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
        .is_err()
        {
            println!(
                "Failed to change memory protection of set_brightness procedure address to RWE"
            );
        }

        static mut SET_BRIGHTNESS_FUNCTION: Option<unsafe extern "stdcall" fn() -> ()> = None;

        // Get the address somewhere in your code
        SET_BRIGHTNESS_FUNCTION = core::mem::transmute(set_brightness_func);

        // Then call it somewhere else
        SET_BRIGHTNESS_FUNCTION.unwrap()();

        if VirtualProtect(
            set_brightness_func as *mut c_void,
            512,
            old_protect,
            &mut old_protect,
        )
        .is_err()
        {
            println!("Failed to change memory protection of set_brightness procedure address to original memory protection flags");
        }
    }
}
