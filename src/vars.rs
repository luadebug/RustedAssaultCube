pub mod hooks
{
    use std::sync::Mutex;

    use ilhook::x86::HookPoint;
    use once_cell::sync::Lazy;

    pub static mut TRIGGERBOT_HOOK: Lazy<Mutex<Option<HookPoint>>> = Lazy::new(|| {
        Mutex::new(None)
    });

    pub static mut LOCAL_PLAYER_HOOK: Lazy<Mutex<Option<HookPoint>>> = Lazy::new(|| {
        Mutex::new(None)
    });

}
pub mod mem_patches
{
    use crate::memorypatch::MemoryPatch;

    pub static mut NO_RECOIL_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
    pub static mut RAPID_FIRE_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
    pub static mut MAPHACK_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
}
pub mod ui_vars
{
    pub static mut IS_SHOW_UI: bool = true;
    pub static mut IS_ESP: bool = true;
    pub static mut IS_GRENADES_INFINITE: bool = false;
    pub static mut IS_NO_RELOAD: bool = false;
    pub static mut IS_INVULNERABLE: bool = false;
    pub static mut IS_INFINITE_AMMO: bool = false;
    pub static mut IS_NO_RECOIL: bool = false;
    pub static mut IS_RAPID_FIRE: bool = false;
    pub static mut IS_DRAW_FOV: bool = false;
    pub static mut IS_SMOOTH: bool = false;
    pub static mut IS_AIMBOT: bool = false;
    pub static mut IS_TRIGGERBOT: bool = false;
    pub static mut IS_MAPHACK: bool = false;
    pub static mut IS_FULLBRIGHT: bool = false;
}
pub mod handles
{
    use std::ffi::c_void;

    use windows::Win32::Foundation::{HMODULE, HWND};

    use crate::window_dimensions::WindowDimensions;

    pub static mut CHEAT_DLL_HMODULE: isize = 0isize;
    pub static mut OPENGL32_DLL_HMODULE: HMODULE = HMODULE(0 as _);
    pub static mut AC_CLIENT_EXE_HMODULE: usize = 0usize;
    pub static mut GAME_WINDOW_HANDLE: HWND = HWND(0 as *mut c_void);
    pub static mut GAME_WINDOW_DIMENSIONS: WindowDimensions = WindowDimensions { width: 0, height: 0 };
}
pub mod game_vars
{
    use std::ptr::null_mut;

    use crate::entity::Entity;

    pub static mut VIEW_MATRIX: *mut [f32; 16] = null_mut();
    pub static mut LOCAL_PLAYER: Entity = Entity { entity_starts_at_addr: 0 };
    pub static mut NUM_PLAYERS_IN_MATCH: usize = 0;
    pub static mut ENTITY_LIST_PTR: u32 = 0;
    pub static mut FOV: f32 = 300.0;
    pub static mut SMOOTH: f32 = 100.0;
    pub static mut TRIGGER_DELAY: f32 = 100.0;

    pub static mut CURRENT_CROSSHAIR_ENTITY_ADDR: * mut usize = null_mut();
}
pub mod hotkeys
{
    use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_C};

    pub static mut AIM_KEY: VIRTUAL_KEY = VK_C;
}

