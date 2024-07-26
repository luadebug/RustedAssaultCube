use std::path::PathBuf;
use std::ptr::addr_of_mut;

use hudhook::{imgui, MessageFilter, RenderContext};
use hudhook::imgui::{Context, internal::RawCast, Io, sys::{ImFontAtlas_AddFontFromFileTTF, ImFontAtlas_GetGlyphRangesChineseFull}};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_INSERT};

use crate::aimbot::aimbot;
use crate::esp::esp_entrypoint;
use crate::game;
use crate::game::set_brightness_toggle;
use crate::style::set_dark_style;
use crate::vars::game_vars::{FOV, SMOOTH, TRIGGER_DELAY};
use crate::vars::mem_patches::{MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH};
use crate::vars::ui_vars::{IS_AIMBOT, IS_DRAW_FOV, IS_ESP, IS_FULLBRIGHT, IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_MAPHACK, IS_NO_RECOIL, IS_NO_RELOAD, IS_RAPID_FIRE, IS_SHOW_UI, IS_SMOOTH, IS_TRIGGERBOT};

pub unsafe fn on_frame(ui: &imgui::Ui) {
    ui.text("Hello from `hudhook`!");
    if ui.checkbox("[Delete] ESP", &mut *addr_of_mut!(IS_ESP)) {
        IS_ESP = !IS_ESP;
        if IS_ESP {
            esp_entrypoint().expect("[ui] Failed to call esp_entrypoint()");
        }
    }
    if ui.checkbox("[F1] Infinite Grenades", &mut *addr_of_mut!(IS_GRENADES_INFINITE)) {
        println!("Set Grenade Infinite Toggle to {}", IS_GRENADES_INFINITE);
    }
    if ui.checkbox("[F2] No Reload", &mut *addr_of_mut!(IS_NO_RELOAD)) {
        println!("Set No Reload Toggle to {}", IS_NO_RELOAD);
    }
    if ui.checkbox("[F3] Invulnerability", &mut *addr_of_mut!(IS_INVULNERABLE)) {
        println!("Set Invulnerability Toggle to {}", IS_INVULNERABLE);
    }
    if ui.checkbox("[F4] Infinite Ammo", &mut *addr_of_mut!(IS_INFINITE_AMMO)) {
        println!("Set Infinite Ammo Toggle to {}", IS_INFINITE_AMMO);
    }
    if ui.checkbox("[F5] No Recoil", &mut *addr_of_mut!(IS_NO_RECOIL)) {
        println!("Set No Recoil Toggle to {}", IS_NO_RECOIL);
        if IS_NO_RECOIL {
            NO_RECOIL_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory no recoil");
        } else {
            NO_RECOIL_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory no recoil");
        }
    }
    if ui.checkbox("[F6] Rapid Fire", &mut *addr_of_mut!(IS_RAPID_FIRE)) {
        println!("Set Rapid Fire Toggle to {}", IS_RAPID_FIRE);
        if IS_RAPID_FIRE {
            RAPID_FIRE_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory rapid fire");
        } else {
            RAPID_FIRE_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory rapid fire");
        }
    }
    if ui.checkbox("[F7] Aimbot (press C for aim to head)", &mut *addr_of_mut!(IS_AIMBOT)) {
        println!("Set Aimbot Toggle to {}", IS_AIMBOT);
    }
    if IS_AIMBOT
    {
        if ui.checkbox("[F8] Aimbot Draw FOV", &mut *addr_of_mut!(IS_DRAW_FOV)) {
            println!("Set Aimbot Draw FOV Toggle to {}", IS_DRAW_FOV);
        }
        if ui.slider("FOV", 1.0, 300.0, &mut *addr_of_mut!(FOV)) {
            println!("Set Aimbot FOV to {}", FOV);
        }
        if ui.checkbox("[F9] Aimbot Smooth", &mut *addr_of_mut!(IS_SMOOTH)) {
            println!("Set Aimbot Draw FOV Toggle to {}", IS_SMOOTH);
        }
        if ui.slider("Smooth", 1.0, 100.0, &mut *addr_of_mut!(SMOOTH)) {
            println!("Set Aimbot Smooth to {}", SMOOTH);
        }
    }

    if ui.checkbox("[F10] Triggerbot", &mut *addr_of_mut!(IS_TRIGGERBOT)) {
        println!("Set Triggerbot Toggle to {}", IS_TRIGGERBOT);
    }
    if ui.slider("Triggerbot Delay ms", 100.0, 1000.0, &mut *addr_of_mut!(TRIGGER_DELAY)) {
        println!("Set Triggerbot Delay to {} ms", TRIGGER_DELAY);
    }
    if ui.checkbox("[F11] Maphack", &mut *addr_of_mut!(IS_MAPHACK)) {
        println!("Set Maphack Toggle to {}", IS_MAPHACK);
        if IS_MAPHACK {
            MAPHACK_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory maphack");
        } else {
            MAPHACK_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory maphack");
        }
    }
    if ui.checkbox("[F12] Full Bright", &mut *addr_of_mut!(IS_FULLBRIGHT)) {
        println!("Set Full Bright Toggle to {}", IS_FULLBRIGHT);

        // Get the function pointer after setting the brightness
        let set_brightness_func = game::set_brightness();

        // Ensure the function pointer is valid
        if !set_brightness_func.is_null() {
            set_brightness_toggle(IS_FULLBRIGHT);

        } else {
            println!("Function pointer to set_brightness is null!");
        }
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
    fn initialize<'a>(&'a mut self, _ctx: &mut Context,
                      _render_context: &'a mut dyn RenderContext) {


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
        _render_context: &'a mut dyn RenderContext,
    ) {
        unsafe {
            if IS_AIMBOT
            {
                aimbot();
            }
            hotkey_handler();
            _ctx.io_mut().mouse_draw_cursor = IS_SHOW_UI;
            _ctx.io_mut().want_set_mouse_pos = IS_SHOW_UI;
            _ctx.io_mut().want_capture_mouse = IS_SHOW_UI;
            return;
        }
    }

    fn message_filter(&self, _io: &Io) -> MessageFilter
    {
        unsafe {
            if IS_SHOW_UI
            {
                MessageFilter::InputAll | MessageFilter::WindowFocus
            } else {
                MessageFilter::WindowFocus
            }
        }
        //MessageFilter::WindowFocus // Filter game cursor midpoint window focus
        //MessageFilter::InputAll
    }


    fn render(&mut self, ui: &mut imgui::Ui) {

        unsafe {
            if !IS_SHOW_UI {
                return;
            }



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

unsafe fn hotkey_handler()
{
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
        IS_AIMBOT = !IS_AIMBOT;
    }
    if GetAsyncKeyState(VK_F8.0 as i32) & 1 == 1 && IS_AIMBOT
    {
        IS_DRAW_FOV = !IS_DRAW_FOV;
    }
    if GetAsyncKeyState(VK_F9.0 as i32) & 1 == 1 && IS_AIMBOT
    {
        IS_SMOOTH = !IS_SMOOTH;
    }
    if GetAsyncKeyState(VK_F10.0 as i32) & 1 == 1
    {
        IS_TRIGGERBOT = !IS_TRIGGERBOT;
    }
    if GetAsyncKeyState(VK_F11.0 as i32) & 1 == 1
    {
        IS_MAPHACK = !IS_MAPHACK;
        if IS_MAPHACK {
            MAPHACK_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory Maphack");
        } else {
            MAPHACK_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory Maphack");
        }
    }
    if GetAsyncKeyState(VK_F12.0 as i32) & 1 == 1
    {
        IS_FULLBRIGHT = !IS_FULLBRIGHT;
        // Get the function pointer after setting the brightness
        let set_brightness_func = game::set_brightness();

        // Ensure the function pointer is valid
        if !set_brightness_func.is_null() {
            set_brightness_toggle(IS_FULLBRIGHT);

        } else {
            println!("Function pointer to set_brightness is null!");
        }
    }
}