use std::sync::atomic::Ordering::SeqCst;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use crate::angle::Angle;
use crate::entity::Entity;
use crate::game::{set_brightness, set_brightness_toggle};
use crate::getclosestentity::get_closest_entity;
use crate::hotkey_widget::to_win_key;
use crate::offsets::offsets::{LOCAL_PLAYER_OFFSET, PITCH_OFFSET, YAW_OFFSET};
use crate::settings::AppSettings;
use crate::utils::{read_memory, write_memory};
use crate::vars::game_vars::{LOCAL_PLAYER, SMOOTH};
use crate::vars::handles::AC_CLIENT_EXE_HMODULE;
use crate::vars::mem_patches::{MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH};
use crate::vars::ui_vars::{IS_AIMBOT, IS_DRAW_FOV, IS_ESP, IS_FULLBRIGHT, IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_MAPHACK, IS_NO_RECOIL, IS_NO_RELOAD, IS_RAPID_FIRE, IS_SHOW_UI, IS_SMOOTH, IS_TRIGGERBOT, IS_WALLHACK};
use crate::vec_structures::Vec3;

pub struct KeyAction {
    pub(crate) key: usize, // The index of the key state in the key_states array
    pub(crate) action: unsafe fn(), // The function to call when the key is pressed
}

// Define the actions for each key
pub unsafe fn toggle_show_ui() {
    IS_SHOW_UI.store(!IS_SHOW_UI.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_wallhack() {
    IS_WALLHACK.store(!IS_WALLHACK.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_esp() {
    IS_ESP.store(!IS_ESP.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_infinite_nades() {
    IS_GRENADES_INFINITE.store(!IS_GRENADES_INFINITE.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_no_reload() {
    IS_NO_RELOAD.store(!IS_NO_RELOAD.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_invulnerability() {
    IS_INVULNERABLE.store(!IS_INVULNERABLE.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_infinite_ammo() {
    IS_INFINITE_AMMO.store(!IS_INFINITE_AMMO.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_no_recoil() {
    IS_NO_RECOIL.store(!IS_NO_RECOIL.load(SeqCst), SeqCst);
    if IS_NO_RECOIL.load(SeqCst) {
        NO_RECOIL_MEMORY_PATCH
            .patch_memory()
            .expect("[ui] Failed to patch memory no recoil");
    } else {
        NO_RECOIL_MEMORY_PATCH
            .unpatch_memory()
            .expect("[ui] Failed to unpatch memory no recoil");
    }
}

pub unsafe fn toggle_rapid_fire() {
    IS_RAPID_FIRE.store(!IS_RAPID_FIRE.load(SeqCst), SeqCst);
    if IS_RAPID_FIRE.load(SeqCst) {
        RAPID_FIRE_MEMORY_PATCH
            .patch_memory()
            .expect("[ui] Failed to patch memory rapid fire");
    } else {
        RAPID_FIRE_MEMORY_PATCH
            .unpatch_memory()
            .expect("[ui] Failed to unpatch memory rapid fire");
    }
}

pub unsafe fn toggle_aimbot() {
    IS_AIMBOT.store(!IS_AIMBOT.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_draw_fov() {
    IS_DRAW_FOV.store(!IS_DRAW_FOV.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_smooth() {
    IS_SMOOTH.store(!IS_SMOOTH.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_triggerbot() {
    IS_TRIGGERBOT.store(!IS_TRIGGERBOT.load(SeqCst), SeqCst);
}

pub unsafe fn toggle_maphack() {
    IS_MAPHACK.store(!IS_MAPHACK.load(SeqCst), SeqCst);
    if IS_MAPHACK.load(SeqCst) {
        MAPHACK_MEMORY_PATCH
            .patch_memory()
            .expect("[ui] Failed to patch memory Maphack");
    } else {
        MAPHACK_MEMORY_PATCH
            .unpatch_memory()
            .expect("[ui] Failed to unpatch memory Maphack");
    }
}

pub unsafe fn toggle_fullbright() {
    IS_FULLBRIGHT.store(!IS_FULLBRIGHT.load(SeqCst), SeqCst);
    let set_brightness_func = set_brightness();
    if !set_brightness_func.is_null() {
        set_brightness_toggle(IS_FULLBRIGHT.load(SeqCst));
    } else {
        println!("Function pointer to set_brightness is null!");
    }
}

pub unsafe fn aimbot() { //app_settings: &AppSettings
    unsafe {
        if !IS_AIMBOT.load(SeqCst) {
            return;
        }

        let local_player_addr =
            match read_memory::<usize>(AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) {
                Ok(addr) => addr,
                Err(err) => {
                    println!("Error reading local player address: {}", err);
                    return;
                }
            };

        LOCAL_PLAYER = Entity::from_addr(local_player_addr);

/*        if GetAsyncKeyState(to_win_key(app_settings.aim_key.as_ref().unwrap().key).0 as i32) & 1
            == 1*/
        {
            let enemy = get_closest_entity();
            if LOCAL_PLAYER.entity_starts_at_addr == 0 || enemy.entity_starts_at_addr == 0 {
                return; // Didn't find player or enemy
            }

            // Handle health and team checks
            if enemy.health().unwrap_or(0) < 0 || enemy.team() == LOCAL_PLAYER.team() {
                return; // Skipping dead or ally
            }

            // Safely read player and enemy positions
            let player_head_pos = match LOCAL_PLAYER.head_position() {
                Ok(pos) => pos,
                Err(err) => {
                    println!("Error reading player head position: {}", err);
                    return;
                }
            };

            let enemy_head_pos = match enemy.head_position() {
                Ok(pos) => pos,
                Err(err) => {
                    println!("Error reading enemy head position: {}", err);
                    return;
                }
            };

            // Calculate angle
            let angle = Angle::get_angle(
                Vec3::new(player_head_pos.x, player_head_pos.y, player_head_pos.z),
                Vec3::new(enemy_head_pos.x, enemy_head_pos.y, enemy_head_pos.z),
            );

            // Safely read and update yaw and pitch
            let update_view_angle = |offset: usize, value: f32| {
                let address = LOCAL_PLAYER.entity_starts_at_addr + offset;
                match read_memory::<f32>(address) {
                    Ok(current_value) => {
                        // Only write if the value is different to avoid unnecessary writes
                        if current_value != value {
                            if let Err(err) = write_memory::<f32>(address, value) {
                                println!(
                                    "Error writing to address {:x} storing value {}: {}",
                                    address, current_value, err
                                );
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error reading from address {:x}: {}", address, err);
                    }
                }
            };

            // Read yaw and pitch with error handling
            let (local_player_yaw, local_player_pitch) =
                match (LOCAL_PLAYER.yaw(), LOCAL_PLAYER.pitch()) {
                    (Ok(y), Ok(p)) => (y as f32, p as f32),
                    (Err(err), _) => {
                        println!("Error reading yaw: {}", err);
                        return;
                    }
                    (_, Err(err)) => {
                        println!("Error reading pitch: {}", err);
                        return;
                    }
                };

            // Calculate the angle difference
            let angle_diff_yaw = angle.yaw - local_player_yaw;
            let angle_diff_pitch = angle.pitch - local_player_pitch;

            let smooth = Angle::new(angle_diff_yaw, angle_diff_pitch);
            let smooth_value = SMOOTH.load(SeqCst) as f32;

            if IS_SMOOTH.load(SeqCst) {
                if smooth_value > 0.0 {
                    let new_yaw = local_player_yaw + (smooth.yaw / smooth_value);
                    let new_pitch = local_player_pitch + (smooth.pitch / smooth_value);
                    update_view_angle(YAW_OFFSET, new_yaw);
                    update_view_angle(PITCH_OFFSET, new_pitch);
                } else {
                    return;
                }
            } else {
                update_view_angle(YAW_OFFSET, angle.yaw);
                update_view_angle(PITCH_OFFSET, angle.pitch);
            }
        }
    }
}