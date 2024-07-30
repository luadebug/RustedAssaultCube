use std::ffi::c_void;
use std::thread;
use crate::get_local_player_hook::setup_invul;
use crate::memorypatch::MemoryPatch;
use crate::pattern_mask::PatternMask;
use crate::triggerbot_hook::setup_trigger_bot;
use crate::utils::find_pattern;
use crate::vars::mem_patches::{MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH};

pub unsafe fn init_mem_patches()
{
    thread::spawn(||
    {
        let pattern_mask = PatternMask::aob_to_pattern_mask(
            "83 ? ? 53 55 8B ? ? ? 56 57 8B ? 8B"
        );


        println!("[No Recoil] {:#x}", &pattern_mask);

        let no_recoil_aob = find_pattern("ac_client.exe",
                                         &*pattern_mask.aob_pattern,
                                         &pattern_mask.mask_to_string());
        let mut no_recoil_res: usize = 0;
        if no_recoil_aob.is_some() {
            no_recoil_res = no_recoil_aob.unwrap();
            println!("[esp] no recoil pattern found at: {:#x}", no_recoil_res);
        } else {
            println!("[esp] no recoil pattern not found");
        }

        //83 EC 28 -> C2 08 00
        unsafe {
        NO_RECOIL_MEMORY_PATCH = MemoryPatch::new(
            &[0xC2, 0x08, 0x00], // return 0008
            0x03,
            no_recoil_res as *mut c_void,
            3usize).expect("Failed to patch No Recoil");
        }
    });

    thread::spawn(||
    {
        let pattern_mask = PatternMask::aob_to_pattern_mask(
            "89 ? 8B ? ? FF ? 8D ? ? ? 50 51 8B"
        );


        println!("[Rapid Fire] {:#x}", &pattern_mask);

        let rapid_fire_aob = find_pattern("ac_client.exe",
                                          &*pattern_mask.aob_pattern,
                                          &pattern_mask.mask_to_string());
        let mut rapid_fire_res:usize = 0;
        if rapid_fire_aob.is_some() {
            rapid_fire_res = rapid_fire_aob.unwrap();
            println!("[esp] rapid fire pattern found at: {:#x}", rapid_fire_res);
        }
        else {
            println!("[esp] rapid fire pattern not found");
        }

        //89 08 -> 90 90
        unsafe {
        RAPID_FIRE_MEMORY_PATCH = MemoryPatch::new(
            &[0x90, 0x90],  // nop nop
            0x02,
            rapid_fire_res as *mut c_void,
            2usize).expect("Failed to patch Rapid Fire");
        }
    });
    thread::spawn(||
    {
        let pattern_mask = PatternMask::aob_to_pattern_mask(
            "75 57 85 C9 ? ? ? ? ? ? 83 F9 04"
        );


        println!("[MapHack] {:#x}", &pattern_mask);

        let maphack_aob = find_pattern("ac_client.exe",
                                       &*pattern_mask.aob_pattern,
                                       &pattern_mask.mask_to_string());

        let mut map_res:usize = 0;
        if maphack_aob.is_some() {
            map_res = maphack_aob.unwrap();
            println!("[esp] maphack pattern found at: {:#x}", map_res);
        }
        else {
            println!("[esp] maphack pattern not found");
        }
        unsafe {
        MAPHACK_MEMORY_PATCH = MemoryPatch::new(
            &[0x90, 0x90],
            0x02,
            map_res as *mut c_void,
            2usize).expect("Failed to patch map");
        }
    });
    setup_trigger_bot();
    setup_invul();

}
