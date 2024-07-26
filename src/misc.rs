use std::ffi::c_void;

use crate::memorypatch::MemoryPatch;
use crate::offsets::offsets::{AMMO_CARBINE, AMMO_IN_MAGAZINE_CARBINE, AMMO_IN_MAGAZINE_PISTOL, AMMO_IN_MAGAZINE_RIFLE, AMMO_IN_MAGAZINE_SHOTGUN, AMMO_IN_MAGAZINE_SNIPER, AMMO_IN_MAGAZINE_SUBMACHINEGUN, AMMO_PISTOL, AMMO_RIFLE, AMMO_SHOTGUN, AMMO_SNIPER, AMMO_SUBMACHINEGUN, ARMOR_OFFSET_FROM_LOCAL_PLAYER, CARBINE_COOLDOWN, GRENADES_COUNT, HEALTH_OFFSET_FROM_LOCAL_PLAYER, KNIFE_COOLDOWN, PISTOL_COOLDOWN, RIFLE_COOLDOWN, SHOTGUN_COOLDOWN, SNIPER_COOLDOWN, SUBMACHINEGUN_COOLDOWN};

use crate::triggerbot::{get_crosshair_entity, setup_trigger_bot};
use crate::utils::find_pattern;
use crate::vars::game_vars::LOCAL_PLAYER;
use crate::vars::mem_patches::{MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH};
use crate::vars::ui_vars::{IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_NO_RELOAD};

pub unsafe fn init_mem_patches()
{
    let no_recoil = find_pattern("ac_client.exe",
                                 &[0x83, 0xEC, 0x28, 0x53, 0x55, 0x8B, 0x6C],
                                 "xxxxxxx");
    let mut no_recoil_res:usize = 0;
    if no_recoil.is_some() {
        no_recoil_res = no_recoil.unwrap();
        println!("[esp] no recoil pattern found at: {:#x}", no_recoil_res);
    }
    else {
        println!("[esp] no recoil pattern not found");
    }

    let rapid_fire = find_pattern("ac_client.exe",
                                  &[0x89, 0x08, 0x8B, 0x46, 0x14, 0xFF],
                                  "xxxxxx");
    let mut rapid_fire_res:usize = 0;
    if rapid_fire.is_some() {
        rapid_fire_res = rapid_fire.unwrap();
        println!("[esp] rapid fire pattern found at: {:#x}", rapid_fire_res);
    }
    else {
        println!("[esp] rapid fire pattern not found");
    }
    let map = find_pattern("ac_client.exe",
                                  &[0x75, 0x57, 0x85, 0xC9, 0x0F, 0x84, 0xA1, 0x00, 0x00, 0x00],
                                  "xxxxxxxxxx");
    let mut map_res:usize = 0;
    if map.is_some() {
        map_res = map.unwrap();
        println!("[esp] rapid fire pattern found at: {:#x}", map_res);
    }
    else {
        println!("[esp] rapid fire pattern not found");
    }
    //83 EC 28 -> C2 08 00
    NO_RECOIL_MEMORY_PATCH = MemoryPatch::new(
        &[0xC2, 0x08, 0x00], // return 0008
        0x03,
        no_recoil_res as *mut c_void,
        3usize).expect("Failed to patch No Recoil");
    //89 08 -> 90 90
    RAPID_FIRE_MEMORY_PATCH = MemoryPatch::new(
        &[0x90, 0x90],  // nop nop
        0x02,
        rapid_fire_res as *mut c_void,
        2usize).expect("Failed to patch Rapid Fire");
    MAPHACK_MEMORY_PATCH = MemoryPatch::new(
        &[0x90, 0x90],
        0x02,
        map_res as *mut c_void,
        10usize).expect("Failed to patch map");
    setup_trigger_bot();

}
pub unsafe fn player_fields_monitor()
{
    if IS_GRENADES_INFINITE
    {
        if *((LOCAL_PLAYER.entity_starts_at_addr + GRENADES_COUNT) as *mut i32) == 0
        {
            *((LOCAL_PLAYER.entity_starts_at_addr + GRENADES_COUNT) as *mut i32) = 1;
        }
    }
    if IS_NO_RELOAD
    {
        *((LOCAL_PLAYER.entity_starts_at_addr + KNIFE_COOLDOWN) as *mut f32) = 0.0f32;
        *((LOCAL_PLAYER.entity_starts_at_addr + PISTOL_COOLDOWN) as *mut f32) = 0.0f32;
        *((LOCAL_PLAYER.entity_starts_at_addr + CARBINE_COOLDOWN) as *mut f32) = 0.0f32;
        *((LOCAL_PLAYER.entity_starts_at_addr + SHOTGUN_COOLDOWN) as *mut f32) = 0.0f32;
        *((LOCAL_PLAYER.entity_starts_at_addr + SUBMACHINEGUN_COOLDOWN) as *mut f32) = 0.0f32;
        *((LOCAL_PLAYER.entity_starts_at_addr + SNIPER_COOLDOWN) as *mut f32) = 0.0f32;
        *((LOCAL_PLAYER.entity_starts_at_addr + RIFLE_COOLDOWN) as *mut f32) = 0.0f32;
    }
    if IS_INVULNERABLE
    {
        *((LOCAL_PLAYER.entity_starts_at_addr + HEALTH_OFFSET_FROM_LOCAL_PLAYER) as *mut i32) = 1337;
        *((LOCAL_PLAYER.entity_starts_at_addr + ARMOR_OFFSET_FROM_LOCAL_PLAYER) as *mut i32) = 1337;
    }
    if IS_INFINITE_AMMO
    {
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_RIFLE) as *mut i32) = 40;
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_IN_MAGAZINE_RIFLE) as *mut i32) = 40;

        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_PISTOL) as *mut i32) = 40;
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_IN_MAGAZINE_PISTOL) as *mut i32) = 40;

        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_CARBINE) as *mut i32) = 40;
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_IN_MAGAZINE_CARBINE) as *mut i32) = 40;

        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_SHOTGUN) as *mut i32) = 40;
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_IN_MAGAZINE_SHOTGUN) as *mut i32) = 40;

        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_SUBMACHINEGUN) as *mut i32) = 40;
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_IN_MAGAZINE_SUBMACHINEGUN) as *mut i32) = 40;

        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_SNIPER) as *mut i32) = 40;
        *((LOCAL_PLAYER.entity_starts_at_addr + AMMO_IN_MAGAZINE_SNIPER) as *mut i32) = 40;
    }
}