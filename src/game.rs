use std::ffi::c_void;

use windows::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect};

use crate::offsets::offsets::{BRIGHTNESS, SET_BRIGHTNESS};
use crate::vars::handles::AC_CLIENT_EXE_HMODULE;

pub unsafe fn c_brightness() -> *mut usize {
    (AC_CLIENT_EXE_HMODULE + BRIGHTNESS) as *mut usize
}

pub unsafe fn set_brightness() -> *mut usize {
    (AC_CLIENT_EXE_HMODULE + SET_BRIGHTNESS) as *mut usize
}

pub unsafe fn set_brightness_toggle(is_on: bool)
{
    if is_on
    {
        *c_brightness() = 100;
    }
    else
    {
        *c_brightness() = 40;
    }
    // Get the function pointer after setting the brightness
    let set_brightness_func = set_brightness();

    let mut old_protect = PAGE_PROTECTION_FLAGS(0);

    if VirtualProtect(set_brightness_func as *mut c_void,
                      512,
                      PAGE_EXECUTE_READWRITE,
                      &mut old_protect
    ).is_err()
    {
        println!("Failed to change memory protection of set_brightness procedure address to RWE");
    }

    static mut SET_BRIGHTNESS_FUNCTION: Option<unsafe extern "stdcall" fn() -> ()> = None;

    // Get the address somewhere in your code
    SET_BRIGHTNESS_FUNCTION = core::mem::transmute(set_brightness_func);


    // Then call it somewhere else
    SET_BRIGHTNESS_FUNCTION.unwrap()();


    if VirtualProtect(set_brightness_func as *mut c_void,
                      512,
                      old_protect,
                      &mut old_protect
    ).is_err()
    {
        println!("Failed to change memory protection of set_brightness procedure address to original memory protection flags");
    }
}