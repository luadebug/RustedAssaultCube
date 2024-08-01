use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::atomic::Ordering::SeqCst;

use gnal_tsur::gnal_tsur;
use hudhook::{imgui, MessageFilter, RenderContext};
use hudhook::imgui::{Context, FontConfig, FontGlyphRanges, FontId, FontSource, Io};
use once_cell::sync::Lazy;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_INSERT};

use crate::key_action::aimbot;
use crate::game::{set_brightness_toggle, set_brightness};
use crate::hotkey_widget::{ImGuiKey, KeyboardInputSystem, to_win_key};
use crate::key_action::{KeyAction, toggle_aimbot, toggle_draw_fov, toggle_esp, toggle_fullbright, toggle_infinite_ammo, toggle_infinite_nades, toggle_invulnerability, toggle_maphack, toggle_no_recoil, toggle_no_reload, toggle_rapid_fire, toggle_show_ui, toggle_smooth, toggle_triggerbot, toggle_wallhack};
use crate::locales::cantonese_locale::CANTONESE_LOCALE_VECTOR;
use crate::locales::chinese_locale::MANDARIN_LOCALE_VECTOR;
use crate::locales::english_locale::ENG_LOCALE_VECTOR;
use crate::locales::hebrew_locale::HEBREW_LOCALE_VECTOR;
use crate::locales::russian_locale::RUS_LOCALE_VECTOR;
use crate::locales::ukrainian_locale::UA_LOCALE_VECTOR;
use crate::settings::{AppSettings, load_app_settings, save_app_settings};
use crate::style::{set_style_minty_light, set_style_minty_mint, set_style_minty_red, set_style_unicore};
use crate::utils::run_cmd;
use crate::vars::game_vars::{FOV, SMOOTH, TRIGGER_DELAY};
use crate::vars::mem_patches::{
    MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH,
};
use crate::vars::ui_vars::{
    IS_AIMBOT, IS_DRAW_FOV, IS_ESP, IS_FULLBRIGHT, IS_GRENADES_INFINITE, IS_INFINITE_AMMO,
    IS_INVULNERABLE, IS_MAPHACK, IS_NO_RECOIL, IS_NO_RELOAD, IS_RAPID_FIRE, IS_SHOW_UI, IS_SMOOTH,
    IS_TRIGGERBOT, IS_WALLHACK,
};
pub static mut KEY_INPUT_SYSTEM: KeyboardInputSystem =  unsafe { KeyboardInputSystem::new() };
static mut IS_NEED_CHANGE_THEME:bool = true;
static mut IS_NEED_CHANGE_LOCALE:bool = true;
static mut CURRENT_LOCALE_VECTOR: [&str; 24] = ENG_LOCALE_VECTOR;

