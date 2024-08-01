use std::arch::asm;
use std::ffi::CString;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

use ilhook::x86::{CallbackOption, HookFlags, HookType, Hooker, Registers};
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetProcAddress;

use crate::vars::handles::OPENGL32_DLL_HMODULE;
use crate::vars::hooks::WALLHACK_HOOK;
use crate::vars::ui_vars::IS_WALLHACK;

static mut GL_DEPTH_FUNC_FN: Option<unsafe extern "stdcall" fn(usize) -> ()> = None;

/*static mut GL_COLOR4F_FN: Option<unsafe extern "stdcall" fn(f32, f32, f32, f32) -> ()> = None;
static mut GL_ENABLE_FN: Option<unsafe extern "stdcall" fn(u32) -> ()> = None;
static mut GL_DISABLE_FN: Option<unsafe extern "stdcall" fn(u32) -> ()> = None;
static mut GL_ENABLE_CLIENT_STATE_FN: Option<unsafe extern "stdcall" fn(u32) -> ()> = None;
static mut GL_DISABLE_CLIENT_STATE_FN: Option<unsafe extern "stdcall" fn(u32) -> ()> = None;

static mut GL_DEPTH_RANGE_FN: Option<unsafe extern "stdcall" fn(f64, f64) -> ()> = None;
*/

#[inline(never)]
pub(crate) unsafe extern "cdecl" fn wallhack_hooked_func(reg: *mut Registers, _: usize) {
    unsafe {
        if let Some(reg_val) = reg.as_ref() {
            if reg_val.esi == 0 {
                return;
            }
            if reg_val.esp == 0 {
                return;
            }
            asm! {
            "pushad",
            }
            if IS_WALLHACK.load(SeqCst) {
                if (*reg).get_arg(7) > 200 && (*reg).get_arg(7) < 10000 {
                    //println!("[ESP+0x1C] = {}", (*reg).get_arg(7));

                    /*GL_DEPTH_RANGE_FN.unwrap()(0.0f64, 0.0f64);*/

                    GL_DEPTH_FUNC_FN.unwrap()(0x207);
                    /*                    GL_DISABLE_CLIENT_STATE_FN.unwrap()(0x8078);
                    GL_DISABLE_CLIENT_STATE_FN.unwrap()(0x8076);
                    GL_ENABLE_FN.unwrap()(0x0B57);
                    GL_COLOR4F_FN.unwrap()(1.0f32, 0.6f32, 0.6f32, 1.0f32);*/
                }
                /*                else
                                {
                /*                    GL_DEPTH_RANGE_FN.unwrap()(0.0f64, 1.0f64);*/

                                    GL_DEPTH_FUNC_FN.unwrap()(0x203);
                /*                    GL_ENABLE_CLIENT_STATE_FN.unwrap()(0x8078);
                                    GL_ENABLE_CLIENT_STATE_FN.unwrap()(0x8076);
                                    GL_DISABLE_FN.unwrap()(0x0B57);
                                    GL_COLOR4F_FN.unwrap()(1.0f32, 1.0f32, 1.0f32, 1.0f32);*/
                                }*/
            }
            asm! {
            "popad
            mov esi, dword ptr ds : [esi + 0xA18]",
            }
        }
    }
}

