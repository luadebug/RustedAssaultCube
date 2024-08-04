use std::collections::HashSet;

use hudhook::imgui;
use hudhook::imgui::{Io, MouseButton};
use imgui::Key;
use serde::{de::Visitor, Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK__none_, VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8,
    VK_9, VK_A, VK_ADD, VK_APPS, VK_B, VK_BACK, VK_C, VK_CAPITAL, VK_CONTROL, VK_D, VK_DECIMAL,
    VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END, VK_ESCAPE, VK_F, VK_F1, VK_F10, VK_F11, VK_F12,
    VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_G, VK_H, VK_HOME, VK_I, VK_INSERT,
    VK_J, VK_K, VK_L, VK_LBUTTON, VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_M,
    VK_MBUTTON, VK_MENU, VK_MULTIPLY, VK_N, VK_NEXT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1,
    VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9,
    VK_O, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_COMMA,
    VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_P, VK_PAUSE, VK_PRIOR, VK_Q, VK_R, VK_RBUTTON,
    VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_S, VK_SCROLL, VK_SNAPSHOT,
    VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2,
    VK_Y, VK_Z,
};

#[derive(Clone, Debug)]
pub struct HotKey {
    pub key: Key, // Named field for the key
}

impl From<Key> for HotKey {
    fn from(value: Key) -> Self {
        Self { key: value } // Create a new HotKey instance
    }
}

// Implement the From<u16> trait for conversion from virtual key code to HotKey
impl From<u16> for HotKey {
    fn from(value: u16) -> Self {
        let imgui_key = match value {
            // Direct mapping from virtual key codes to imgui::Key
            0x08 => Key::Backspace,
            0x09 => Key::Tab,
            0x0D => Key::Enter,
            0x20 => Key::Space,
            0x1B => Key::Escape,
            0x25 => Key::LeftArrow,
            0x26 => Key::UpArrow,
            0x27 => Key::RightArrow,
            0x28 => Key::DownArrow,
            0x2C => Key::Insert,
            0x2E => Key::Delete,
            0x21 => Key::PageUp,
            0x22 => Key::PageDown,
            0x24 => Key::Home,
            0x23 => Key::End,
            // Add more mappings for additional keys as needed
            // Numeric keys
            0x30 => Key::Alpha0,
            0x31 => Key::Alpha1,
            0x32 => Key::Alpha2,
            0x33 => Key::Alpha3,
            0x34 => Key::Alpha4,
            0x35 => Key::Alpha5,
            0x36 => Key::Alpha6,
            0x37 => Key::Alpha7,
            0x38 => Key::Alpha8,
            0x39 => Key::Alpha9,
            // Alphabet keys
            0x41 => Key::A,
            0x42 => Key::B,
            0x43 => Key::C,
            0x44 => Key::D,
            0x45 => Key::E,
            0x46 => Key::F,
            0x47 => Key::G,
            0x48 => Key::H,
            0x49 => Key::I,
            0x4A => Key::J,
            0x4B => Key::K,
            0x4C => Key::L,
            0x4D => Key::M,
            0x4E => Key::N,
            0x4F => Key::O,
            0x50 => Key::P,
            0x51 => Key::Q,
            0x52 => Key::R,
            0x53 => Key::S,
            0x54 => Key::T,
            0x55 => Key::U,
            0x56 => Key::V,
            0x57 => Key::W,
            0x58 => Key::X,
            0x59 => Key::Y,
            0x5A => Key::Z,
            // Function keys
            0x70 => Key::F1,
            0x71 => Key::F2,
            0x72 => Key::F3,
            0x73 => Key::F4,
            0x74 => Key::F5,
            0x75 => Key::F6,
            0x76 => Key::F7,
            0x77 => Key::F8,
            0x78 => Key::F9,
            0x79 => Key::F10,
            0x7A => Key::F11,
            0x7B => Key::F12,
            // Mouse buttons
            0x01 => Key::MouseLeft,
            0x02 => Key::MouseRight,
            0x04 => Key::MouseMiddle,
            // Add more mappings if necessary
            _ => Key::MouseLeft, // Fallback for unmapped keys
        };

        HotKey { key: imgui_key } // Create a HotKey from the mapped imgui::Key
    }
}

