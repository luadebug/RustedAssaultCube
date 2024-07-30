use std::thread;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering::SeqCst;
use std::thread::sleep;
use std::time::Duration;

use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookType, Registers};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_MOUSE, MOUSE_EVENT_FLAGS, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SendInput};

use crate::entity::Entity;
use crate::pattern_mask::PatternMask;
use crate::utils::find_pattern;
use crate::vars::game_vars::{CURRENT_CROSSHAIR_ENTITY_ADDR, LOCAL_PLAYER, TRIGGER_DELAY};
use crate::vars::hooks::TRIGGERBOT_HOOK;
use crate::vars::ui_vars::IS_TRIGGERBOT;

// Define a constant for the pattern mask
const TRIGGER_BOT_PATTERN_MASK: &str = "83 ? ? 89 ? ? ? 8B ? 83 3D";

// The function to be hooked
#[inline(never)]
pub(crate) unsafe extern "cdecl" fn get_crosshair_entity(
    reg: *mut Registers,
    _: usize
) {
    unsafe {
        if IS_TRIGGERBOT.load(SeqCst) {
            if let Some(reg_val) = reg.as_ref() {
                if reg_val.eax == 0
                {
                    return;
                }
                CURRENT_CROSSHAIR_ENTITY_ADDR = reg_val.eax as *mut usize;
                if CURRENT_CROSSHAIR_ENTITY_ADDR == null_mut() {
                    return;
                }

                // Get local player's team, handling potential errors
                let local_player_team = match LOCAL_PLAYER.team() {
                    Ok(team) => team,
                    Err(err) => {
                        eprintln!("Error reading local player team: {}", err);
                        return;
                    }
                };

                // Safely check entity conditions using match expressions
                match (CURRENT_CROSSHAIR_ENTITY_ADDR as usize, Entity::from_addr(CURRENT_CROSSHAIR_ENTITY_ADDR as usize)) {
                    (addr, ent) if addr != 0 => {
                        match (ent.team(), ent.health()) {
                            (Ok(team), Ok(health)) if team != local_player_team && health > 0 => {
                                trigger_bot();
                            }
                            (Err(err), _) | (_, Err(err)) => {
                                eprintln!("Error reading entity data: {}", err);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
unsafe fn trigger_bot() {
    let mut input: INPUT = INPUT::default();
    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dx = 0;
    input.Anonymous.mi.dy = 0;
    input.Anonymous.mi.mouseData = 0;
    input.Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0);
    input.Anonymous.mi.time = 0;
    input.Anonymous.mi.dwExtraInfo = 0;

    // Use a mutex to synchronize access to the input variable
    let input_mutex = Arc::new(Mutex::new(input));


    thread::spawn(move || unsafe {
        // Mouse button press
        {
            let mut input = input_mutex.lock().unwrap();
            input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
            send_mouse_input(&input);
        }

        sleep(Duration::from_millis(100));

        // Mouse button release
        {
            let mut input = input_mutex.lock().unwrap();
            input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;
            send_mouse_input(&input);
        }

        sleep(Duration::from_millis(TRIGGER_DELAY.load(SeqCst) as u64));
    });
}

// Helper function to send mouse input and handle errors
fn send_mouse_input(input: &INPUT) {
    let result = unsafe { SendInput(&[*input], size_of::<INPUT>() as i32) };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        eprintln!("Mouse input failed with error code: {:?}", error_code);
    }
}

// Example of finding a pattern and setting up the hook
pub fn setup_trigger_bot() {
    thread::spawn(|| {
        let pattern_mask = PatternMask::aob_to_pattern_mask(TRIGGER_BOT_PATTERN_MASK);
        println!("[TriggerBotHook] {:#x}", &pattern_mask);

        let trigger_bot_aob = find_pattern("ac_client.exe", &*pattern_mask.aob_pattern, &pattern_mask.mask_to_string());

        match trigger_bot_aob {
            Some(addr) => unsafe {
                println!("[triggerbot_hook.rs->setup_trigger_bot] trigger bot pattern found at: {:#x}", addr);
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
                        *TRIGGERBOT_HOOK.lock().unwrap() = Some(trampoline_hook);
                        println!("[triggerbot_hook.rs->setup_trigger_bot] trigger bot hook succeeded!");
                    }
                    Err(e) => {
                        eprintln!("[triggerbot_hook.rs->setup_trigger_bot] trigger bot hook failed: {:?}", e);
                    }
                }
            }
            None => {
                println!("[triggerbot_hook.rs->setup_trigger_bot] trigger bot pattern not found");
            }
        }
    });
}