// Example of finding a pattern and setting up the hook
pub fn setup_wallhack() {
    thread::spawn(|| unsafe {
        let gl_depth_func_cstring = match CString::new("glDepthFunc") {
            Ok(cstring) => cstring,
            Err(err) => {
                println!("Error creating CString: {}", err);
                return;
            }
        };
        let gl_depth_func = GetProcAddress(
            OPENGL32_DLL_HMODULE,
            PCSTR(gl_depth_func_cstring.as_ptr() as *const u8),
        );

        println!(
            "[wallhack_hook.rs->setup_wallhack] glDepthFunc found at: {:#x}",
            gl_depth_func.unwrap() as usize
        );

        GL_DEPTH_FUNC_FN = core::mem::transmute(gl_depth_func);

        /*

                //############################################################################//
                let gl_color4f_cstring = match CString::new("glColor4f") {
                    Ok(cstring) => cstring,
                    Err(err) => {
                        println!("Error creating CString: {}", err);
                        return;
                    }
                };

                let gl_color4f = GetProcAddress(OPENGL32_DLL_HMODULE,
                                                PCSTR(gl_color4f_cstring.as_ptr() as *const u8));

                println!("[wallhack_hook.rs->setup_chams] gl_color4f found at: {:#x}",
                         gl_color4f.unwrap() as usize);


                GL_COLOR4F_FN = core::mem::transmute(gl_color4f);
                //############################################################################//
                let gl_enable_fn_cstring = match CString::new("glEnable") {
                    Ok(cstring) => cstring,
                    Err(err) => {
                        println!("Error creating CString: {}", err);
                        return;
                    }
                };

                let gl_enable = GetProcAddress(OPENGL32_DLL_HMODULE,
                                               PCSTR(gl_enable_fn_cstring.as_ptr() as *const u8));

                println!("[wallhack_hook.rs->setup_chams] gl_enable found at: {:#x}",
                         gl_enable.unwrap() as usize);


                GL_ENABLE_FN = core::mem::transmute(gl_enable);
                //############################################################################//
                let gl_disable_fn_cstring = match CString::new("glDisable") {
                    Ok(cstring) => cstring,
                    Err(err) => {
                        println!("Error creating CString: {}", err);
                        return;
                    }
                };

                let gl_disable = GetProcAddress(OPENGL32_DLL_HMODULE,
                                                PCSTR(gl_disable_fn_cstring.as_ptr() as *const u8));

                println!("[wallhack_hook.rs->setup_chams] gl_disable found at: {:#x}",
                         gl_disable.unwrap() as usize);


                GL_DISABLE_FN = core::mem::transmute(gl_disable);
                //############################################################################//
                let gl_enable_client_state_fn_cstring = match CString::new("glEnableClientState") {
                    Ok(cstring) => cstring,
                    Err(err) => {
                        println!("Error creating CString: {}", err);
                        return;
                    }
                };

                let gl_enable_client_state = GetProcAddress(OPENGL32_DLL_HMODULE,
                                                            PCSTR(gl_enable_client_state_fn_cstring.as_ptr() as *const u8));

                println!("[wallhack_hook.rs->setup_chams] gl_enable_client_state found at: {:#x}",
                         gl_enable_client_state.unwrap() as usize);


                GL_ENABLE_CLIENT_STATE_FN = core::mem::transmute(gl_enable_client_state);
                //############################################################################//
                let gl_disable_client_state_fn_cstring = match CString::new("glDisableClientState") {
                    Ok(cstring) => cstring,
                    Err(err) => {
                        println!("Error creating CString: {}", err);
                        return;
                    }
                };

                let gl_disable_client_state = GetProcAddress(OPENGL32_DLL_HMODULE,
                                                             PCSTR(gl_disable_client_state_fn_cstring.as_ptr() as *const u8));

                println!("[wallhack_hook.rs->setup_chams] gl_disable_client_state found at: {:#x}",
                         gl_disable_client_state.unwrap() as usize);


                GL_DISABLE_CLIENT_STATE_FN = core::mem::transmute(gl_disable_client_state);
                //############################################################################//
                let gl_depth_range_fn_cstring = match CString::new("gl_depth_range") {
                    Ok(cstring) => cstring,
                    Err(err) => {
                        println!("Error creating CString: {}", err);
                        return;
                    }
                };

                let gl_depth_range = GetProcAddress(OPENGL32_DLL_HMODULE,
                                                    PCSTR(gl_depth_range_fn_cstring.as_ptr() as *const u8));

                println!("[wallhack_hook.rs->setup_chams] gl_disable_client_state found at: {:#x}",
                         gl_depth_range.unwrap() as usize);


                GL_DEPTH_RANGE_FN = core::mem::transmute(gl_depth_range);
                //############################################################################//

        */

        let gl_draw_elements_cstring = match CString::new("glDrawElements") {
            Ok(cstring) => cstring,
            Err(err) => {
                println!("Error creating CString: {}", err);
                return;
            }
        };

        let gl_draw_elements: unsafe extern "system" fn() -> isize = GetProcAddress(
            OPENGL32_DLL_HMODULE,
            PCSTR(gl_draw_elements_cstring.as_ptr() as *const u8),
        )
        .unwrap();

        println!(
            "[wallhack_hook.rs->setup_wallhack] glDrawElements found at: {:#x}",
            gl_draw_elements as usize
        );
        let opengl_hook_location: usize = gl_draw_elements as usize + 0x16;

        println!(
            "[wallhack_hook.rs->setup_wallhack] glDrawElements found at: {:#x}",
            opengl_hook_location
        );
        let hooker = Hooker::new(
            opengl_hook_location,
            HookType::JmpBack(wallhack_hooked_func),
            CallbackOption::None,
            0,
            HookFlags::empty(),
        );
        let hook_res = hooker.hook();

        match hook_res {
            Ok(trampoline_hook) => {
                *WALLHACK_HOOK.lock().unwrap() = Some(trampoline_hook);
                println!("[wallhack_hook.rs->setup_wallhack] wallhack hook succeeded!");
            }
            Err(e) => {
                eprintln!(
                    "[wallhack_hook.rs->setup_wallhack] wallhack hook failed: {:?}",
                    e
                );
            }
        }
    });
}