impl Serialize for HotKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self.key))
    }
}

struct HotKeyVisitor;

impl<'de> Visitor<'de> for HotKeyVisitor {
    type Value = HotKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a config key")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        for key in imgui::Key::VARIANTS.iter() {
            if format!("{:?}", key) == v {
                return Ok(HotKey { key: *key });
            }
        }

        Err(E::custom("unknown key value"))
    }
}

impl<'de> Deserialize<'de> for HotKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(HotKeyVisitor)
    }
}

pub trait ImguiUiEx {
    #[allow(unused)]
    fn set_cursor_pos_x(&self, pos: f32);
    #[allow(unused)]
    fn set_cursor_pos_y(&self, pos: f32);
}

impl ImguiUiEx for imgui::Ui {
    fn set_cursor_pos_x(&self, pos: f32) {
        unsafe { imgui::sys::igSetCursorPosX(pos) };
    }

    fn set_cursor_pos_y(&self, pos: f32) {
        unsafe { imgui::sys::igSetCursorPosY(pos) };
    }
}

pub trait ImGuiKey {
    /*    fn button_key(&self, label: &str, key: &mut HotKey, size: [f32; 2]) -> bool;*/
    fn button_key_optional(
        &self,
        label: &str,
        key: &mut Option<HotKey>,
        size: [f32; 2],
        backup_key: &Option<HotKey>,
        key_table: HashSet<&Key>,
    ) -> bool;
}

impl ImGuiKey for imgui::Ui {
    /*    fn button_key(&self, label: &str, key: &mut HotKey, size: [f32; 2]) -> bool {
        let mut key_opt = Some(key.clone());
        if render_button_key(self, label, &mut key_opt, size, false) {
            *key = key_opt.unwrap();
            true
        } else {
            false
        }
    }*/

    fn button_key_optional(
        &self,
        label: &str,
        key: &mut Option<HotKey>,
        size: [f32; 2],
        backup_key: &Option<HotKey>,
        key_table: HashSet<&Key>,
    ) -> bool {
        render_button_key(self, label, key, size, true, backup_key, key_table)
    }
}

pub fn render_button_key(
    ui: &imgui::Ui,
    label: &str,
    key: &mut Option<HotKey>,
    size: [f32; 2],
    optional: bool,
    backup_key: &Option<HotKey>,
    key_table: HashSet<&Key>,
) -> bool {
    let _container = ui.push_id(label);

    let button_label = if let Some(key) = &key {
        format!("{:?}", key.key)
    } else {
        "None".to_string()
    };

    let mut updated = false;
    if optional {
        if ui.button_with_size(&button_label, [size[0] - 35.0, size[1]]) {
            ui.open_popup(label);
        }

        ui.same_line_with_spacing(0.0, 10.0);

        ui.disabled(key.is_none(), || {
            if ui.button_with_size("X", [25.0, 0.0]) {
                updated = true;

                *key = backup_key.clone();

                //*key = None;
            }
        });
    } else if ui.button_with_size(&button_label, size) {
            ui.open_popup(label);
        }


    ui.modal_popup_config(label)
        .inputs(true)
        .collapsible(true)
        .movable(false)
        .menu_bar(false)
        .resizable(false)
        .title_bar(false)
        .build(|| {
            ui.text("Press any key or ESC to exit");
            if ui.is_key_pressed(Key::Escape) {
                ui.close_current_popup();
            } else {
                for key_variant in Key::VARIANTS {
                    if ui.is_key_pressed(key_variant) {
                        if key_table.contains(&key_variant) {
                            ui.close_current_popup();
                        } else {
                            *key = Some(HotKey { key: key_variant });
                            updated = true;
                            ui.close_current_popup();
                        }
                    }
                }
            }
        });

    updated
}

/// Simple input system using the global mouse / keyboard state.
/// This does not require the need to process window messages or the imgui overlay to be active.
// Implement Default for [bool; VK_KEY_MAX]

#[derive(Debug)]
#[allow(unused)]
pub struct KeyboardInputSystem {
    pub key_states: [bool; VK_KEY_MAX],
}

