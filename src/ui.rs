use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::atomic::Ordering::SeqCst;

use gnal_tsur::gnal_tsur;
use hudhook::{imgui, MessageFilter, RenderContext};
use hudhook::imgui::{Context, FontConfig, FontGlyphRanges, FontId, FontSource, ImColor32, Io};
use once_cell::sync::Lazy;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_INSERT};
use crate::distance;
use crate::entity::Entity;
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
use crate::offsets::offsets::{ENTITY_LIST_OFFSET, LOCAL_PLAYER_OFFSET, NUMBER_OF_PLAYERS_IN_MATCH_OFFSET, VIEW_MATRIX_ADDR};
use crate::settings::{AppSettings, load_app_settings, save_app_settings};
use crate::style::{set_style_minty_light, set_style_minty_mint, set_style_minty_red, set_style_unicore};
use crate::utils::{read_memory, read_view_matrix, run_cmd};
use crate::vars::game_vars::{ENTITY_LIST_PTR, FOV, LOCAL_PLAYER, NUM_PLAYERS_IN_MATCH, SMOOTH, TRIGGER_DELAY, VIEW_MATRIX};
use crate::vars::handles::{AC_CLIENT_EXE_HMODULE, GAME_WINDOW_DIMENSIONS};
use crate::vars::mem_patches::{
    MAPHACK_MEMORY_PATCH, NO_RECOIL_MEMORY_PATCH, RAPID_FIRE_MEMORY_PATCH,
};
use crate::vars::ui_vars::{
    IS_AIMBOT, IS_DRAW_FOV, IS_ESP, IS_FULLBRIGHT, IS_GRENADES_INFINITE, IS_INFINITE_AMMO,
    IS_INVULNERABLE, IS_MAPHACK, IS_NO_RECOIL, IS_NO_RELOAD, IS_RAPID_FIRE, IS_SHOW_UI, IS_SMOOTH,
    IS_TRIGGERBOT, IS_WALLHACK,
};
use crate::vec_structures::Vec2;
use crate::world_to_screen::world_to_screen;

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
            if let Some(item) = ui.tab_item("ESP settings")
            {
                if let Some(tab_esp) = ui.tab_bar("ESP Config Tab Bar")
                {
                    if let Some(tab_esp_item) = ui.tab_item("ESP traceline settings")
                    {
                        if ui.checkbox("Draw tracelines", &mut SETTINGS.is_draw_trace_lines) {
                            println!("Set ESP drawing tracelines Toggle to {}",
                                     SETTINGS.deref().is_draw_trace_lines);
                        }
                        if SETTINGS.is_draw_trace_lines
                        {
                            if ui.slider("Traceline thickness",
                                         0.1f32, 10.0f32,
                                         &mut SETTINGS.trace_line_thickness)
                            {
                                println!("Set traceline thickness {}", SETTINGS.deref().trace_line_thickness);
                            }
                            if ui.checkbox("Show ally traceline", &mut SETTINGS.is_draw_trace_lines_ally)
                            {
                                println!("Toggled show ally traceline {}", SETTINGS.deref().is_draw_trace_lines_ally);
                            }
                            if ui.color_edit3("Ally traceline color",
                                                &mut SETTINGS.ally_trace_line_color)
                            {
                                println!("Set color for ally traceline {:?}",
                                         SETTINGS.deref().ally_trace_line_color);
                            }
                            if ui.checkbox("Show enemy traceline", &mut SETTINGS.is_draw_trace_lines_enemy)
                            {
                                println!("Toggled show enemy traceline {}", SETTINGS.deref().is_draw_trace_lines_enemy);
                            }
                            if ui.color_edit3("Enemy traceline color",
                                                &mut SETTINGS.enemy_trace_line_color)
                            {
                                println!("Set color for enemy traceline {:?}",
                                         SETTINGS.enemy_trace_line_color);
                            }
                        }
                        tab_esp_item.end();
                    }
                    if let Some(tab_esp_item) = ui.tab_item("ESP boxes settings")
                    {
                        if ui.checkbox("Draw boxes", &mut SETTINGS.is_draw_boxes)
                        {
                            println!("Toggled show enemy traceline {}", SETTINGS.deref().is_draw_boxes);
                        }

                        if SETTINGS.is_draw_boxes
                        {
                            if ui.slider("Boxes thickness",
                                         0.1f32, 10.0f32,
                                         &mut SETTINGS.box_thickness)
                            {
                                println!("Set box thickness {}", SETTINGS.deref().box_thickness);
                            }
                            if ui.checkbox("Show ally boxes", &mut SETTINGS.is_draw_boxes_ally)
                            {
                                println!("Toggled show ally boxes {}", SETTINGS.deref().is_draw_boxes_ally);
                            }
                            if ui.color_edit3("Ally boxes color",
                                              &mut SETTINGS.ally_box_color)
                            {
                                println!("Set color for ally boxes {:?}",
                                         SETTINGS.deref().ally_box_color);
                            }
                            if ui.checkbox("Show enemy boxes", &mut SETTINGS.is_draw_boxes_enemy)
                            {
                                println!("Toggled show enemy boxes {}", SETTINGS.deref().is_draw_boxes_enemy);
                            }
                            if ui.color_edit3("Enemy boxes color",
                                              &mut SETTINGS.enemy_box_color)
                            {
                                println!("Set color for enemy boxes {:?}",
                                         SETTINGS.enemy_box_color);
                            }
                        }


                        tab_esp_item.end();
                    }
                    if let Some(tab_esp_item) = ui.tab_item("ESP health bar settings")
                    {
                        if ui.checkbox("Draw health bars", &mut SETTINGS.is_draw_hp_bar)
                        {
                            println!("Toggled show enemy traceline {}", SETTINGS.deref().is_draw_boxes);
                        }

                        if SETTINGS.is_draw_hp_bar
                        {
                            if ui.checkbox("Show horizontal hp bar", &mut SETTINGS.is_horizontal_hp_bar)
                            {
                                println!("Toggled horizontal hp bar {}", SETTINGS.deref().is_horizontal_hp_bar);
                            }
                            ui.same_line();
                            if ui.checkbox("Show vertical hp bar", &mut SETTINGS.is_vertical_hp_bar)
                            {
                                println!("Toggled vertical hp bar {}", SETTINGS.deref().is_vertical_hp_bar);
                            }
                            if ui.slider("Health bar thickness",
                                         0.1f32, 10.0f32,
                                         &mut SETTINGS.hp_bar_thickness)
                            {
                                println!("Set health bar thickness {}", SETTINGS.deref().hp_bar_thickness);
                            }
                            if ui.checkbox("Show ally health bar", &mut SETTINGS.is_draw_hp_bar_ally)
                            {
                                println!("Toggled show ally health bar {}", SETTINGS.deref().is_draw_hp_bar_ally);
                            }
                            ui.same_line();
                            if ui.checkbox("Show enemy health bar", &mut SETTINGS.is_draw_hp_bar_enemy)
                            {
                                println!("Toggled show enemy health bar {}", SETTINGS.deref().is_draw_hp_bar_enemy);
                            }
                            if ui.color_edit3("Inner health bar color",
                                              &mut SETTINGS.inner_hp_bar_color)
                            {
                                println!("Set color for inner health bar {:?}",
                                         SETTINGS.deref().inner_hp_bar_color);
                            }

                            if ui.color_edit3("Outer health bar color",
                                              &mut SETTINGS.outer_hp_bar_color)
                            {
                                println!("Set color for outer health bar {:?}",
                                         SETTINGS.outer_hp_bar_color);
                            }
                        }
                        tab_esp_item.end();
                    }
                    if let Some(tab_esp_item) = ui.tab_item("ESP text settings")
                    {
                        if ui.checkbox("Draw name text", &mut SETTINGS.is_draw_name_text)
                        {
                            println!("Toggled draw name text to {}", SETTINGS.deref().is_draw_name_text)
                        }
                        if SETTINGS.deref().is_draw_name_text
                        {
                            if ui.checkbox("Draw ally name text", &mut SETTINGS.is_draw_name_text_ally)
                            {
                                println!("Toggled draw ally name text to {}", SETTINGS.deref().is_draw_name_text_ally)
                            }
                            ui.same_line();
                            if ui.checkbox("Draw enemy name text", &mut SETTINGS.is_draw_name_text_enemy)
                            {
                                println!("Toggled draw enemy name text to {}", SETTINGS.deref().is_draw_name_text_enemy)
                            }
                            if ui.slider("Name text size", 10.0f32, 60.0f32, &mut SETTINGS.name_text_thickness)
                            {
                                println!("Set name text size to {}", SETTINGS.deref().name_text_thickness)
                            }
                            if ui.color_edit3("Ally name text color", &mut SETTINGS.ally_name_text_color)
                            {
                                println!("Set ally name text color to {:?}", SETTINGS.ally_name_text_color);
                            }
                            if ui.color_edit3("Enemy name text color", &mut SETTINGS.enemy_name_text_color)
                            {
                                println!("Set ally name text color to {:?}", SETTINGS.enemy_name_text_color);
                            }
                        }
                        tab_esp_item.end();
                    }
                    tab_esp.end();
                }
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

            if IS_ESP.load(SeqCst) {




                let local_player_addr =
                    match read_memory::<usize>(AC_CLIENT_EXE_HMODULE + LOCAL_PLAYER_OFFSET) {
                        Ok(addr) => addr,
                        Err(err) => {
                            println!("Error reading local player address: {}", err);
                            return ();
                        }
                    };

                LOCAL_PLAYER = Entity::from_addr(local_player_addr);

                let num_players_in_match =
                    match read_memory::<i32>(AC_CLIENT_EXE_HMODULE + NUMBER_OF_PLAYERS_IN_MATCH_OFFSET)
                    {
                        Ok(num) => num as usize,
                        Err(err) => {
                            println!("Error reading number of players in match: {}", err);
                            return ();
                        }
                    };
                NUM_PLAYERS_IN_MATCH = num_players_in_match;

                let entity_list_ptr =
                    match read_memory::<usize>(AC_CLIENT_EXE_HMODULE + ENTITY_LIST_OFFSET) {
                        Ok(ptr) => ptr,
                        Err(err) => {
                            println!("Error reading entity list pointer: {}", err);
                            return ();
                        }
                    };
                ENTITY_LIST_PTR = entity_list_ptr;

                match read_view_matrix(VIEW_MATRIX_ADDR) {
                    Ok(matrix) => {
                        VIEW_MATRIX.copy_from_slice(&matrix);
                    }
                    Err(err) => {
                        println!("Error reading view matrix: {}", err);
                    }
                };

                for i in 1..NUM_PLAYERS_IN_MATCH {
                    println!("IMGUI ESP iterating for i={}",i);
                    let entity_addr =
                        match Entity::from_addr(ENTITY_LIST_PTR).read_value::<usize>(i * 0x4) {
                            Ok(addr) => addr,
                            Err(err) => {
                                println!("Error reading entity address: {}", err);
                                continue;
                            }
                        };

                    if entity_addr == 0 {
                        continue;
                    }

                    let entity = Entity::from_addr(entity_addr);

                    // Check if the entity is alive using the updated is_alive method
                    if !entity.is_alive() {
                        continue;
                    }

                    let mut feet_screen_pos = Vec2 { x: 0.0, y: 0.0 };
                    let mut head_screen_pos = Vec2 { x: 0.0, y: 0.0 };

                    // Use match expressions for error handling with position and head_position
                    let entity_position = match entity.position() {
                        Ok(pos) => pos,
                        Err(err) => {
                            println!("Error reading entity position: {}", err);
                            continue; // Skip to the next entity if there's an error
                        }
                    };

                    let entity_head_position = match entity.head_position() {
                        Ok(pos) => pos,
                        Err(err) => {
                            println!("Error reading entity head position: {}", err);
                            continue; // Skip to the next entity if there's an error
                        }
                    };

                    if !world_to_screen(
                        entity_position,
                        &mut feet_screen_pos,
                        VIEW_MATRIX,
                        GAME_WINDOW_DIMENSIONS.width,
                        GAME_WINDOW_DIMENSIONS.height,
                    ) {
                        continue;
                    }

                    if !world_to_screen(
                        entity_head_position,
                        &mut head_screen_pos,
                        VIEW_MATRIX,
                        GAME_WINDOW_DIMENSIONS.width,
                        GAME_WINDOW_DIMENSIONS.height,
                    ) {
                        continue;
                    }

                    let distance = match (LOCAL_PLAYER.position(), entity.position()) {
                        (Ok(local_pos), Ok(entity_pos)) => distance::distance_3d(local_pos, entity_pos),
                        (Err(err), _) | (_, Err(err)) => {
                            println!("Error reading position: {}", err);
                            continue; // Skip to the next entity if there's an error
                        }
                    };

                    let EntityHeight = head_screen_pos.y - feet_screen_pos.y;
                    let EntityWidth = EntityHeight / 2.5f32;

                    let ESPBottom = feet_screen_pos.y;
                    let ESPLeft = feet_screen_pos.x + (EntityWidth / 2.0f32);
                    let ESPRight = feet_screen_pos.x - (EntityWidth / 2.0f32);
                    let ESPTop = head_screen_pos.y + (EntityHeight * 0.1f32);
                    let background_draw_list = ui.get_background_draw_list();
                    if SETTINGS.deref().is_draw_trace_lines
                    {
                        if (SETTINGS.deref().is_draw_trace_lines_ally && LOCAL_PLAYER.team().unwrap() == entity.team().unwrap()) ||
                            (SETTINGS.deref().is_draw_trace_lines_enemy && LOCAL_PLAYER.team().unwrap() != entity.team().unwrap())
                        {
                            let traceline = background_draw_list.add_line(
                                [GAME_WINDOW_DIMENSIONS.width as f32 / 2f32, GAME_WINDOW_DIMENSIONS.height as f32],
                                [(ESPRight + ESPLeft) / 2f32, ESPBottom],
                                if LOCAL_PLAYER.team().unwrap() == entity.team().unwrap() {
                                    SETTINGS.deref().ally_trace_line_color // Color for same team
                                } else {
                                    SETTINGS.deref().enemy_trace_line_color // Color for different team (example)
                                }
                            ).thickness(SETTINGS.deref().trace_line_thickness);
                            traceline.build();
                        }
                    }
                    if SETTINGS.deref().is_draw_boxes
                    {
                        if (SETTINGS.deref().is_draw_boxes_ally && LOCAL_PLAYER.team().unwrap() == entity.team().unwrap()) ||
                            (SETTINGS.deref().is_draw_boxes_enemy && LOCAL_PLAYER.team().unwrap() != entity.team().unwrap())
                        {
                            let box_upper_line = background_draw_list.add_line(
                                [ESPLeft, ESPTop],
                                [ESPRight, ESPTop],
                                if LOCAL_PLAYER.team().unwrap() == entity.team().unwrap() {
                                    SETTINGS.deref().ally_box_color // Color for same team
                                } else {
                                    SETTINGS.deref().enemy_box_color // Color for different team (example)
                                }
                            ).thickness(SETTINGS.deref().box_thickness);

                            let box_bottom_line = background_draw_list.add_line(
                                [ESPLeft, ESPBottom],
                                [ESPRight, ESPBottom],
                                if LOCAL_PLAYER.team().unwrap() == entity.team().unwrap() {
                                    SETTINGS.deref().ally_box_color // Color for same team
                                } else {
                                    SETTINGS.deref().enemy_box_color // Color for different team (example)
                                }
                            ).thickness(SETTINGS.deref().box_thickness);

                            let box_left_line = background_draw_list.add_line(
                                [ESPLeft, ESPTop],
                                [ESPLeft, ESPBottom],
                                if LOCAL_PLAYER.team().unwrap() == entity.team().unwrap() {
                                    SETTINGS.deref().ally_box_color // Color for same team
                                } else {
                                    SETTINGS.deref().enemy_box_color // Color for different team (example)
                                }
                            ).thickness(SETTINGS.deref().box_thickness);

                            let box_right_line = background_draw_list.add_line(
                                [ESPRight, ESPTop],
                                [ESPRight, ESPBottom],
                                if LOCAL_PLAYER.team().unwrap() == entity.team().unwrap() {
                                    SETTINGS.deref().ally_box_color // Color for same team
                                } else {
                                    SETTINGS.deref().enemy_box_color // Color for different team (example)
                                }
                            ).thickness(SETTINGS.deref().box_thickness);

                            box_upper_line.build();
                            box_bottom_line.build();
                            box_left_line.build();
                            box_right_line.build();
                        }
                    }
                    if SETTINGS.deref().is_draw_hp_bar
                    {
                        if (SETTINGS.deref().is_draw_hp_bar_ally && LOCAL_PLAYER.team().unwrap() == entity.team().unwrap()) ||
                            (SETTINGS.deref().is_draw_hp_bar_enemy && LOCAL_PLAYER.team().unwrap() != entity.team().unwrap())
                        {
                            if SETTINGS.deref().is_vertical_hp_bar
                            {
                                // Calculate the height of the health bar based on the entity's health

                                let HPTop = entity.health().unwrap() as f32 * (ESPTop - ESPBottom) / 100.0f32;

                                // Draw the background for the health bar
                                let outer_hp_bar = background_draw_list.add_line(
                                    [ESPLeft - 20.0f32, ESPTop], // Left side of the box
                                    [ESPLeft - 20.0f32, ESPBottom], // Bottom of the box to the top
                                    SETTINGS.deref().outer_hp_bar_color // Outer hp bar color
                                ).thickness(SETTINGS.deref().hp_bar_thickness);

                                // Draw the inner health bar
                                let inner_hp_bar = background_draw_list.add_line(
                                    [ESPLeft - 20.0f32, ESPBottom + HPTop], // Start at the top of the inner health bar
                                    [ESPLeft - 20.0f32, ESPBottom], // End at the bottom of the box
                                    SETTINGS.deref().inner_hp_bar_color // Inner hp bar color
                                ).thickness(SETTINGS.deref().hp_bar_thickness);

                                // Build the lines
                                outer_hp_bar.build();
                                inner_hp_bar.build();
                            }
                            if SETTINGS.deref().is_horizontal_hp_bar
                            {
                                let HPRight = entity.health().unwrap() as f32 * (ESPRight - ESPLeft) / 100.0f32;
                                let inner_hp_bar = background_draw_list.add_line(
                                    [ESPLeft, ESPBottom + 20.0f32],
                                    [ESPRight, ESPBottom + 20.0f32],
                                    SETTINGS.deref().inner_hp_bar_color // Inner hp bar color
                                ).thickness(SETTINGS.deref().hp_bar_thickness);
                                let outer_hp_bar = background_draw_list.add_line(
                                    [ESPLeft + HPRight, ESPBottom + 20.0f32],
                                    [ESPRight, ESPBottom + 20.0f32],
                                    SETTINGS.deref().outer_hp_bar_color // Outer hp bar color
                                ).thickness(SETTINGS.deref().hp_bar_thickness);
                                inner_hp_bar.build();
                                outer_hp_bar.build();
                            }
                        }
                    }
                    if SETTINGS.deref().is_draw_name_text
                    {
                        if (SETTINGS.deref().is_draw_name_text_ally && LOCAL_PLAYER.team().unwrap() == entity.team().unwrap()) ||
                            (SETTINGS.deref().is_draw_name_text_enemy && LOCAL_PLAYER.team().unwrap() != entity.team().unwrap())
                        {
                            background_draw_list.add_text([ESPLeft, ESPTop - 20.0f32],
                                                                  if LOCAL_PLAYER.team().unwrap() == entity.team().unwrap()
                                                                  {SETTINGS.deref().ally_name_text_color} else {SETTINGS.deref().enemy_name_text_color},
                                                                    entity.name().unwrap());

                        }
                    }
                    if IS_AIMBOT.load(SeqCst) && IS_DRAW_FOV.load(SeqCst) {
                        let circle = background_draw_list.add_circle([
                            GAME_WINDOW_DIMENSIONS.width as f32 / 2.0f32,
                            GAME_WINDOW_DIMENSIONS.height as f32 / 2.0f32],
                            FOV.load(SeqCst) as f32,
                            [255.0f32,255.0f32,255.0f32]);
                        circle.build();
                    }
                }



            }


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
