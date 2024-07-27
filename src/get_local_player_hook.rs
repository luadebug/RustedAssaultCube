
use std::ptr::null_mut;
use std::thread;
use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookType, Registers};

use crate::entity::Entity;
use crate::offsets::offsets::{AMMO_CARBINE, AMMO_IN_MAGAZINE_CARBINE, AMMO_IN_MAGAZINE_PISTOL, AMMO_IN_MAGAZINE_RIFLE, AMMO_IN_MAGAZINE_SHOTGUN, AMMO_IN_MAGAZINE_SNIPER, AMMO_IN_MAGAZINE_SUBMACHINEGUN, AMMO_PISTOL, AMMO_RIFLE, AMMO_SHOTGUN, AMMO_SNIPER, AMMO_SUBMACHINEGUN, ARMOR_OFFSET_FROM_LOCAL_PLAYER, CARBINE_COOLDOWN, GRENADES_COUNT, HEALTH_OFFSET_FROM_LOCAL_PLAYER, KNIFE_COOLDOWN, PISTOL_COOLDOWN, RIFLE_COOLDOWN, SHOTGUN_COOLDOWN, SNIPER_COOLDOWN, SUBMACHINEGUN_COOLDOWN};
use crate::pattern_mask::PatternMask;
use crate::utils::find_pattern;
use crate::vars::hooks::{LOCAL_PLAYER_HOOK};
use crate::vars::ui_vars::{IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_NO_RELOAD};

pub static mut LOCAL_PLAYER_FIELDS_ADDR: * mut usize = null_mut();

// The function to be hooked
#[inline(never)]
pub(crate) unsafe extern "cdecl" fn get_local_player_health(
    reg: *mut Registers,
    _: usize
) {
    LOCAL_PLAYER_FIELDS_ADDR = (*reg).ebx as *mut usize;
    if LOCAL_PLAYER_FIELDS_ADDR != null_mut()
    {
        let local_player_ent = Entity::from_addr(LOCAL_PLAYER_FIELDS_ADDR as usize);
        if IS_GRENADES_INFINITE
        {
            if *((local_player_ent.entity_starts_at_addr + GRENADES_COUNT) as *mut i32) == 0
            {
                *((local_player_ent.entity_starts_at_addr + GRENADES_COUNT) as *mut i32) = 1;
            }
        }
        if IS_NO_RELOAD
        {
            *((local_player_ent.entity_starts_at_addr + KNIFE_COOLDOWN) as *mut f32) = 0.0f32;
            *((local_player_ent.entity_starts_at_addr + PISTOL_COOLDOWN) as *mut f32) = 0.0f32;
            *((local_player_ent.entity_starts_at_addr + CARBINE_COOLDOWN) as *mut f32) = 0.0f32;
            *((local_player_ent.entity_starts_at_addr + SHOTGUN_COOLDOWN) as *mut f32) = 0.0f32;
            *((local_player_ent.entity_starts_at_addr + SUBMACHINEGUN_COOLDOWN) as *mut f32) = 0.0f32;
            *((local_player_ent.entity_starts_at_addr + SNIPER_COOLDOWN) as *mut f32) = 0.0f32;
            *((local_player_ent.entity_starts_at_addr + RIFLE_COOLDOWN) as *mut f32) = 0.0f32;
        }
        if IS_INVULNERABLE
        {
            *((local_player_ent.entity_starts_at_addr + HEALTH_OFFSET_FROM_LOCAL_PLAYER) as *mut i32) = 1337;
            *((local_player_ent.entity_starts_at_addr + ARMOR_OFFSET_FROM_LOCAL_PLAYER) as *mut i32) = 1337;
        }
        if IS_INFINITE_AMMO
        {
            *((local_player_ent.entity_starts_at_addr + AMMO_RIFLE) as *mut i32) = 40;
            *((local_player_ent.entity_starts_at_addr + AMMO_IN_MAGAZINE_RIFLE) as *mut i32) = 40;

            *((local_player_ent.entity_starts_at_addr + AMMO_PISTOL) as *mut i32) = 40;
            *((local_player_ent.entity_starts_at_addr + AMMO_IN_MAGAZINE_PISTOL) as *mut i32) = 40;

            *((local_player_ent.entity_starts_at_addr + AMMO_CARBINE) as *mut i32) = 40;
            *((local_player_ent.entity_starts_at_addr + AMMO_IN_MAGAZINE_CARBINE) as *mut i32) = 40;

            *((local_player_ent.entity_starts_at_addr + AMMO_SHOTGUN) as *mut i32) = 40;
            *((local_player_ent.entity_starts_at_addr + AMMO_IN_MAGAZINE_SHOTGUN) as *mut i32) = 40;

            *((local_player_ent.entity_starts_at_addr + AMMO_SUBMACHINEGUN) as *mut i32) = 40;
            *((local_player_ent.entity_starts_at_addr + AMMO_IN_MAGAZINE_SUBMACHINEGUN) as *mut i32) = 40;

            *((local_player_ent.entity_starts_at_addr + AMMO_SNIPER) as *mut i32) = 40;
            *((local_player_ent.entity_starts_at_addr + AMMO_IN_MAGAZINE_SNIPER) as *mut i32) = 40;
        }
    }
}




// Example of finding a pattern and setting up the hook
pub fn setup_invul() {
    unsafe {
        thread::spawn(||
        {
            /*8B 8B EC 00 00 00 83 F9 19*/
            /*8B ? ? ? ? ? 83 F9 19*/
            /*x?????xxx*/
            let pattern_mask = PatternMask::aob_to_pattern_mask(
                "8B ? ? ? ? ? 83 F9 19"
            );

            println!("[GetLocalPlayerHealthHook] {:#x}", &pattern_mask);

            let get_local_player_health_aob = find_pattern("ac_client.exe",
                                                           &*pattern_mask.aob_pattern,
                                                           &pattern_mask.mask_to_string());
/*                                                           &[0x8B, 0x8B, 0xEC, 0x00, 0x00, 0x00, 0x83, 0xF9, 0x19],
                                                           "x?????xxx");*/

            if let Some(addr) = get_local_player_health_aob {
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
                        println!("[get_local_player_hook.rs->setup_invul] local player get current hp pattern hook failed: {:?}", e);
                    }
                }
            } else {
                println!("[get_local_player_hook.rs->setup_invul] local player get current hp pattern not found");
            }
        });
    }
}