use windows::Win32::Foundation::COLORREF;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

use crate::angle::Angle;
use crate::draw_utils;
use crate::entity::Entity;
use crate::getclosestentity::get_closest_entity;
use crate::offsets::offsets::{ENTITY_LIST_OFFSET, LOCAL_PLAYER_OFFSET, NUMBER_OF_PLAYERS_IN_MATCH_OFFSET, PITCH_OFFSET, VIEW_MATRIX_ADDR, YAW_OFFSET};
use crate::vars::game_vars::{ENTITY_LIST_PTR, FOV, LOCAL_PLAYER, NUM_PLAYERS_IN_MATCH, SMOOTH, VIEW_MATRIX};
use crate::vars::handles::{AC_CLIENT_EXE_HMODULE, GAME_WINDOW_DIMENSIONS};
use crate::vars::hotkeys::AIM_KEY;
use crate::vars::ui_vars::{IS_AIMBOT, IS_DRAW_FOV, IS_SMOOTH};
use crate::vec_structures::Vec3;

pub unsafe fn aimbot()
{
    println!("aimbot function start");
    if !IS_AIMBOT
    {
        return;
    }
    LOCAL_PLAYER = Entity::from_addr(*((AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) as *mut usize));
    VIEW_MATRIX = VIEW_MATRIX_ADDR as *mut [f32; 16];
    NUM_PLAYERS_IN_MATCH = *((AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET) as *const i32) as usize;
    ENTITY_LIST_PTR = *((AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) as *const u32);
    if GetAsyncKeyState(AIM_KEY.0 as i32) & 1 == 1 {
        println!("AIM_KEY has been pressed!");
        let enemy = get_closest_entity();
        if LOCAL_PLAYER.entity_starts_at_addr == 0 || enemy.entity_starts_at_addr == 0
        {
            return; //Didn't find player or enemy
        }
        if enemy.health() < 0 || enemy.team() == LOCAL_PLAYER.team()
        {
            return; //Skipping dead or ally
        }
        let angle = Angle::get_angle(
            Vec3::new(
                LOCAL_PLAYER.head_position().x,
                LOCAL_PLAYER.head_position().y,
                LOCAL_PLAYER.head_position().z),
            Vec3::new(enemy.head_position().x,
                      enemy.head_position().y,
                      enemy.head_position().z)
        );
        let smooth = Angle::new(
            angle.yaw - *((LOCAL_PLAYER.entity_starts_at_addr + YAW_OFFSET) as *const f32),
            angle.pitch - *((LOCAL_PLAYER.entity_starts_at_addr + PITCH_OFFSET) as *const f32)
        );
        if IS_SMOOTH
        {
            *((LOCAL_PLAYER.entity_starts_at_addr + YAW_OFFSET) as *mut f32) += smooth.yaw / SMOOTH;
            *((LOCAL_PLAYER.entity_starts_at_addr + PITCH_OFFSET) as *mut f32) += smooth.pitch / SMOOTH;
        }
        else
        {
            *((LOCAL_PLAYER.entity_starts_at_addr + YAW_OFFSET) as *mut f32) = angle.yaw;
            *((LOCAL_PLAYER.entity_starts_at_addr + PITCH_OFFSET) as *mut f32) = angle.pitch;
        }
    }
    println!("aimbot function end");

}