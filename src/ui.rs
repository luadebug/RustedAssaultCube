use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::addr_of_mut;

use fermium::mouse::{SDL_SetRelativeMouseMode, SDL_ShowCursor};
use fermium::prelude::SDL_bool;
use fermium::video::{SDL_CreateWindow, SDL_SetWindowGrab, SDL_WINDOW_OPENGL, SDL_WINDOW_RESIZABLE, SDL_WINDOWPOS_UNDEFINED};
use hudhook::{imgui, RenderContext};
use hudhook::imgui::{Context, internal::RawCast, sys::{ImFontAtlas_AddFontFromFileTTF, ImFontAtlas_GetGlyphRangesChineseFull}};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE, VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_INSERT};

use crate::esp::esp_entrypoint;
use crate::style::set_dark_style;
use crate::vars::game_vars::{FOV, SMOOTH};
use crate::vars::handles::GAME_WINDOW_DIMENSIONS;
use crate::vars::mem_patches::{NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH};
use crate::vars::ui_vars::{IS_DRAW_FOV, IS_ESP, IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_NO_RECOIL, IS_NO_RELOAD, IS_RAPID_FIRE, IS_SHOW_UI, IS_SMOOTH};

pub static mut CANRELATIVEMOUSE:bool = true;
pub static mut RELATIVEMOUSE:bool = false;


pub unsafe fn on_frame(ui: &imgui::Ui) {
    ui.text("Hello from `hudhook`!");
    if ui.checkbox("[Delete] ESP", &mut *addr_of_mut!(IS_ESP)) {
        IS_ESP = !IS_ESP;
        if IS_ESP {
            esp_entrypoint().expect("[ui] Failed to call esp_entrypoint()");
        }
    }
    if ui.checkbox("[F1] Infinite Grenades", &mut *addr_of_mut!(IS_GRENADES_INFINITE)) {
        IS_GRENADES_INFINITE = !IS_GRENADES_INFINITE;
    }
    if ui.checkbox("[F2] No Reload", &mut *addr_of_mut!(IS_NO_RELOAD)) {
        IS_NO_RELOAD = !IS_NO_RELOAD;
    }
    if ui.checkbox("[F3] Invulnerability", &mut *addr_of_mut!(IS_INVULNERABLE)) {
        IS_INVULNERABLE = !IS_INVULNERABLE;
    }
    if ui.checkbox("[F4] Infinite Ammo", &mut *addr_of_mut!(IS_INFINITE_AMMO)) {
        IS_INFINITE_AMMO = !IS_INFINITE_AMMO;
    }
    if ui.checkbox("[F5] No Recoil", &mut *addr_of_mut!(IS_NO_RECOIL)) {
        IS_NO_RECOIL = !IS_NO_RECOIL;
        if IS_NO_RECOIL {
            NO_RECOIL_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory no recoil");
        }
        else {
            NO_RECOIL_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory no recoil");
        }
    }
    if ui.checkbox("[F6] Rapid Fire", &mut *addr_of_mut!(IS_RAPID_FIRE)) {
        IS_RAPID_FIRE = !IS_RAPID_FIRE;
        if IS_RAPID_FIRE {
            RAPID_FIRE_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory rapid fire");
        }
        else {
            RAPID_FIRE_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory rapid fire");
        }
    }
    if ui.checkbox("[F7] Aimbot Draw FOV", &mut *addr_of_mut!(IS_DRAW_FOV)) {
        IS_DRAW_FOV = !IS_DRAW_FOV;
    }
    if ui.slider("FOV", 1.0, 300.0, &mut *addr_of_mut!(FOV)){
        println!("Set Aimbot FOV to {}", FOV);
    }
    if ui.checkbox("[F8] Aimbot Smooth", &mut *addr_of_mut!(IS_SMOOTH)) {
        IS_SMOOTH = !IS_SMOOTH;
    }
    if ui.slider("Smooth", 1.0, 100.0, &mut *addr_of_mut!(SMOOTH)){
        println!("Set Aimbot Smooth to {}", SMOOTH);
    }
}



pub struct RenderLoop;

impl RenderLoop {
    pub fn new() -> Self {
        println!("Initializing");
        Self
    }
}


impl Default for RenderLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(&'a mut self, _ctx: &mut Context, render_context: &'a mut dyn RenderContext) {
        unsafe { inputgrab(IS_SHOW_UI); }
/*        unsafe
        {
            SDL_ShowCursor(c_int::from(!IS_SHOW_UI));
            if SDL_SetRelativeMouseMode(SDL_bool::new(!IS_SHOW_UI)) == -1 {
                println!("[ui->initialize] Failed to set relative mouse mode");
            }
        }*/
        _ctx.set_ini_filename(None);

