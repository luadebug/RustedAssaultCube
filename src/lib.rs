#![cfg(windows)]

use std::ffi::c_void;

use windows::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

use vars::handles::CHEAT_DLL_HMODULE;

mod main_thread;
mod trampoline;
mod ui;
mod esp;
mod utils;
mod offsets;

mod vec_structures;
mod entity;
mod get_window_dimensions;
mod window_dimensions;
mod draw_utils;
mod vars;
mod world_to_screen;
mod distance;
mod style;
mod memorypatch;
mod angle;
mod getclosestentity;
mod aimbot;
mod misc;
mod triggerbot;
mod game;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HMODULE,
    call_reason: u32,
    reserved: *mut c_void)
    -> BOOL
{
    match call_reason {
        DLL_PROCESS_ATTACH =>
            unsafe {
                DisableThreadLibraryCalls(dll_module).expect("[lib] Failed to disable thread library calls");
                CHEAT_DLL_HMODULE = dll_module.0 as isize;
                CreateThread(None,
                             0,
                             Some(main_thread::MainThread),
                             Some(dll_module.0),
                             THREAD_CREATION_FLAGS(0),
                             None).expect("[lib] Failed to create thread");
            }
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    TRUE
}



