use std::mem;
use std::path::PathBuf;
use std::sync::atomic::Ordering::SeqCst;
use std::error::Error;
use std::ops::{Deref, DerefMut};
// Import the Error trait
use hudhook::{imgui, MessageFilter, RenderContext};
use hudhook::imgui::{Context, FontId, FontSource, Io};
use once_cell::sync::Lazy; // Import Lazy for lazy initialization
use std::sync::Mutex; // Import Mutex if you need to mutate settings
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_INSERT};

use lazy_static::lazy_static; // Import lazy_static macro
use crate::aimbot::aimbot;
use crate::esp::esp_entrypoint;
use crate::game;
use crate::game::set_brightness_toggle;
use crate::hotkey_widget::{ImGuiKey, render_button_key, to_win_key};
use crate::style::set_style_unicore;
use crate::vars::game_vars::{FOV, SMOOTH, TRIGGER_DELAY};
/*use crate::vars::hotkeys::AIM_KEY2;*/
use crate::vars::mem_patches::{MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH};
use crate::vars::ui_vars::{IS_AIMBOT, IS_DRAW_FOV, IS_ESP, IS_FULLBRIGHT, IS_GRENADES_INFINITE, IS_INFINITE_AMMO, IS_INVULNERABLE, IS_MAPHACK, IS_NO_RECOIL, IS_NO_RELOAD, IS_RAPID_FIRE, IS_SHOW_UI, IS_SMOOTH, IS_TRIGGERBOT};
use crate::settings::{AppSettings, load_app_settings};
pub unsafe fn on_frame(ui: &imgui::Ui, app_settings: &mut AppSettings) {
    unsafe {
        if ui.checkbox("[Delete] ESP", IS_ESP.get_mut()) {  //&mut *addr_of_mut!(
            IS_ESP.store(!IS_ESP.load(SeqCst), SeqCst);
        }
        ui.same_line();
        if ui.button_key_optional("ESP HotKey", &mut app_settings.ESP_KEY, [0., 0.], &AppSettings::default().ESP_KEY) {
            println!("Binded ESP toggle key!");
        }
        if ui.checkbox("[F1] Infinite Grenades", IS_GRENADES_INFINITE.get_mut()) {
            println!("Set Grenade Infinite Toggle to {}", IS_GRENADES_INFINITE.load(SeqCst));
        }
        ui.same_line();
        if ui.button_key_optional("Inf Grenades HotKey", &mut app_settings.INF_NADE, [0., 0.], &AppSettings::default().INF_NADE) {
            println!("Binded Inf Grenades toggle key!");
        }
        if ui.checkbox("[F2] No Reload", IS_NO_RELOAD.get_mut()) {
            println!("Set No Reload Toggle to {}", IS_NO_RELOAD.load(SeqCst));
        }
        ui.same_line();
        if ui.button_key_optional("No Reload HotKey", &mut app_settings.NO_RELOAD, [0., 0.], &AppSettings::default().NO_RELOAD) {
            println!("Binded No Reload toggle key!");
        }
        if ui.checkbox("[F3] Invulnerability", IS_INVULNERABLE.get_mut()) {
            println!("Set Invulnerability Toggle to {}", IS_INVULNERABLE.load(SeqCst));
        }
        ui.same_line();
        if ui.button_key_optional("Invulnerability HotKey", &mut app_settings.INVUL, [0., 0.], &AppSettings::default().INVUL) {
            println!("Binded Invulnerability toggle key!");
        }
        if ui.checkbox("[F4] Infinite Ammo", IS_INFINITE_AMMO.get_mut()) {
            println!("Set Infinite Ammo Toggle to {}", IS_INFINITE_AMMO.load(SeqCst));
        }
        ui.same_line();
        if ui.button_key_optional("Inf Ammo HotKey", &mut app_settings.INF_AMMO, [0., 0.], &AppSettings::default().INF_AMMO) {
            println!("Binded Inf Ammo toggle key!");
        }
        if ui.checkbox("[F5] No Recoil", IS_NO_RECOIL.get_mut()) {
            println!("Set No Recoil Toggle to {}", IS_NO_RECOIL.load(SeqCst));
            if IS_NO_RECOIL.load(SeqCst) {
                NO_RECOIL_MEMORY_PATCH.
                    patch_memory().
                    expect("[ui] Failed to patch memory no recoil");
            } else {
                NO_RECOIL_MEMORY_PATCH.
                    unpatch_memory().
                    expect("[ui] Failed to unpatch memory no recoil");
            }
        }
        ui.same_line();
        if ui.button_key_optional("No Recoil HotKey", &mut app_settings.NO_RECOIL, [0., 0.], &AppSettings::default().NO_RECOIL) {
            println!("Binded no recoil toggle key!");
        }
        if ui.checkbox("[F6] Rapid Fire", IS_RAPID_FIRE.get_mut()) {
            println!("Set Rapid Fire Toggle to {}", IS_RAPID_FIRE.load(SeqCst));
            if IS_RAPID_FIRE.load(SeqCst) {
                RAPID_FIRE_MEMORY_PATCH.
                    patch_memory().
                    expect("[ui] Failed to patch memory rapid fire");
            } else {
                RAPID_FIRE_MEMORY_PATCH.
                    unpatch_memory().
                    expect("[ui] Failed to unpatch memory rapid fire");
            }
        }
        ui.same_line();
        if ui.button_key_optional("Rapid Fire HotKey", &mut app_settings.RAPID_FIRE, [0., 0.], &AppSettings::default().RAPID_FIRE) {
            println!("Binded rapid fire toggle key!");
        }
        if ui.checkbox("[F7] Aimbot", IS_AIMBOT.get_mut()) {
            println!("Set Aimbot toggle to {}", IS_AIMBOT.load(SeqCst));
        }
        ui.same_line();
        if ui.button_key_optional("Aimbot HotKey", &mut app_settings.AIMBOT, [0., 0.], &AppSettings::default().AIMBOT) {
            println!("Binded aimbot toggle key!");
        }
        if IS_AIMBOT.load(SeqCst)
        {
            ui.same_line();
            if ui.button_key_optional("Aimbot Aim HotKey", &mut app_settings.AIM_KEY, [0., 0.], &AppSettings::default().AIM_KEY) {
                println!("Binded aimbot aim key!");
            }
            if ui.checkbox("[F8] Aimbot Draw FOV", IS_DRAW_FOV.get_mut()) {
                println!("Set Aimbot Draw FOV Toggle to {}", IS_DRAW_FOV.load(SeqCst));
            }
            ui.same_line();
            if ui.button_key_optional("Aimbot Draw FOV HotKey", &mut app_settings.AIM_DRAW_FOV, [0., 0.], &AppSettings::default().AIM_DRAW_FOV) {
                println!("Binded aimbot draw fov key!");
            }
            if ui.slider("FOV", 1, 300, FOV.get_mut()) {
                println!("Set Aimbot FOV to {}", FOV.load(SeqCst));
            }
            if ui.checkbox("[F9] Aimbot Smooth", IS_SMOOTH.get_mut()) {
                println!("Set Aimbot Draw FOV Toggle to {}", IS_SMOOTH.load(SeqCst));
            }
            ui.same_line();
            if ui.button_key_optional("Aimbot Smooth HotKey", &mut app_settings.AIM_SMOOTH, [0., 0.], &AppSettings::default().AIM_SMOOTH) {
                println!("Binded aimbot smooth key!");
            }
            if ui.slider("Smooth", 1, 100, SMOOTH.get_mut()) {
                println!("Set Aimbot Smooth to {}", SMOOTH.load(SeqCst));
            }
        }

        if ui.checkbox("[F10] Triggerbot", IS_TRIGGERBOT.get_mut()) {
            println!("Set Triggerbot Toggle to {}", IS_TRIGGERBOT.load(SeqCst));
        }
        ui.same_line();
        if ui.button_key_optional("Triggerbot HotKey", &mut app_settings.TRIGGER_BOT, [0., 0.], &AppSettings::default().TRIGGER_BOT) {
            println!("Binded Triggerbot toggle key!");
        }
        if ui.slider("Triggerbot Delay ms", 100, 1000, TRIGGER_DELAY.get_mut()) {
            println!("Set Triggerbot Delay to {} ms", TRIGGER_DELAY.load(SeqCst));
        }
        if ui.checkbox("[F11] Maphack", IS_MAPHACK.get_mut()) {
            println!("Set Maphack Toggle to {}", IS_MAPHACK.load(SeqCst));
            if IS_MAPHACK.load(SeqCst) {
                MAPHACK_MEMORY_PATCH.
                    patch_memory().
                    expect("[ui] Failed to patch memory maphack");
            } else {
                MAPHACK_MEMORY_PATCH.
                    unpatch_memory().
                    expect("[ui] Failed to unpatch memory maphack");
            }
        }
        ui.same_line();
        if ui.button_key_optional("MapHack HotKey", &mut app_settings.MAPHACK, [0., 0.], &AppSettings::default().MAPHACK) {
            println!("Binded MapHack toggle key!");
        }
        if ui.checkbox("[F12] Full Bright", IS_FULLBRIGHT.get_mut()) {
            println!("Set Full Bright Toggle to {}", IS_FULLBRIGHT.load(SeqCst));

            // Get the function pointer after setting the brightness
            let set_brightness_func = game::set_brightness();

            // Ensure the function pointer is valid
            if !set_brightness_func.is_null() {
                set_brightness_toggle(IS_FULLBRIGHT.load(SeqCst));
            } else {
                println!("Function pointer to set_brightness is null!");
            }
        }
        ui.same_line();
        if ui.button_key_optional("FullBright HotKey", &mut app_settings.FULLBRIGHT, [0., 0.], &AppSettings::default().FULLBRIGHT) {
            println!("Binded FullBright toggle key!");
        }
    }
}

