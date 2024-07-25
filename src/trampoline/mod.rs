use std::{error, fmt, result};
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::{copy_nonoverlapping, null_mut, write_bytes};
use hudhook::windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE, PAGE_WRITECOPY, VirtualAlloc, VirtualFree, VirtualProtect, VirtualQueryEx};
use windows::Win32::System::Threading::GetCurrentProcess;

#[cfg(target_pointer_width = "32")]
const JMP_SIZE: usize = 5;

#[cfg(target_pointer_width = "64")]
const JMP_SIZE: usize = 14;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ToSmall,
    InvalidTarget,
    //Windows(windows::Error),
    TooSmall,
    NoFreeCave,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ToSmall => write!(f, "value to small"),
            Error::InvalidTarget => write!(f, "invalid target"),
            Error::TooSmall => write!(f, "value to small"),
            Error::NoFreeCave => write!(f, "no free cave"),
            //Error::Windows(ref err) => write!(f, "windows api failed '{}'", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ToSmall => None,
            Error::InvalidTarget => None,
            Error::TooSmall => None,
            Error::NoFreeCave => None,
            //Error::Windows(ref err) => Some(err),
        }
    }
}

/*impl From<windows::Error> for Error {
    fn from(err: windows::Error) -> Self {
        Error::Windows(err)
    }
}
*/


/// A 32 or 64 bit hook.
///
/// After creating a `Hook` by [`hook`]ing a function, it redirects the control flow.
///
/// The function will be unhooked when the value is dropped.
///
/// [`hook`]: #method.hook
///
/// # Examples
///
/// ```no_run
/// use crate::bindings::Windows::Win32::Foundation::{HANDLE, BOOL};
/// use crate::bindings::Windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
/// use std::ffi::c_void;
/// use std::mem::transmute;
/// use trampoline::Hook;
///
/// mod bindings {
///     windows::include_bindings!();
/// }
///
/// pub extern "stdcall" fn wgl_swap_buffers(hdc: HANDLE) -> BOOL {
///     BOOL::from(true)
/// }
///
/// fn main() {
///     let module = unsafe { GetModuleHandleA("opengl32.dll") };
///     let src_wgl_swap_buffers = unsafe {
///         GetProcAddress(module, "wglSwapBuffers")
///     }.unwrap();
///
///     let hook = Hook::hook(
///         src_wgl_swap_buffers as *mut c_void,
///         wgl_swap_buffers as *mut c_void,
///         21
///     ).unwrap();
/// }
/// ```
pub struct Hook {
    src: *mut c_void,
    len: usize,
    orig_bytes: Vec<u8>,
    active: bool,
}


/// A 32 or 64 bit trampoline hook.
///
/// After creating a `TrampolineHook` by [`hook`]ing a function, it redirects the control flow.
///
/// The function will be unhooked when the value is dropped.
///
/// [`hook`]: #method.hook
///
/// # Examples
///
/// ```no_run
/// use crate::bindings::Windows::Win32::Foundation::{HANDLE, BOOL};
/// use crate::bindings::Windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
/// use std::ffi::c_void;
/// use std::sync::Mutex;
/// use std::mem::transmute;
/// use once_cell::sync::Lazy;
/// use trampoline::TrampolineHook;
///
/// mod bindings {
///     windows::include_bindings!();
/// }
///
/// static HOOK: Lazy<Mutex<Option<TrampolineHook>>> = Lazy::new(|| {
///     Mutex::new(None)
/// });
///
/// pub extern "stdcall" fn wgl_swap_buffers(hdc: HANDLE) -> BOOL {
///     let gateway = HOOK
///         .lock()
///         .unwrap()
///         .as_ref()
///         .unwrap()
///         .gateway();
///
///     let gateway_call: extern "stdcall" fn(hdc: HANDLE) -> BOOL;
///     gateway_call = unsafe { transmute(gateway) };
///     gateway_call(hdc);
///
///     BOOL::from(true)
/// }
///
/// fn main() {
///     let module = unsafe { GetModuleHandleA("opengl32.dll") };
///     let src_wgl_swap_buffers = unsafe {
///         GetProcAddress(module, "wglSwapBuffers")
///     }.unwrap();
///
///     let hook = TrampolineHook::hook(
///         src_wgl_swap_buffers as *mut c_void,
///         wgl_swap_buffers as *mut c_void,
///         21
///     ).unwrap();
///
///     *HOOK
///         .lock()
///         .unwrap() = Some(hook);
/// }
/// ```
pub struct TrampolineHook {
    gateway: *mut c_void,
    hook: Hook,
}

impl Hook {
    /// Hooks a function.
    ///
    /// `src` is the function to be hooked.
    ///
    /// `dst` is the destination of the hook.
    ///
    /// `len` is the amount of bytes that should be overridden.
    pub fn hook(src: *mut c_void, dst: *mut c_void, len: usize) -> Result<Self> {
        if len < JMP_SIZE {
            return Err(Error::ToSmall);
        }

        let mut protection = PAGE_PROTECTION_FLAGS::default();

        unsafe {
            VirtualProtect(
                src,
                len,
                PAGE_EXECUTE_READWRITE,
                &mut protection,
            )
        }.ok().unwrap();

        let mut orig_bytes: Vec<u8> = vec![0x90; len];
        unsafe { copy_nonoverlapping(src, orig_bytes.as_mut_ptr() as *mut c_void, len); }
        unsafe { write_bytes(src, 0x90, len); }

        if cfg!(target_pointer_width = "32") {
            unsafe { *(src as *mut usize) = 0xE9; }
            unsafe {
                *(((src as *mut usize) as usize + 1) as *mut usize) =
                    (((dst as *mut isize) as isize - (src as *mut isize) as isize) - 5) as usize;
            }
        } else if cfg!(target_pointer_width = "64") {
            let mut jmp_bytes: [u8; 14] = [
                0xFF, 0x25, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ];

            let jmp_bytes_ptr = jmp_bytes.as_mut_ptr() as *mut c_void;

            unsafe {
                copy_nonoverlapping(
                    (&(dst as usize) as *const usize) as *mut c_void,
                    jmp_bytes_ptr.offset(6),
                    8,
                );
            }

            unsafe { copy_nonoverlapping(jmp_bytes_ptr, src, JMP_SIZE); }
        } else {
            return Err(Error::InvalidTarget);
        }

        unsafe {
            VirtualProtect(
                src,
                len,
                protection,
                &mut protection,
            )
        }.ok().unwrap();

        Ok(Self { src, len, orig_bytes, active: true })
    }

