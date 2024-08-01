#![cfg(windows)]

use std::ffi::c_void;

use windows::Win32::Foundation::{CloseHandle, BOOL, HMODULE, TRUE};
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

use vars::handles::CHEAT_DLL_HMODULE;

mod main_thread;

mod esp;
mod offsets;
mod ui;
mod utils;

mod aimbot;
mod angle;
mod distance;
mod draw_utils;
mod entity;
mod fonts;
mod game;
mod get_local_player_hook;
mod get_window_dimensions;
mod getclosestentity;
mod hotkey_widget;
mod memorypatch;
mod misc;
mod pattern_mask;
mod settings;
mod state;
mod style;
mod triggerbot_hook;
mod vars;
mod vec_structures;
mod wallhack_hook;
mod window_dimensions;
mod world_to_screen;
mod locales;
mod key_action;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, reserved: *mut c_void) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            DisableThreadLibraryCalls(dll_module)
                .expect("[lib.rs] Failed to disable thread library calls");
            CHEAT_DLL_HMODULE = dll_module.0 as isize;
            let handle = CreateThread(
                None,
                0,
                Some(main_thread::MainThread),
                Some(dll_module.0),
                THREAD_CREATION_FLAGS(0),
                None,
            )
            .expect("[lib.rs] Failed to create thread");
            if !handle.0.is_null() {
                CloseHandle(handle).expect("[lib.rs] Failed to close null handle.");
            }
        },
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    TRUE
}