        unsafe {
            ImFontAtlas_AddFontFromFileTTF(
                _ctx.fonts().raw_mut(),
                "C:\\windows\\fonts\\calibri.ttf\0".as_ptr().cast(),
                26.0,
                std::ptr::null(),
                ImFontAtlas_GetGlyphRangesChineseFull(_ctx.fonts().raw_mut()),
            )
        };
        set_dark_style(_ctx);
        _ctx.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_KEYBOARD;
        _ctx.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_SET_MOUSE_POS;
        _ctx.io_mut().backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;
        _ctx.io_mut().backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        _ctx.set_log_filename(PathBuf::from("imgui_log.txt"));
        unsafe
        {
            _ctx.io_mut().want_set_mouse_pos = IS_SHOW_UI;
            _ctx.io_mut().want_capture_mouse = IS_SHOW_UI;
        }
    }

    fn before_render<'a>(
        &'a mut self,
        _ctx: &mut Context,
        render_context: &'a mut dyn RenderContext,
    ) {
        unsafe {
            if GetAsyncKeyState(VK_INSERT.0 as i32) & 1 == 1 {
                IS_SHOW_UI = !IS_SHOW_UI;
            }
            if GetAsyncKeyState(VK_DELETE.0 as i32) & 1 == 1 {
                IS_ESP = !IS_ESP;
            }
            if GetAsyncKeyState(VK_F1.0 as i32) & 1 == 1 {
                IS_GRENADES_INFINITE = !IS_GRENADES_INFINITE;
            }
            if GetAsyncKeyState(VK_F2.0 as i32) & 1 == 1 {
                IS_NO_RELOAD = !IS_NO_RELOAD;
            }
            if GetAsyncKeyState(VK_F3.0 as i32) & 1 == 1 {
                IS_INVULNERABLE = !IS_INVULNERABLE;
            }
            if GetAsyncKeyState(VK_F4.0 as i32) & 1 == 1 {
                IS_INFINITE_AMMO = !IS_INFINITE_AMMO;
            }
            if GetAsyncKeyState(VK_F5.0 as i32) & 1 == 1 {
                IS_NO_RECOIL = !IS_NO_RECOIL;
                if IS_NO_RECOIL {
                    NO_RECOIL_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory no recoil");
                }
                else {
                    NO_RECOIL_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory no recoil");
                }
            }
            if GetAsyncKeyState(VK_F6.0 as i32) & 1 == 1 {
                IS_RAPID_FIRE = !IS_RAPID_FIRE;
                if IS_RAPID_FIRE {
                    RAPID_FIRE_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory rapid fire");
                }
                else {
                    RAPID_FIRE_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory rapid fire");
                }
            }
            if GetAsyncKeyState(VK_F7.0 as i32) & 1 == 1
            {
                IS_DRAW_FOV = !IS_DRAW_FOV;
            }
            if GetAsyncKeyState(VK_F8.0 as i32) & 1 == 1
            {
                IS_SMOOTH = !IS_SMOOTH;
            }
            _ctx.io_mut().mouse_draw_cursor = IS_SHOW_UI;
            inputgrab(IS_SHOW_UI);
/*            SDL_ShowCursor(c_int::from(!IS_SHOW_UI));
            if SDL_SetRelativeMouseMode(SDL_bool::new(!IS_SHOW_UI)) == -1 {
                println!("[ui->before_render] Failed to set relative mouse mode");
            }*/
            _ctx.io_mut().want_set_mouse_pos = IS_SHOW_UI;
            _ctx.io_mut().want_capture_mouse = IS_SHOW_UI;
            return;
        }
    }

    fn render(&mut self, ui: &mut imgui::Ui) {
        unsafe {
            if !IS_SHOW_UI {
                return;
            }
            inputgrab(IS_SHOW_UI);
/*            SDL_ShowCursor(c_int::from(!IS_SHOW_UI));
            if SDL_SetRelativeMouseMode(SDL_bool::new(!IS_SHOW_UI)) == -1 {
                println!("[ui->render] Failed to set relative mouse mode");
            }*/
            ui.window("[Insert] Menu")
                .title_bar(true)
                .size([1000.0, 700.0], imgui::Condition::FirstUseEver)
                .position([300.0, 300.0], imgui::Condition::FirstUseEver)
                .build(||
                {
                    on_frame(ui);
                });
        }
    }
}

pub unsafe fn inputgrab(on:bool)
{
    let windowname: CString = CString::new("AssaultCube").unwrap();
    let screen = SDL_CreateWindow(windowname.as_ptr(),
                                  SDL_WINDOWPOS_UNDEFINED,
                                  SDL_WINDOWPOS_UNDEFINED,
                                  GAME_WINDOW_DIMENSIONS.width,
                                  GAME_WINDOW_DIMENSIONS.height,
                                  SDL_WINDOW_OPENGL.0 |  SDL_WINDOW_RESIZABLE.0);

    if on
    {
        SDL_ShowCursor(0);
        if CANRELATIVEMOUSE
        {
            if SDL_SetRelativeMouseMode(SDL_bool::new(true)) >= 0 {
                println!("[ui->inputgrab] Success to set relative mouse mode");
                SDL_SetWindowGrab(screen, SDL_bool::new(false));
                CANRELATIVEMOUSE = false;
                RELATIVEMOUSE = false;

            }
            else {
                println!("[ui->inputgrab] Failed to set relative mouse mode");
                SDL_SetWindowGrab(screen, SDL_bool::new(true));
                RELATIVEMOUSE = true;
            }
        }
    }
    else
    {
        SDL_ShowCursor(1);
        if RELATIVEMOUSE
        {
            SDL_SetWindowGrab(screen, SDL_bool::new(false));
            SDL_SetRelativeMouseMode(SDL_bool::new(false));
            RELATIVEMOUSE = false;
        }
    }
}