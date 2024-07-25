use std::{mem, thread};
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;

use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookType, Registers};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_MOUSE, MOUSE_EVENT_FLAGS, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SendInput};

use crate::entity::Entity;
use crate::utils::find_pattern;
use crate::vars::game_vars::{CURRENT_CROSSHAIR_ENTITY_ADDR, LOCAL_PLAYER, TRIGGER_DELAY};
use crate::vars::hooks::HOOK;
use crate::vars::ui_vars::IS_TRIGGERBOT;

// The function to be hooked
#[inline(never)]
pub(crate) unsafe extern "cdecl" fn get_crosshair_entity(
    reg: *mut Registers,
    _: usize
) {
    if IS_TRIGGERBOT
    {
        let mut input: INPUT = mem::zeroed();
        input.r#type = INPUT_MOUSE; // Set the type to INPUT_MOUSE
        input.Anonymous.mi.dx = 0; // Mouse movement in X (0 for no movement)
        input.Anonymous.mi.dy = 0; // Mouse movement in Y (0 for no movement)
        input.Anonymous.mi.mouseData = 0; // Additional data (not used)
        input.Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0); // No flags initially
        input.Anonymous.mi.time = 0; // Use default time
        input.Anonymous.mi.dwExtraInfo = 0; // No extra info

        CURRENT_CROSSHAIR_ENTITY_ADDR = (*reg).eax as *mut usize;
        let ent = Entity::from_addr(CURRENT_CROSSHAIR_ENTITY_ADDR as usize);
        if CURRENT_CROSSHAIR_ENTITY_ADDR != null_mut() &&
            ent.team() != LOCAL_PLAYER.team() && // Enemy
            ent.health() > 0 // Alive
        {
            thread::spawn(move || {
                // Mouse button press
                input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
                let mouse_press = SendInput(&[input], mem::size_of::<INPUT>() as i32);
                if mouse_press == 0 {
                    let error_code = GetLastError();
                    println!("Mouse button press failed with error code: {:?}", error_code);
                }

                sleep(Duration::from_millis(100));

                // Mouse button release
                input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;
                let mouse_release = SendInput(&[input], mem::size_of::<INPUT>() as i32);
                if mouse_release == 0 {
                    let error_code = GetLastError();
                    println!("Mouse button release failed with error code: {:?}", error_code);
                }
                sleep(Duration::from_millis(TRIGGER_DELAY as u64));
            });
        }
    }
}



// Example of finding a pattern and setting up the hook
pub fn setup_trigger_bot() {
    unsafe {
        let trigger_bot = find_pattern("ac_client.exe",
                                       &[0x83, 0xC4, 0x10, 0x89, 0x44, 0x24, 0x10, 0x8B],
                                       "xxxxxxxx");

        if let Some(addr) = trigger_bot {
            println!("[triggerbot->setup_trigger_bot] trigger bot pattern found at: {:#x}", addr);
            let hooker = Hooker::new(
                addr,
                HookType::JmpBack(get_crosshair_entity),
                CallbackOption::None,
                0,
                HookFlags::empty(),
            );
            let hook_res = hooker.hook();

        match hook_res {
            Ok(trampoline_hook) => {
                *HOOK.lock().unwrap() = Some(trampoline_hook);
                println!("[triggerbot->setup_trigger_bot] trigger bot hook succeeded!");
            }
            Err(e) => {
                println!("[triggerbot->setup_trigger_bot] trigger bot hook failed: {:?}", e);
            }
        }


        } else {
            println!("[triggerbot->setup_trigger_bot] trigger bot pattern not found");
        }
    }
}