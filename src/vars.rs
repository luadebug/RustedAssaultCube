pub mod hooks {
    use std::sync::Mutex;

    use ilhook::x86::HookPoint;
    use once_cell::sync::Lazy;

    pub static mut WALLHACK_HOOK: Lazy<Mutex<Option<HookPoint>>> = Lazy::new(|| Mutex::new(None));

    pub static mut TRIGGERBOT_HOOK: Lazy<Mutex<Option<HookPoint>>> = Lazy::new(|| Mutex::new(None));

    pub static mut LOCAL_PLAYER_HOOK: Lazy<Mutex<Option<HookPoint>>> =
        Lazy::new(|| Mutex::new(None));
}
pub mod mem_patches {
    use crate::memorypatch::MemoryPatch;

    pub static mut NO_RECOIL_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
    pub static mut RAPID_FIRE_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
    pub static mut MAPHACK_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
    pub static mut RADAR_MEMORY_PATCH: MemoryPatch = MemoryPatch::new_empty();
}
pub mod ui_vars {
    use std::sync::atomic::AtomicBool;
    pub static mut IS_WALLHACK: AtomicBool = AtomicBool::new(false);
    pub static mut IS_SHOW_UI: AtomicBool = AtomicBool::new(false);
    pub static mut IS_ESP: AtomicBool = AtomicBool::new(true);
    /*pub static mut IS_ESP: bool = true;*/
    pub static mut IS_GRENADES_INFINITE: AtomicBool = AtomicBool::new(false);
    pub static mut IS_NO_RELOAD: AtomicBool = AtomicBool::new(false);
    pub static mut IS_INVULNERABLE: AtomicBool = AtomicBool::new(false);
    pub static mut IS_INFINITE_AMMO: AtomicBool = AtomicBool::new(false);
    pub static mut IS_NO_RECOIL: AtomicBool = AtomicBool::new(false);
    pub static mut IS_RAPID_FIRE: AtomicBool = AtomicBool::new(false);
    pub static mut IS_DRAW_FOV: AtomicBool = AtomicBool::new(false);
    pub static mut IS_SMOOTH: AtomicBool = AtomicBool::new(false);
    pub static mut IS_AIMBOT: AtomicBool = AtomicBool::new(false);
    pub static mut IS_TRIGGERBOT: AtomicBool = AtomicBool::new(false);
    pub static mut IS_MAPHACK: AtomicBool = AtomicBool::new(false);
    pub static mut IS_FULLBRIGHT: AtomicBool = AtomicBool::new(false);
    //pub static mut IS_FULLBRIGHT: bool = false;
}
pub mod handles {
    use std::ffi::c_void;

    use windows::Win32::Foundation::{HMODULE, HWND};

    use crate::window_dimensions::WindowDimensions;

    pub static mut CHEAT_DLL_HMODULE: isize = 0isize;
    pub static mut OPENGL32_DLL_HMODULE: HMODULE = HMODULE(0 as _);
    pub static mut AC_CLIENT_EXE_HMODULE: usize = 0usize;
    pub static mut GAME_WINDOW_HANDLE: HWND = HWND(0 as *mut c_void);
    pub static mut GAME_WINDOW_DIMENSIONS: WindowDimensions = WindowDimensions {
        width: 0,
        height: 0,
    };
}
pub mod game_vars {
    use std::ptr::null_mut;
    use std::sync::atomic::AtomicU32;

    use crate::entity::Entity;

    pub static mut VIEW_MATRIX: [f32; 16] = [0.0; 16];
    /*pub static mut VIEW_MATRIX: *mut [f32; 16] = null_mut();*/
    pub static mut LOCAL_PLAYER: Entity = Entity {
        entity_starts_at_addr: 0,
    };
    pub static mut NUM_PLAYERS_IN_MATCH: usize = 0;
    pub static mut ENTITY_LIST_PTR: usize = 0;
    pub static mut FOV: AtomicU32 = AtomicU32::new(300);
    pub static mut SMOOTH: AtomicU32 = AtomicU32::new(100);
    pub static mut TRIGGER_DELAY: AtomicU32 = AtomicU32::new(100);
    pub static mut CURRENT_CROSSHAIR_ENTITY_ADDR: *mut usize = null_mut();
}
/*pub mod hotkeys
{

    use std::sync::atomic::AtomicU16;


    pub static mut AIM_KEY: AtomicU16 = AtomicU16::new(67);


/*    use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_C}

    pub static mut aim_key: VIRTUAL_KEY = VK_C;;*/
}*/