#[allow(unused)]
impl KeyboardInputSystem {
    pub const fn new() -> Self {
        Self {
            key_states: [false; VK_KEY_MAX], // Initialize with all false
        }
    }

    pub unsafe fn update(&mut self, io: &mut Io, is_show_ui: bool) {
        for vkey in 0..VK_KEY_MAX {
            let key_state = unsafe { GetAsyncKeyState(vkey as i32) as u16 };
            let pressed = key_state & 1 == 1;
            if self.key_states[vkey] == pressed {
                continue;
            }

            self.key_states[vkey] = pressed;
            let vkey = VIRTUAL_KEY(vkey as u16);

            handle_key_modifier(io, vkey, pressed);
            unsafe {
                if !is_show_ui
                //Do not handle mouse input in case menu is opened up
                {
                    let mouse_button = match vkey {
                        VK_LBUTTON => Some(MouseButton::Left),
                        VK_RBUTTON => Some(MouseButton::Right),
                        VK_MBUTTON => Some(MouseButton::Middle),
                        VK_XBUTTON1 => Some(MouseButton::Extra1),
                        VK_XBUTTON2 => Some(MouseButton::Extra2),
                        _ => None,
                    };

                    if let Some(button) = mouse_button {
                        io.add_mouse_button_event(button, pressed);
                    }
                }
                if let Some(key) = to_imgui_key(vkey) {
                    println!("Key toogle {:?}: {}", key, pressed);
                    io.add_key_event(key, pressed);
                } else {
                    println!("Missing ImGui key for {:?}", vkey);
                }
            }
        }
    }
}

const VK_KEY_MAX: usize = 256;

