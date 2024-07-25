/*use std::arch::asm;
use std::ffi::c_void;
use std::mem::zeroed;
use std::ptr::addr_of_mut;
use hudhook::mh::{MH_CreateHook, MH_STATUS};
use crate::trampoline::{Error, TrampolineHook};
// Define the types for addresses
type Address = usize;

// Hook function prototype
extern "C" {
    fn ori_call_address() -> Address; // Original function address
    fn ori_jump_address(); // Address to jump back to
}

// To hold the address to jump back after the hook
static mut JUMP_BACK: Address = 0;
static mut hook_addr: *mut c_void = ori_call_address as *mut c_void;

// The function to be hooked
#[inline(never)]
pub(crate) unsafe extern "system" fn get_crosshair_entity() {
    unsafe
    {
        asm!(
        "mov {0}, eax", // move eax to currentCrossHairEntityAddr
        "add esp, 0x10", // adjust the stack
        "mov [esp + 0x10], eax", // store eax on the stack
        "jmp {1}", // jump back to the original code
        in(reg) addr_of_mut!(CURRENT_CROSSHAIR_ENTITY_ADDR),
        in(reg) JUMP_BACK,
        options(noreturn)
        );
    }
}
// Static variable to hold the current crosshair entity address
static mut CURRENT_CROSSHAIR_ENTITY_ADDR: Address = 0;

// Hook setup
unsafe fn setup_hook() {
    const STOLEN_BYTES_LEN: usize = 7; // Length of the bytes to replace


    // Set the jump back address
    JUMP_BACK = (hook_addr as Address + STOLEN_BYTES_LEN) as Address;

    // Perform the hook
    let trigger_bot_hook =
        TrampolineHook::hook(hook_addr, get_crosshair_entity as *mut c_void, STOLEN_BYTES_LEN).unwrap();
    if trigger_bot_hook.gateway() != 0 as *mut c_void {
        println!("Hook successful, currentCrossHairEntityAddr: {:#X} and status: {}",
                 CURRENT_CROSSHAIR_ENTITY_ADDR, trigger_bot_hook.active());
    }
    else {
        println!("Hook failed.");
    }
}
/*use std::any::Any;
use std::arch::asm;
use std::ffi::c_void;
use std::mem;
use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SendInput};
use crate::trampoline::TrampolineHook;
pub static mut ImplantTriggerBot: TrampolineHook = TrampolineHook::default();
// Assume these are defined somewhere in your code
extern "C" {
    fn ori_call_address() -> usize; // Function pointer to the original call
    fn ori_jump_address(); // Jump address to return to after executing codecave
}

unsafe fn implant()
{
    ImplantTriggerBot = TrampolineHook::hook(ori_call_address as *mut c_void, codecave as *mut c_void, 6).unwrap();
}
unsafe fn inimplant()
{
    ImplantTriggerBot.unhook().unwrap();
}

fn codecave() {
    unsafe {
        let mut edi_value: usize;

        // Call the original function and save the return value
        asm!(
        "call {0}",
        in(reg) ori_call_address,
        out("eax") edi_value,
        options(nostack)
        );

        // Check if the result of the call is not zero
        let mut input: INPUT = mem::zeroed();
        input.r#type.0 = 0; // Set the type for INPUT_MOUSE
        if edi_value != 0 {
            input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
            SendInput(&[input], mem::size_of::<INPUT>() as i32);//1, &mut input, mem::size_of::<INPUT>() as u32);
        } else {
            input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;
            SendInput(&[input], mem::size_of::<INPUT>() as i32);
        }

        // Restore registers and jump back to the original code
        asm!(
        "jmp {0}",
        in(reg) ori_jump_address,
        options(noreturn)
        );
    }
}*/*/
use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookType, Registers};

use crate::utils::find_pattern;
use crate::vars::game_vars::CURRENT_CROSSHAIR_ENTITY_ADDR;
use crate::vars::hooks::HOOK;

// The function to be hooked
#[inline(never)]
pub(crate) unsafe extern "cdecl" fn get_crosshair_entity(
    reg: *mut Registers,
    _: usize
) { //system
    // Temporary variable to hold the value of EAX
 /*   let eax_value: usize;

    asm!(
    "mov {0}, eax",                  // Move EAX to temporary variable
    "add esp, 0x10",                // Adjust the stack
    "mov [esp + 0x10], eax",        // Store EAX on the stack
    out(reg) eax_value,              // Output operand for the EAX value
    options(nostack)
    );*/
    println!("[triggerbot->get_crosshair_entity] HOOK has been called!");
    println!("[triggerbot->get_crosshair_entity] Assigning CURRENT_CROSSHAIR_ENTITY_ADDR: {:p}", CURRENT_CROSSHAIR_ENTITY_ADDR);
    CURRENT_CROSSHAIR_ENTITY_ADDR = (*reg).eax as *mut usize;
    // Log HOOK status


    // Debug output to check the stored address
    if !CURRENT_CROSSHAIR_ENTITY_ADDR.is_null() {
        println!("[triggerbot->get_crosshair_entity] After assembly, CURRENT_CROSSHAIR_ENTITY_ADDR: {:p}", CURRENT_CROSSHAIR_ENTITY_ADDR);
    } else {
        println!("[triggerbot->get_crosshair_entity] CURRENT_CROSSHAIR_ENTITY_ADDR is null!");
    }
}

// Example of finding a pattern and setting up the hook
pub fn setup() {
    unsafe {
        let trigger_bot = find_pattern("ac_client.exe",
                                       &[0x83, 0xC4, 0x10, 0x89, 0x44, 0x24, 0x10, 0x8B],
                                       "xxxxxxxx");

        if let Some(addr) = trigger_bot {
            println!("[triggerbot->setup] trigger bot pattern found at: {:#x}", addr);
            let hooker = Hooker::new(
                addr,
                HookType::JmpBack(get_crosshair_entity),
                CallbackOption::None,
                0,
                HookFlags::empty(),
            );
            let hook_res = hooker.hook();

        match hook_res {
            Ok(trampoline_hook) => {
                *HOOK.lock().unwrap() = Some(trampoline_hook);
                println!("[triggerbot->setup] trigger bot hook succeeded!");
            }
            Err(e) => {
                println!("[triggerbot->setup] trigger bot hook failed: {:?}", e);
            }
        }


        } else {
            println!("[triggerbot->setup] trigger bot pattern not found");
        }
    }
}