    /// Unhooks the function.
    pub fn unhook(&mut self) -> Result<()> {
        if !self.active {
            return Ok(());
        }

        let mut protection = PAGE_PROTECTION_FLAGS::default();

        unsafe {
            VirtualProtect(
                self.src,
                self.len,
                PAGE_EXECUTE_READWRITE,
                &mut protection,
            )
        }.ok().unwrap();

        unsafe {
            copy_nonoverlapping(
                self.orig_bytes.as_ptr() as *mut c_void,
                self.src,
                self.len,
            );
        }

        unsafe {
            VirtualProtect(
                self.src,
                self.len,
                protection,
                &mut protection,
            )
        }.ok().unwrap();

        self.active = false;
        Ok(())
    }

    /// Returns the state of this hook.
    pub fn active(&self) -> bool {
        self.active
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        let _ = self.unhook();
    }
}

unsafe impl Sync for Hook {}
unsafe impl Send for Hook {}






// Function to find a free code cave
unsafe fn find_free_code_cave(size: usize) -> Option<*mut c_void> {
    let mut address: *mut c_void = null_mut();

    loop {
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();

        // Query memory information for the current address
        let result: usize = VirtualQueryEx(
            GetCurrentProcess(),
            Option::from(address as *const c_void),
            &mut mbi,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        );

        if result == 0 {
            break; // No more memory to query
        }

        // Check if the memory region is committed and has the right permissions
        if mbi.State == MEM_COMMIT && (mbi.Protect & (PAGE_EXECUTE_READWRITE | PAGE_READWRITE | PAGE_WRITECOPY)) != PAGE_PROTECTION_FLAGS(0) {
            // Calculate the start and end of the region
            let region_size = mbi.RegionSize;
            let base_address = mbi.BaseAddress as usize;

            // Ensure the region is large enough for the cave
            if region_size < size {
                address = (base_address + region_size) as *mut c_void; // Move to next region
                continue;
            }

            // Scan the region for NOPs (0x90)
            let mut nops_count = 0;

            for offset in 0..region_size {
                let current_address = (base_address + offset) as *const u8;

                // Read the byte at the current address
                let byte = *current_address;

                if byte == 0x90 { // NOP found
                    nops_count += 1;
                } else {
                    nops_count = 0; // Reset count if we hit a non-NOP
                }

                // Check if we have found enough NOPs
                if nops_count >= size {
                    // Ensure the cave does not overlap existing code
                    let cave_start = (current_address as usize) - (nops_count - size);
                    let cave_end = cave_start + size;

                    if cave_start >= base_address && cave_end <= (base_address + region_size) {
                        return Some(cave_start as *mut c_void);
                    }
                }
            }
        }

        // Move to the next memory region
        address = (mbi.BaseAddress as usize + mbi.RegionSize) as *mut c_void;
    }

    None
}

// Function to set up a jump back to the original function
unsafe fn setup_jump(gateway: *mut c_void, target: *mut c_void, len: usize) {
    let jump_offset = target as isize - (gateway as isize + len as isize + JMP_SIZE as isize);

    // Write the jump instruction
    *(gateway.add(len) as *mut u8) = 0xE9; // JMP opcode
    *(gateway.add(len + 1) as *mut isize) = jump_offset; // Offset
}




impl TrampolineHook {
    /// Hooks a function and allocates a gateway with the overridden bytes.
    ///
    /// `src` is the function to be hooked.
    ///
    /// `dst` is the destination of the hook.
    ///
    /// `len` is the amount of bytes that should be overridden.
    pub fn hook(src: *mut c_void, dst: *mut c_void, len: usize) -> Result<Self> {
        if len < JMP_SIZE {
            return Err(Error::TooSmall);
        }

        // Check for existing code cave
        let gateway = unsafe {
            find_free_code_cave(len + JMP_SIZE).ok_or(Error::NoFreeCave)?
        };

        // Copy original bytes to the gateway
        unsafe { copy_nonoverlapping(src, gateway, len); }

        // Set up the jump back to the original function
        unsafe { setup_jump(gateway, src, len); }

        // Create the hook
        let hook = Hook::hook(src, dst, len)?;
        Ok(Self { gateway, hook })
    }

    /// Unhooks the function and deallocates the gateway.
    pub fn unhook(&mut self) -> Result<()> {
        if !self.active() {
            return Ok(());
        }

        unsafe {
            VirtualFree(self.gateway, 0, MEM_RELEASE)
                .expect("Failed to deallocate gateway");
        }
        self.hook.unhook()?;
        Ok(())
    }

    /// Returns the state of this hook.
    pub fn active(&self) -> bool {
        self.hook.active()
    }

    /// Returns the allocated gateway of this hook.
    pub fn gateway(&self) -> *mut c_void {
        self.gateway
    }
}

impl Drop for TrampolineHook {
    fn drop(&mut self) {
        let _ = self.unhook();
    }
}

unsafe impl Sync for TrampolineHook {}
unsafe impl Send for TrampolineHook {}