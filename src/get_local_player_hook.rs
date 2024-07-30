use std::ptr::null_mut;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookType, Registers};

use crate::entity::Entity;
use crate::offsets::offsets::{AMMO_CARBINE, AMMO_IN_MAGAZINE_CARBINE, AMMO_IN_MAGAZINE_PISTOL, AMMO_IN_MAGAZINE_RIFLE, AMMO_IN_MAGAZINE_SHOTGUN, AMMO_IN_MAGAZINE_SNIPER, AMMO_IN_MAGAZINE_SUBMACHINEGUN, AMMO_PISTOL, AMMO_RIFLE, AMMO_SHOTGUN, AMMO_SNIPER, AMMO_SUBMACHINEGUN, ARMOR_OFFSET_FROM_LOCAL_PLAYER, CARBINE_COOLDOWN, GRENADES_COUNT, HEALTH_OFFSET_FROM_LOCAL_PLAYER, KNIFE_COOLDOWN, PISTOL_COOLDOWN, RIFLE_COOLDOWN, SHOTGUN_COOLDOWN, SNIPER_COOLDOWN, SUBMACHINEGUN_COOLDOWN};
use crate::pattern_mask::PatternMask;
use crate::utils::find_pattern;
use crate::vars::hooks::LOCAL_PLAYER_HOOK;
use crate::vars::ui_vars::{IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_NO_RELOAD};

static mut LOCAL_PLAYER_FIELDS_ADDR: * mut usize = null_mut();

// The function to be hooked
#[inline(never)]
pub(crate) unsafe extern "cdecl" fn get_local_player_health(
    reg: *mut Registers,
    _: usize
) {
    unsafe {
        if let Some(reg_val) = reg.as_ref() {
            if reg_val.ebx == 0
            {
                return;
            }
            if LOCAL_PLAYER_FIELDS_ADDR == null_mut() {
                LOCAL_PLAYER_FIELDS_ADDR = reg_val.ebx as *mut usize;
                if LOCAL_PLAYER_FIELDS_ADDR == null_mut() { return; }
            }

            let mut local_player = match Entity::from_addr(LOCAL_PLAYER_FIELDS_ADDR as usize) {
                ent if ent.entity_starts_at_addr != 0 => ent,
                _ => return,
            };

            if IS_GRENADES_INFINITE.load(SeqCst) {
                if let Ok(current_grenades) = local_player.read_value::<i32>(GRENADES_COUNT) {
                    if current_grenades == 0 {
                        local_player.write_value(GRENADES_COUNT, 1).ok();
                    }
                } else {
                    eprintln!("Error reading grenades count");
                }
            }

            if IS_NO_RELOAD.load(SeqCst) {
                local_player.write_value(KNIFE_COOLDOWN, 0.0f32).ok();
                local_player.write_value(PISTOL_COOLDOWN, 0.0f32).ok();
                local_player.write_value(CARBINE_COOLDOWN, 0.0f32).ok();
                local_player.write_value(SHOTGUN_COOLDOWN, 0.0f32).ok();
                local_player.write_value(SUBMACHINEGUN_COOLDOWN, 0.0f32).ok();
                local_player.write_value(SNIPER_COOLDOWN, 0.0f32).ok();
                local_player.write_value(RIFLE_COOLDOWN, 0.0f32).ok();
            }

            if IS_INVULNERABLE.load(SeqCst) {
                local_player.write_value(HEALTH_OFFSET_FROM_LOCAL_PLAYER, 1337).ok();
                local_player.write_value(ARMOR_OFFSET_FROM_LOCAL_PLAYER, 1337).ok();
            }

            if IS_INFINITE_AMMO.load(SeqCst) {
                local_player.write_value(AMMO_RIFLE, 40).ok();
                local_player.write_value(AMMO_IN_MAGAZINE_RIFLE, 40).ok();

                local_player.write_value(AMMO_PISTOL, 40).ok();
                local_player.write_value(AMMO_IN_MAGAZINE_PISTOL, 40).ok();

                local_player.write_value(AMMO_CARBINE, 40).ok();
                local_player.write_value(AMMO_IN_MAGAZINE_CARBINE, 40).ok();

                local_player.write_value(AMMO_SHOTGUN, 40).ok();
                local_player.write_value(AMMO_IN_MAGAZINE_SHOTGUN, 40).ok();

                local_player.write_value(AMMO_SUBMACHINEGUN, 40).ok();
                local_player.write_value(AMMO_IN_MAGAZINE_SUBMACHINEGUN, 40).ok();

                local_player.write_value(AMMO_SNIPER, 40).ok();
                local_player.write_value(AMMO_IN_MAGAZINE_SNIPER, 40).ok();
            }
        }
    }
}
// Example of finding a pattern and setting up the hook
pub fn setup_invul() {
    thread::spawn(move || {

        let pattern_mask = PatternMask::aob_to_pattern_mask(
            "8B ? ? ? ? ? 83 F9 19"
        );

        println!("[GetLocalPlayerHealthHook] {:#x}", &pattern_mask);

        let get_local_player_health_aob = find_pattern("ac_client.exe",
                                                       &*pattern_mask.aob_pattern,
                                                       &pattern_mask.mask_to_string());

        match get_local_player_health_aob {
            Some(addr) => unsafe {
                println!("[get_local_player_hook.rs->setup_invul] local player get current hp pattern found at: {:#x}", addr);
                let hooker = Hooker::new(
                    addr,
                    HookType::JmpBack(get_local_player_health),
                    CallbackOption::None,
                    0,
                    HookFlags::empty(),
                );
                let hook_res = hooker.hook();

                match hook_res {
                    Ok(trampoline_hook) => {
                        *LOCAL_PLAYER_HOOK.lock().unwrap() = Some(trampoline_hook);
                        println!("[get_local_player_hook.rs->setup_invul] local player get current hp pattern hook succeeded!");
                    }
                    Err(e) => {
                        eprintln!("[get_local_player_hook.rs->setup_invul] local player get current hp pattern hook failed: {:?}", e);
                    }
                }
            }
            None => {
                println!("[get_local_player_hook.rs->setup_invul] local player get current hp pattern not found");
            }
        }
    });
}