pub unsafe fn on_frame(ui: &imgui::Ui, app_settings: &mut AppSettings) {
    unsafe {
        let set_cl = app_settings.clone();


        if let Some(tab) = ui.tab_bar("Main Menu Tab Bar")
        {
            if let Some(item) = ui.tab_item(CURRENT_LOCALE_VECTOR[1])
            {
                if ui.checkbox(CURRENT_LOCALE_VECTOR[2], IS_WALLHACK.get_mut()) {
                    println!("Set WallHack Toggle to {}", IS_WALLHACK.load(SeqCst));
                }
                ui.same_line();
                if ui.button_key_optional(
                    "WallHack HotKey",
                    &mut app_settings.wallhack_key,
                    [0., 0.],
                    &AppSettings::default().wallhack_key,
                    set_cl.keys(),
                ) {
                    println!("Binded Wallhack toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[3], IS_ESP.get_mut()) {
                    println!("Set ESP Toggle to {}", IS_ESP.load(SeqCst));
                }
                ui.same_line();
                if ui.button_key_optional(
                    "ESP HotKey",
                    &mut app_settings.esp_key,
                    [0., 0.],
                    &AppSettings::default().esp_key,
                    set_cl.keys(),
                ) {
                    println!("Binded ESP toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[4], IS_GRENADES_INFINITE.get_mut()) {
                    println!(
                        "Set Grenade Infinite Toggle to {}",
                        IS_GRENADES_INFINITE.load(SeqCst)
                    );
                }
                ui.same_line();
                if ui.button_key_optional(
                    "Inf Grenades HotKey",
                    &mut app_settings.inf_nade,
                    [0., 0.],
                    &AppSettings::default().inf_nade,
                    set_cl.keys(),
                ) {
                    println!("Binded Inf Grenades toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[5], IS_NO_RELOAD.get_mut()) {
                    println!("Set No Reload Toggle to {}", IS_NO_RELOAD.load(SeqCst));
                }
                ui.same_line();
                if ui.button_key_optional(
                    "No Reload HotKey",
                    &mut app_settings.no_reload,
                    [0., 0.],
                    &AppSettings::default().no_reload,
                    set_cl.keys(),
                ) {
                    println!("Binded No Reload toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[6], IS_INVULNERABLE.get_mut()) {
                    println!(
                        "Set Invulnerability Toggle to {}",
                        IS_INVULNERABLE.load(SeqCst)
                    );
                }
                ui.same_line();
                if ui.button_key_optional(
                    "Invulnerability HotKey",
                    &mut app_settings.invul,
                    [0., 0.],
                    &AppSettings::default().invul,
                    set_cl.keys(),
                ) {
                    println!("Binded Invulnerability toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[7], IS_INFINITE_AMMO.get_mut()) {
                    println!(
                        "Set Infinite Ammo Toggle to {}",
                        IS_INFINITE_AMMO.load(SeqCst)
                    );
                }
                ui.same_line();
                if ui.button_key_optional(
                    "Inf Ammo HotKey",
                    &mut app_settings.inf_ammo,
                    [0., 0.],
                    &AppSettings::default().inf_ammo,
                    set_cl.keys(),
                ) {
                    println!("Binded Inf Ammo toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[8], IS_NO_RECOIL.get_mut()) {
                    println!("Set No Recoil Toggle to {}", IS_NO_RECOIL.load(SeqCst));
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
                ui.same_line();
                if ui.button_key_optional(
                    "No Recoil HotKey",
                    &mut app_settings.no_recoil,
                    [0., 0.],
                    &AppSettings::default().no_recoil,
                    set_cl.keys(),
                ) {
                    println!("Binded no recoil toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[9], IS_RAPID_FIRE.get_mut()) {
                    println!("Set Rapid Fire Toggle to {}", IS_RAPID_FIRE.load(SeqCst));
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
                ui.same_line();
                if ui.button_key_optional(
                    "Rapid Fire HotKey",
                    &mut app_settings.rapid_fire,
                    [0., 0.],
                    &AppSettings::default().rapid_fire,
                    set_cl.keys(),
                ) {
                    println!("Binded rapid fire toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[10], IS_AIMBOT.get_mut()) {
                    println!("Set Aimbot toggle to {}", IS_AIMBOT.load(SeqCst));
                }
                ui.same_line();
                if ui.button_key_optional(
                    "Aimbot HotKey",
                    &mut app_settings.aimbot,
                    [0., 0.],
                    &AppSettings::default().aimbot,
                    set_cl.keys(),
                ) {
                    println!("Binded aimbot toggle key!");
                }
                if IS_AIMBOT.load(SeqCst) {
                    ui.text("Aim Hotkey");
                    ui.same_line();
                    if ui.button_key_optional(
                        "Aimbot Aim HotKey",
                        &mut app_settings.aim_key,
                        [0., 0.],
                        &AppSettings::default().aim_key,
                        set_cl.keys(),
                    ) {
                        println!("Binded aimbot aim key!");
                    }

                    if ui.checkbox(CURRENT_LOCALE_VECTOR[11], IS_DRAW_FOV.get_mut()) {
                        println!("Set Aimbot Draw FOV Toggle to {}", IS_DRAW_FOV.load(SeqCst));
                    }
                    ui.same_line();
                    if ui.button_key_optional(
                        "Aimbot Draw FOV HotKey",
                        &mut app_settings.aim_draw_fov,
                        [0., 0.],
                        &AppSettings::default().aim_draw_fov,
                        set_cl.keys(),
                    ) {
                        println!("Binded aimbot draw fov key!");
                    }
                    if ui.slider(CURRENT_LOCALE_VECTOR[12], 1, 300, FOV.get_mut()) {
                        println!("Set Aimbot FOV to {}", FOV.load(SeqCst));
                    }
                    if ui.checkbox(CURRENT_LOCALE_VECTOR[13], IS_SMOOTH.get_mut()) {
                        println!("Set Aimbot Draw FOV Toggle to {}", IS_SMOOTH.load(SeqCst));
                    }
                    ui.same_line();
                    if ui.button_key_optional(
                        "Aimbot Smooth HotKey",
                        &mut app_settings.aim_smooth,
                        [0., 0.],
                        &AppSettings::default().aim_smooth,
                        set_cl.keys(),
                    ) {
                        println!("Binded aimbot smooth key!");
                    }
                    if ui.slider(CURRENT_LOCALE_VECTOR[14], 1, 100, SMOOTH.get_mut()) {
                        println!("Set Aimbot Smooth to {}", SMOOTH.load(SeqCst));
                    }
                }

                if ui.checkbox(CURRENT_LOCALE_VECTOR[15], IS_TRIGGERBOT.get_mut()) {
                    println!("Set Triggerbot Toggle to {}", IS_TRIGGERBOT.load(SeqCst));
                }
                ui.same_line();
                if ui.button_key_optional(
                    "Triggerbot HotKey",
                    &mut app_settings.trigger_bot,
                    [0., 0.],
                    &AppSettings::default().trigger_bot,
                    set_cl.keys(),
                ) {
                    println!("Binded Triggerbot toggle key!");
                }
                if ui.slider(CURRENT_LOCALE_VECTOR[16], 100, 1000, TRIGGER_DELAY.get_mut()) {
                    println!("Set Triggerbot Delay to {} ms", TRIGGER_DELAY.load(SeqCst));
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[17], IS_MAPHACK.get_mut()) {
                    println!("Set Maphack Toggle to {}", IS_MAPHACK.load(SeqCst));
                    if IS_MAPHACK.load(SeqCst) {
                        MAPHACK_MEMORY_PATCH
                            .patch_memory()
                            .expect("[ui] Failed to patch memory maphack");
                    } else {
                        MAPHACK_MEMORY_PATCH
                            .unpatch_memory()
                            .expect("[ui] Failed to unpatch memory maphack");
                    }
                }
                ui.same_line();
                if ui.button_key_optional(
                    "MapHack HotKey",
                    &mut app_settings.maphack,
                    [0., 0.],
                    &AppSettings::default().maphack,
                    set_cl.keys(),
                ) {
                    println!("Binded MapHack toggle key!");
                }
                if ui.checkbox(CURRENT_LOCALE_VECTOR[18], IS_FULLBRIGHT.get_mut()) {
                    println!("Set Full Bright Toggle to {}", IS_FULLBRIGHT.load(SeqCst));

                    // Get the function pointer after setting the brightness
                    let set_brightness_func = set_brightness();

                    // Ensure the function pointer is valid
                    if !set_brightness_func.is_null() {
                        set_brightness_toggle(IS_FULLBRIGHT.load(SeqCst));
                    } else {
                        println!("Function pointer to set_brightness is null!");
                    }
                }
                ui.same_line();
                if ui.button_key_optional(
                    "FullBright HotKey",
                    &mut app_settings.fullbright,
                    [0., 0.],
                    &AppSettings::default().fullbright,
                    set_cl.keys(),
                ) {
                    println!("Binded FullBright toggle key!");
                }

                if ui.button(CURRENT_LOCALE_VECTOR[19]) {
                    save_app_settings(app_settings).expect("Failed to save current settings");
                }
                item.end();
            }
            if let Some(item) = ui.tab_item(CURRENT_LOCALE_VECTOR[20])
            {
                if ui.list_box(CURRENT_LOCALE_VECTOR[21],
                               &mut app_settings.theme_id,
                               &["Unicore", "Minty Red", "Minty Light", "Minty Mint"],
                               10i32)
                {
                    println!("Going to change theme to {}", app_settings.theme_id);
                    IS_NEED_CHANGE_THEME = true;
                }
                item.end();
            }
            if let Some(item) = ui.tab_item(CURRENT_LOCALE_VECTOR[22])
            {
                if ui.list_box(CURRENT_LOCALE_VECTOR[23],
                               &mut app_settings.language_id,
                               &["English", "Русский", "Українська",
                                   "中文（简体）", "粵語",
                                   gnal_tsur!("עברית")],
                               10i32)
                {
                    println!("Going to change locale to {}", app_settings.language_id);
                    IS_NEED_CHANGE_LOCALE = true;
                }
                item.end();
            }

            tab.end();
        }

    }
}

static mut SETTINGS: Lazy<AppSettings> = Lazy::new(|| {
    load_app_settings().unwrap_or_default() // Load the settings lazily
});

static mut FONTS_STORAGE: Option<FontIDs> = Some(FontIDs {
    small: unsafe { mem::zeroed() },
    normal: unsafe { mem::zeroed() },
    big: unsafe { mem::zeroed() },
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
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut Context,
        _render_context: &'a mut dyn RenderContext,
    ) {


        _ctx.set_ini_filename(None);

        unsafe {
            init_fonts(_ctx);
        }

        set_style_unicore(_ctx);

        _ctx.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_KEYBOARD;
        _ctx.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_SET_MOUSE_POS;
        _ctx.io_mut().backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;
        _ctx.io_mut().backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        _ctx.set_log_filename(PathBuf::from("imgui_log.txt"));
        unsafe {
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

            let key_actions: Vec<KeyAction> = vec![
                KeyAction { key: VK_INSERT.0 as usize, action: toggle_show_ui },
                KeyAction { key: to_win_key(SETTINGS.deref().wallhack_key.as_ref().unwrap().key).0 as usize, action: toggle_wallhack },
                KeyAction { key: to_win_key(SETTINGS.deref().esp_key.as_ref().unwrap().key).0 as usize, action: toggle_esp },
                KeyAction { key: to_win_key(SETTINGS.deref().inf_nade.as_ref().unwrap().key).0 as usize, action: toggle_infinite_nades },
                KeyAction { key: to_win_key(SETTINGS.deref().no_reload.as_ref().unwrap().key).0 as usize, action: toggle_no_reload },
                KeyAction { key: to_win_key(SETTINGS.deref().invul.as_ref().unwrap().key).0 as usize, action: toggle_invulnerability },
                KeyAction { key: to_win_key(SETTINGS.deref().inf_ammo.as_ref().unwrap().key).0 as usize, action: toggle_infinite_ammo },
                KeyAction { key: to_win_key(SETTINGS.deref().no_recoil.as_ref().unwrap().key).0 as usize, action: toggle_no_recoil },
                KeyAction { key: to_win_key(SETTINGS.deref().rapid_fire.as_ref().unwrap().key).0 as usize, action: toggle_rapid_fire },
                KeyAction { key: to_win_key(SETTINGS.deref().aimbot.as_ref().unwrap().key).0 as usize, action: toggle_aimbot },
                KeyAction { key: to_win_key(SETTINGS.deref().aim_draw_fov.as_ref().unwrap().key).0 as usize, action: toggle_draw_fov },
                KeyAction { key: to_win_key(SETTINGS.deref().aim_smooth.as_ref().unwrap().key).0 as usize, action: toggle_smooth },
                KeyAction { key: to_win_key(SETTINGS.deref().trigger_bot.as_ref().unwrap().key).0 as usize, action: toggle_triggerbot },
                KeyAction { key: to_win_key(SETTINGS.deref().maphack.as_ref().unwrap().key).0 as usize, action: toggle_maphack },
                KeyAction { key: to_win_key(SETTINGS.deref().fullbright.as_ref().unwrap().key).0 as usize, action: toggle_fullbright },
                KeyAction { key: to_win_key(SETTINGS.deref().aim_key.as_ref().unwrap().key).0 as usize, action: aimbot}
            ];


            KEY_INPUT_SYSTEM.update(_ctx.io_mut());

            for key_action in key_actions {
                match KEY_INPUT_SYSTEM.key_states[key_action.key] {
                    true => (key_action.action)(), // Call the action function if the key is pressed
                    false => {}
                }
            }

            if IS_NEED_CHANGE_THEME
            {
                match SETTINGS.deref_mut().theme_id
                {
                    0 => {set_style_unicore(_ctx);},
                    1 => {set_style_minty_red(_ctx);}
                    2 => {set_style_minty_light(_ctx);}
                    3 => {set_style_minty_mint(_ctx);}
                    _ => {set_style_unicore(_ctx);}
                }
                println!("Changed theme to {}", SETTINGS.deref_mut().theme_id);
                IS_NEED_CHANGE_THEME = false;
            }
            if IS_NEED_CHANGE_LOCALE
            {
                match SETTINGS.deref_mut().language_id
                {
                    0 => {CURRENT_LOCALE_VECTOR = ENG_LOCALE_VECTOR;},
                    1 => {CURRENT_LOCALE_VECTOR = RUS_LOCALE_VECTOR;},
                    2 => {CURRENT_LOCALE_VECTOR = UA_LOCALE_VECTOR;},
                    3 => {CURRENT_LOCALE_VECTOR = MANDARIN_LOCALE_VECTOR;},
                    4 => {CURRENT_LOCALE_VECTOR = CANTONESE_LOCALE_VECTOR;},
                    5 => {CURRENT_LOCALE_VECTOR = HEBREW_LOCALE_VECTOR},
                    _ => {CURRENT_LOCALE_VECTOR = ENG_LOCALE_VECTOR;},
                }
                println!("Changed locale to {}", SETTINGS.deref_mut().language_id);
                IS_NEED_CHANGE_LOCALE = false;
            }




            _ctx.io_mut().mouse_draw_cursor = IS_SHOW_UI.load(SeqCst);
            _ctx.io_mut().want_set_mouse_pos = IS_SHOW_UI.load(SeqCst);
            _ctx.io_mut().want_capture_mouse = IS_SHOW_UI.load(SeqCst);
            return;
        }
    }

    fn message_filter(&self, _io: &Io) -> MessageFilter {
        unsafe {
            if IS_SHOW_UI.load(SeqCst) {
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
            let font_id = FONTS_STORAGE
                .as_mut()
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

            if let Some(wt) = ui.window(CURRENT_LOCALE_VECTOR[0])
                .title_bar(true)
                .size([1000.0, 700.0], imgui::Condition::FirstUseEver)
                .position([300.0, 300.0], imgui::Condition::FirstUseEver)
                .begin()
                {
                    unsafe { on_frame(ui, SETTINGS.deref_mut()); }
                    custom_font.pop();
                    wt.end();
                };
        }
    }
}






unsafe fn init_fonts(_ctx: &mut Context) {
    unsafe {
        let fonts = _ctx.fonts();
        let fonts_config_small = FontConfig {
            glyph_ranges:FontGlyphRanges::cyrillic(),
            ..Default::default()};
        let fonts_config_normal = FontConfig {
            glyph_ranges:FontGlyphRanges::cyrillic(),
            ..Default::default()};
        let fonts_config_big = FontConfig {
            glyph_ranges:FontGlyphRanges::cyrillic(),
            ..Default::default()};

        let fonts_config_small_cn = FontConfig {
            glyph_ranges:FontGlyphRanges::chinese_full(),
            ..Default::default()};
        let fonts_config_normal_cn = FontConfig {
            glyph_ranges:FontGlyphRanges::chinese_full(),
            ..Default::default()};
        let fonts_config_big_cn = FontConfig {
            glyph_ranges:FontGlyphRanges::chinese_full(),
            ..Default::default()};

        let fonts_config_small_hebrew = FontConfig {
          glyph_ranges:FontGlyphRanges::from_slice(&[
              0x0590, 0x05FF, // Main Hebrew block
              0xFB1D, 0xFB4F, // Extended Hebrew characters
              0,               // Zero-termination
          ]),
          ..Default::default()
        };
        let fonts_config_normal_hebrew = FontConfig {
            glyph_ranges:FontGlyphRanges::from_slice(&[
                0x0590, 0x05FF, // Main Hebrew block
                0xFB1D, 0xFB4F, // Extended Hebrew characters
                0,               // Zero-termination
            ]),
            ..Default::default()
        };
        let fonts_config_big_hebrew = FontConfig {
            glyph_ranges:FontGlyphRanges::from_slice(&[
                0x0590, 0x05FF, // Main Hebrew block
                0xFB1D, 0xFB4F, // Extended Hebrew characters
                0,               // Zero-termination
            ]),
            ..Default::default()
        };

        // Get Windows Fonts directory
        let fonts_dir = run_cmd("echo %windir%\\Fonts").trim().to_string();

        let cn_font_file_path = format!("{}\\SimHei.ttf", fonts_dir);

        let mut cn_font_file = File::open(&cn_font_file_path).unwrap();
        let mut cn_font_file_bytes = Vec::new();
        cn_font_file.read_to_end(&mut cn_font_file_bytes).unwrap();

        let hebrew_font_file_path = format!("{}\\arial.ttf", fonts_dir);

        let mut hebrew_font_file = File::open(&hebrew_font_file_path).unwrap();
        let mut hebrew_font_file_bytes = Vec::new();
        hebrew_font_file.read_to_end(&mut hebrew_font_file_bytes).unwrap();


        FONTS_STORAGE = Some(FontIDs {
            small: fonts.add_font(&[
                FontSource::TtfData {
                    data: &crate::fonts::graffiti_font::GRAFFITI, // &crate::fonts::clash_font::CLASH,
                    size_pixels: 11.,
                    config: Some(fonts_config_small), //None,
                },
                FontSource::TtfData {
                    data: cn_font_file_bytes.as_slice(),
                    size_pixels: 11.,
                    config: Some(fonts_config_small_cn),
                },
                FontSource::TtfData {
                    data: hebrew_font_file_bytes.as_slice(),
                    size_pixels: 11.,
                    config: Some(fonts_config_small_hebrew),
                },
            ]),
            normal: fonts.add_font(&[
                FontSource::TtfData {
                    data: &crate::fonts::graffiti_font::GRAFFITI, // &crate::fonts::clash_font::CLASH,
                    size_pixels: 18.,
                    config: Some(fonts_config_normal),
                },
                FontSource::TtfData {
                    data: cn_font_file_bytes.as_slice(),
                    size_pixels: 18.,
                    config: Some(fonts_config_normal_cn),
                },
                FontSource::TtfData {
                    data: hebrew_font_file_bytes.as_slice(),
                    size_pixels: 18.,
                    config: Some(fonts_config_normal_hebrew),
                },
            ]),
            big:

                fonts.add_font(&[
                FontSource::TtfData {
                    data: &crate::fonts::graffiti_font::GRAFFITI, // &crate::fonts::clash_font::CLASH,
                    size_pixels: 24.,
                    config: Some(fonts_config_big),
                },
                FontSource::TtfData {
                    data: cn_font_file_bytes.as_slice(),
                    size_pixels: 24.,
                    config: Some(fonts_config_big_cn),
                },
                FontSource::TtfData {
                    data: hebrew_font_file_bytes.as_slice(),
                    size_pixels: 24.,
                    config: Some(fonts_config_big_hebrew),
                },
            ]),
        });
    }
}