static mut SETTINGS: Lazy<AppSettings> = Lazy::new(|| {
    load_app_settings().unwrap_or_default() // Load the settings lazily
});

static mut FONTS_STORAGE: Option<FontIDs> = Some(FontIDs {
    small: unsafe {mem::zeroed()},
    normal: unsafe {mem::zeroed()},
    big: unsafe { mem::zeroed() }
});
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

struct FontIDs {
    small: FontId,
    normal: FontId,
    big: FontId,
}

unsafe impl Send for FontIDs {}

unsafe impl Sync for FontIDs {}


impl hudhook::ImguiRenderLoop for RenderLoop {

    fn initialize<'a>(&'a mut self, _ctx: &mut Context,
                      _render_context: &'a mut dyn RenderContext) {

        _ctx.set_ini_filename(None);

        unsafe { init_fonts(_ctx); }



        set_style_unicore(_ctx);

        _ctx.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_KEYBOARD;
        _ctx.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_SET_MOUSE_POS;
        _ctx.io_mut().backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;
        _ctx.io_mut().backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        _ctx.set_log_filename(PathBuf::from("imgui_log.txt"));
        unsafe
        {
            _ctx.io_mut().want_set_mouse_pos = IS_SHOW_UI.load(SeqCst);
            _ctx.io_mut().want_capture_mouse = IS_SHOW_UI.load(SeqCst);
        }
    }

    fn before_render<'a>(
        &'a mut self,
        _ctx: &mut Context,
        _render_context: &'a mut dyn RenderContext,
    ) {
        unsafe {
            if IS_AIMBOT.load(SeqCst)
            {
                aimbot(SETTINGS.deref());
            }
            hotkey_handler();
            _ctx.io_mut().mouse_draw_cursor = IS_SHOW_UI.load(SeqCst);
            _ctx.io_mut().want_set_mouse_pos = IS_SHOW_UI.load(SeqCst);
            _ctx.io_mut().want_capture_mouse = IS_SHOW_UI.load(SeqCst);
            return;
        }
    }

    fn message_filter(&self, _io: &Io) -> MessageFilter
    {
        unsafe {
            if IS_SHOW_UI.load(SeqCst)
            {
                MessageFilter::InputAll |  // Filter any input that being sent in-game
                MessageFilter::WindowFocus // Filter game cursor midpoint window focus
            } else {
                MessageFilter::WindowFocus // Filter game cursor midpoint window focus
            }
        }
    }


    fn render(&mut self, ui: &mut imgui::Ui) {
        unsafe {
            if !IS_SHOW_UI.load(SeqCst) {
                ui.set_mouse_cursor(None);
                return;
            }
            ui.set_mouse_cursor(Some(imgui::MouseCursor::Arrow));

                let width = ui.io().display_size[0];
                let font_id = FONTS_STORAGE.as_mut()
                    .map(|fonts| {
                        if width > 2000. {
                            fonts.big
                        } else if width > 1200. {
                            fonts.normal
                        } else {
                            fonts.small
                        }
                    })
            .unwrap();


            let custom_font = ui.push_font(font_id);

            ui.window("[Insert] Menu")
                .title_bar(true)
                .size([1000.0, 700.0], imgui::Condition::FirstUseEver)
                .position([300.0, 300.0], imgui::Condition::FirstUseEver)
                .build(||
                {
                    on_frame(ui, SETTINGS.deref_mut());
                    custom_font.pop();
                });
        }
    }
}