pub fn to_imgui_key(keycode: VIRTUAL_KEY) -> Option<Key> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    match keycode {
        VK_TAB => Some(Key::Tab),
        VK_LEFT => Some(Key::LeftArrow),
        VK_RIGHT => Some(Key::RightArrow),
        VK_SHIFT => Some(Key::LeftShift),
        VK_MENU => Some(Key::LeftAlt),
        VK_UP => Some(Key::UpArrow),
        VK_DOWN => Some(Key::DownArrow),
        VK_PRIOR => Some(Key::PageUp),
        VK_NEXT => Some(Key::PageDown),
        VK_HOME => Some(Key::Home),
        VK_END => Some(Key::End),
        VK_INSERT => Some(Key::Insert),
        VK_DELETE => Some(Key::Delete),
        VK_BACK => Some(Key::Backspace),
        VK_SPACE => Some(Key::Space),
        VK_RETURN => Some(Key::Enter),
        VK_ESCAPE => Some(Key::Escape),
        VK_OEM_7 => Some(Key::Apostrophe),
        VK_OEM_COMMA => Some(Key::Comma),
        VK_OEM_MINUS => Some(Key::Minus),
        VK_OEM_PERIOD => Some(Key::Period),
        VK_OEM_2 => Some(Key::Slash),
        VK_OEM_1 => Some(Key::Semicolon),
        VK_OEM_PLUS => Some(Key::Equal),
        VK_OEM_4 => Some(Key::LeftBracket),
        VK_OEM_5 => Some(Key::Backslash),
        VK_OEM_6 => Some(Key::RightBracket),
        VK_OEM_3 => Some(Key::GraveAccent),
        VK_CAPITAL => Some(Key::CapsLock),
        VK_SCROLL => Some(Key::ScrollLock),
        VK_NUMLOCK => Some(Key::NumLock),
        VK_SNAPSHOT => Some(Key::PrintScreen),
        VK_PAUSE => Some(Key::Pause),
        VK_NUMPAD0 => Some(Key::Keypad0),
        VK_NUMPAD1 => Some(Key::Keypad1),
        VK_NUMPAD2 => Some(Key::Keypad2),
        VK_NUMPAD3 => Some(Key::Keypad3),
        VK_NUMPAD4 => Some(Key::Keypad4),
        VK_NUMPAD5 => Some(Key::Keypad5),
        VK_NUMPAD6 => Some(Key::Keypad6),
        VK_NUMPAD7 => Some(Key::Keypad7),
        VK_NUMPAD8 => Some(Key::Keypad8),
        VK_NUMPAD9 => Some(Key::Keypad9),
        VK_DECIMAL => Some(Key::KeypadDecimal),
        VK_DIVIDE => Some(Key::KeypadDivide),
        VK_MULTIPLY => Some(Key::KeypadMultiply),
        VK_SUBTRACT => Some(Key::KeypadSubtract),
        VK_ADD => Some(Key::KeypadAdd),
        VK_LSHIFT => Some(Key::LeftShift),
        VK_LCONTROL | VK_CONTROL => Some(Key::LeftCtrl),
        VK_RCONTROL => Some(Key::RightCtrl),
        VK_LMENU => Some(Key::LeftAlt),
        VK_LWIN => Some(Key::LeftSuper),
        VK_RSHIFT => Some(Key::RightShift),
        VK_RMENU => Some(Key::RightAlt),
        VK_RWIN => Some(Key::RightSuper),
        VK_APPS => Some(Key::Menu),
        VK_0 => Some(Key::Alpha0),
        VK_1 => Some(Key::Alpha1),
        VK_2 => Some(Key::Alpha2),
        VK_3 => Some(Key::Alpha3),
        VK_4 => Some(Key::Alpha4),
        VK_5 => Some(Key::Alpha5),
        VK_6 => Some(Key::Alpha6),
        VK_7 => Some(Key::Alpha7),
        VK_8 => Some(Key::Alpha8),
        VK_9 => Some(Key::Alpha9),
        VK_A => Some(Key::A),
        VK_B => Some(Key::B),
        VK_C => Some(Key::C),
        VK_D => Some(Key::D),
        VK_E => Some(Key::E),
        VK_F => Some(Key::F),
        VK_G => Some(Key::G),
        VK_H => Some(Key::H),
        VK_I => Some(Key::I),
        VK_J => Some(Key::J),
        VK_K => Some(Key::K),
        VK_L => Some(Key::L),
        VK_M => Some(Key::M),
        VK_N => Some(Key::N),
        VK_O => Some(Key::O),
        VK_P => Some(Key::P),
        VK_Q => Some(Key::Q),
        VK_R => Some(Key::R),
        VK_S => Some(Key::S),
        VK_T => Some(Key::T),
        VK_U => Some(Key::U),
        VK_V => Some(Key::V),
        VK_W => Some(Key::W),
        VK_X => Some(Key::X),
        VK_Y => Some(Key::Y),
        VK_Z => Some(Key::Z),
        VK_F1 => Some(Key::F1),
        VK_F2 => Some(Key::F2),
        VK_F3 => Some(Key::F3),
        VK_F4 => Some(Key::F4),
        VK_F5 => Some(Key::F5),
        VK_F6 => Some(Key::F6),
        VK_F7 => Some(Key::F7),
        VK_F8 => Some(Key::F8),
        VK_F9 => Some(Key::F9),
        VK_F10 => Some(Key::F10),
        VK_F11 => Some(Key::F11),
        VK_F12 => Some(Key::F12),
        _ => None,
    }
}
// Function to convert imgui::Key to VIRTUAL_KEY
pub fn to_win_key(key: Key) -> VIRTUAL_KEY {
    match key {
        Key::Tab => VK_TAB,
        Key::LeftArrow => VK_LEFT,
        Key::RightArrow => VK_RIGHT,
        Key::UpArrow => VK_UP,
        Key::DownArrow => VK_DOWN,
        Key::PageUp => VK_PRIOR,
        Key::PageDown => VK_NEXT,
        Key::Home => VK_HOME,
        Key::End => VK_END,
        Key::Insert => VK_INSERT,
        Key::Delete => VK_DELETE,
        Key::Backspace => VK_BACK,
        Key::Space => VK_SPACE,
        Key::Enter => VK_RETURN,
        Key::Escape => VK_ESCAPE,
        Key::Apostrophe => VK_OEM_7,
        Key::Comma => VK_OEM_COMMA,
        Key::Minus => VK_OEM_MINUS,
        Key::Period => VK_OEM_PERIOD,
        Key::Slash => VK_OEM_2,
        Key::Semicolon => VK_OEM_1,
        Key::Equal => VK_OEM_PLUS,
        Key::LeftBracket => VK_OEM_4,
        Key::Backslash => VK_OEM_5,
        Key::RightBracket => VK_OEM_6,
        Key::GraveAccent => VK_OEM_3,
        Key::CapsLock => VK_CAPITAL,
        Key::ScrollLock => VK_SCROLL,
        Key::NumLock => VK_NUMLOCK,
        Key::PrintScreen => VK_SNAPSHOT,
        Key::Pause => VK_PAUSE,
        Key::Keypad0 => VK_NUMPAD0,
        Key::Keypad1 => VK_NUMPAD1,
        Key::Keypad2 => VK_NUMPAD2,
        Key::Keypad3 => VK_NUMPAD3,
        Key::Keypad4 => VK_NUMPAD4,
        Key::Keypad5 => VK_NUMPAD5,
        Key::Keypad6 => VK_NUMPAD6,
        Key::Keypad7 => VK_NUMPAD7,
        Key::Keypad8 => VK_NUMPAD8,
        Key::Keypad9 => VK_NUMPAD9,
        Key::KeypadDecimal => VK_DECIMAL,
        Key::KeypadDivide => VK_DIVIDE,
        Key::KeypadMultiply => VK_MULTIPLY,
        Key::KeypadSubtract => VK_SUBTRACT,
        Key::KeypadAdd => VK_ADD,
        Key::LeftShift => VK_LSHIFT,
        Key::LeftCtrl => VK_LCONTROL,
        Key::RightCtrl => VK_RCONTROL,
        Key::LeftAlt => VK_LMENU,
        Key::LeftSuper => VK_LWIN,
        Key::RightShift => VK_RSHIFT,
        Key::RightAlt => VK_RMENU,
        Key::RightSuper => VK_RWIN,
        Key::Menu => VK_APPS,
        Key::Alpha0 => VK_0,
        Key::Alpha1 => VK_1,
        Key::Alpha2 => VK_2,
        Key::Alpha3 => VK_3,
        Key::Alpha4 => VK_4,
        Key::Alpha5 => VK_5,
        Key::Alpha6 => VK_6,
        Key::Alpha7 => VK_7,
        Key::Alpha8 => VK_8,
        Key::Alpha9 => VK_9,
        Key::A => VK_A,
        Key::B => VK_B,
        Key::C => VK_C,
        Key::D => VK_D,
        Key::E => VK_E,
        Key::F => VK_F,
        Key::G => VK_G,
        Key::H => VK_H,
        Key::I => VK_I,
        Key::J => VK_J,
        Key::K => VK_K,
        Key::L => VK_L,
        Key::M => VK_M,
        Key::N => VK_N,
        Key::O => VK_O,
        Key::P => VK_P,
        Key::Q => VK_Q,
        Key::R => VK_R,
        Key::S => VK_S,
        Key::T => VK_T,
        Key::U => VK_U,
        Key::V => VK_V,
        Key::W => VK_W,
        Key::X => VK_X,
        Key::Y => VK_Y,
        Key::Z => VK_Z,
        Key::F1 => VK_F1,
        Key::F2 => VK_F2,
        Key::F3 => VK_F3,
        Key::F4 => VK_F4,
        Key::F5 => VK_F5,
        Key::F6 => VK_F6,
        Key::F7 => VK_F7,
        Key::F8 => VK_F8,
        Key::F9 => VK_F9,
        Key::F10 => VK_F10,
        Key::F11 => VK_F11,
        Key::F12 => VK_F12,
        _ => VK__none_, // Fallback for unmapped keys
    }
}

fn handle_key_modifier(io: &mut imgui::Io, key: VIRTUAL_KEY, down: bool) {
    if key == VK_LSHIFT || key == VK_RSHIFT {
        io.add_key_event(imgui::Key::ModShift, down);
    } else if key == VK_LCONTROL || key == VK_CONTROL {
        io.add_key_event(imgui::Key::ModCtrl, down);
    } else if key == VK_MENU || key == VK_LMENU || key == VK_RMENU {
        io.add_key_event(imgui::Key::ModAlt, down);
    } else if key == VK_LWIN || key == VK_RWIN {
        io.add_key_event(imgui::Key::ModSuper, down);
    }
}
