use std::ffi::{c_void, CString};

use hudhook::{eject, Hudhook};
use hudhook::hooks::opengl3::ImguiOpenGl3Hooks;
use hudhook::mh::{MH_Initialize, MH_STATUS};
use hudhook::windows::Win32::Foundation::HINSTANCE;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Threading::Sleep;

use crate::esp::esp_entrypoint;
use crate::ui::RenderLoop;
use crate::utils::setup_tracing;
use crate::vars::handles::{CHEAT_DLL_HMODULE, OPENGL32_DLL_HMODULE};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn MainThread(lpReserved: *mut c_void) -> u32 {
    unsafe {
        setup_tracing();

        let module_name = CString::new("OPENGL32.dll").unwrap();
        let swapbuffers_name = CString::new("wglSwapBuffers").unwrap();
        let module_name_pcstr = PCSTR(module_name.as_ptr() as *const u8);
        let swapbuffers_name_pcstr = PCSTR(swapbuffers_name.as_ptr() as *const u8);
        OPENGL32_DLL_HMODULE = GetModuleHandleA(module_name_pcstr).unwrap();
        while OPENGL32_DLL_HMODULE.0.is_null() {
            OPENGL32_DLL_HMODULE = GetModuleHandleA(module_name_pcstr).unwrap();
            println!("[MainThread] OPENGL32.dll isn't initialized, waiting for 2 sec.");
            Sleep(2000);
        }
        println!("[MainThread] Found OPENGL32.dll.");
        println!("[MainThread] OPENGL32_DLL_HMODULE is: {:?}", OPENGL32_DLL_HMODULE.0);
        if let Err(e) = MH_Initialize().ok()
        {
            if e == MH_STATUS::MH_OK
            {
                println!("[MainThread] MinHook has been initialized. MH_STATUS is: {:?}", e);
            } else {
                println!("[MainThread] MinHook failed to initialize. MH_STATUS is: {:?}", e);
            }
        }
        if let Err(e) = Hudhook::builder()
            .with::<ImguiOpenGl3Hooks>(RenderLoop)
            .with_hmodule(HINSTANCE(CHEAT_DLL_HMODULE))
            .build()
            .apply()
        {
            eject();

            println!("[MainThread] HudHook has been ejected. MH_STATUS is: {:?}", e);
        } else {
            println!("[MainThread] HudHook has been injected.");
        }
        esp_entrypoint().expect("[MainThread] Failed to call esp_entrypoint()");
    }
    return 1;
}