unsafe fn hotkey_handler()
{
    unsafe {
        if GetAsyncKeyState(VK_INSERT.0 as i32) & 1 == 1 {
            IS_SHOW_UI.store(!IS_SHOW_UI.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().ESP_KEY.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_ESP.store(!IS_ESP.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().INF_NADE.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_GRENADES_INFINITE.store(!IS_GRENADES_INFINITE.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().NO_RELOAD.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_NO_RELOAD.store(!IS_NO_RELOAD.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().INVUL.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_INVULNERABLE.store(!IS_INVULNERABLE.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().INF_AMMO.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_INFINITE_AMMO.store(!IS_INFINITE_AMMO.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().NO_RECOIL.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_NO_RECOIL.store(!IS_NO_RECOIL.load(SeqCst), SeqCst);
            if IS_NO_RECOIL.load(SeqCst) {
                NO_RECOIL_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory no recoil");
            } else {
                NO_RECOIL_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory no recoil");
            }
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().RAPID_FIRE.as_ref().unwrap().key).0 as i32) & 1 == 1 {
            IS_RAPID_FIRE.store(!IS_RAPID_FIRE.load(SeqCst), SeqCst);
            if IS_RAPID_FIRE.load(SeqCst) {
                RAPID_FIRE_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory rapid fire");
            } else {
                RAPID_FIRE_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory rapid fire");
            }
        }
        //if GetAsyncKeyState(VK_F7.0 as i32) & 1 == 1
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().AIMBOT.as_ref().unwrap().key).0 as i32) & 1 == 1
        {
            IS_AIMBOT.store(!IS_AIMBOT.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().AIM_DRAW_FOV.as_ref().unwrap().key).0 as i32) & 1 == 1 && IS_AIMBOT.load(SeqCst)
        {
            IS_DRAW_FOV.store(!IS_DRAW_FOV.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().AIM_SMOOTH.as_ref().unwrap().key).0 as i32) & 1 == 1 && IS_AIMBOT.load(SeqCst)
        {
            IS_SMOOTH.store(!IS_SMOOTH.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().TRIGGER_BOT.as_ref().unwrap().key).0 as i32) & 1 == 1
        {
            IS_TRIGGERBOT.store(!IS_TRIGGERBOT.load(SeqCst), SeqCst);
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().MAPHACK.as_ref().unwrap().key).0 as i32) & 1 == 1
        {
            IS_MAPHACK.store(!IS_MAPHACK.load(SeqCst), SeqCst);
            if IS_MAPHACK.load(SeqCst) {
                MAPHACK_MEMORY_PATCH.patch_memory().expect("[ui] Failed to patch memory Maphack");
            } else {
                MAPHACK_MEMORY_PATCH.unpatch_memory().expect("[ui] Failed to unpatch memory Maphack");
            }
        }
        if GetAsyncKeyState(to_win_key(SETTINGS.deref().FULLBRIGHT.as_ref().unwrap().key).0 as i32) & 1 == 1
        {
            //IS_FULLBRIGHT = !IS_FULLBRIGHT;
            IS_FULLBRIGHT.store(!IS_FULLBRIGHT.load(SeqCst), SeqCst);
            // Get the function pointer after setting the brightness
            let set_brightness_func = game::set_brightness();

            // Ensure the function pointer is valid
            if !set_brightness_func.is_null() {
                set_brightness_toggle(IS_FULLBRIGHT.load(SeqCst));
            } else {
                println!("Function pointer to set_brightness is null!");
            }
        }
    }
}

unsafe fn init_fonts(_ctx: &mut Context)
{
    unsafe {
        let fonts = _ctx.fonts();
        FONTS_STORAGE = Some(FontIDs {
            small: fonts.add_font(
                &[FontSource::TtfData {
                    data: &crate::fonts::clash_font::CLASH,
                    size_pixels: 11.,
                    config: None,
                }]),
            normal: fonts.add_font(
                &[FontSource::TtfData {
                    data: &crate::fonts::clash_font::CLASH,
                    size_pixels: 18.,
                    config: None,
                }]),
            big: fonts.add_font(
                &[FontSource::TtfData {
                    data: &crate::fonts::clash_font::CLASH,
                    size_pixels: 24.,
                    config: None,
                }]),
        });
    }
}