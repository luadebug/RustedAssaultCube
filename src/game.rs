use std::ffi::c_void;
use crate::offsets::offsets::{BRIGHTNESS, SET_BRIGHTNESS};
use crate::vars::handles::AC_CLIENT_EXE_HMODULE;
use windows::Win32::System::Memory::{PAGE_PROTECTION_FLAGS, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, VirtualProtect};
pub unsafe fn c_brightness() -> *mut usize {
    (AC_CLIENT_EXE_HMODULE + BRIGHTNESS) as *mut usize
}

pub unsafe fn set_brightness() -> *mut usize {
    (AC_CLIENT_EXE_HMODULE + SET_BRIGHTNESS) as *mut usize
}

pub unsafe fn set_brightness_toggle(isON: bool)
{
    if isON
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

    static mut some_function: Option<unsafe extern "stdcall" fn() -> ()> = None;

    // Get the address somewhere in your code
    some_function = core::mem::transmute(set_brightness_func);


    // Then call it somewhere else
    (some_function.unwrap())();


    if VirtualProtect(set_brightness_func as *mut c_void,
                      512,
                      old_protect,
                      &mut old_protect
    ).is_err()
    {
        println!("Failed to change memory protection of set_brightness procedure address to original memory protection flags");
    }


}


/*use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_READWRITE};
use windows::Win32::System::Threading::OpenProcess;

const PROCESS_ALL_ACCESS: u32 = 0x1F0FFF;

fn main() {
    // Assuming `set_brightness` is a function you want to call
    let set_brightness_func: *const c_void = game::set_brightness() as *const c_void;

    // Step 1: Change the memory protection
    let mut old_protect: u32 = 0;
    let result = unsafe {
        VirtualProtect(
            set_brightness_func,
            4096, // Size of memory region (adjust as necessary)
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
    };

    if result.as_bool() {
        // Step 2: Now you can safely call the function pointer
        unsafe {
            asm!(
            "call *{0}",
            in(reg) set_brightness_func,
            options(noreturn)
            );
        }
    } else {
        println!("Failed to change memory protection: {:?}", result);
    }
}*/