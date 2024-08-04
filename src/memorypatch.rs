use std::alloc::{alloc, dealloc, Layout};
use std::error::Error;
use std::ffi::c_void;
use std::mem;
use std::ptr;

use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE};

pub struct MemoryPatch {
    location: *mut c_void,
    patch_instructions: *mut u8,
    original_instructions: *mut u8,
    size_patch: usize,
    is_patched: bool,
}

impl MemoryPatch {
    // Constructor for creating an empty MemoryPatch
    pub const fn new_empty() -> Self {
        MemoryPatch {
            location: ptr::null_mut(),              // Initialize with null
            patch_instructions: ptr::null_mut(),    // Initialize with null
            original_instructions: ptr::null_mut(), // Initialize with null
            size_patch: 0,                          // No size initially
            is_patched: false,                      // Not patched yet
        }
    }
    // Constructor for patching with a buffer and size
    #[allow(unused)]
    pub fn new(
        buffer_to_patch: &[u8],
        size_buffer: usize,
        location: *mut c_void,
        size_location: usize,
    ) -> Result<Self, Box<dyn Error>> {
        if size_buffer > size_location {
            return Err("Error on MemoryPatch trying to write buffer bigger than expected".into());
        }

        // Allocate memory for patch instructions and original instructions
        let patch_instructions = unsafe {
            alloc(Layout::from_size_align(
                size_location,
                mem::align_of::<u8>(),
            )?)
        };
        let original_instructions = unsafe {
            alloc(Layout::from_size_align(
                size_location,
                mem::align_of::<u8>(),
            )?)
        };

        // Copy the buffer to patch and the original instructions
        unsafe {
            ptr::copy_nonoverlapping(buffer_to_patch.as_ptr(), patch_instructions, size_buffer);
            ptr::copy_nonoverlapping(location as *const u8, original_instructions, size_location);
            for i in size_buffer..size_location {
                *patch_instructions.add(i) = 0x90; // NOP
            }
        }

        Ok(MemoryPatch {
            location,
            patch_instructions,
            original_instructions,
            size_patch: size_location,
            is_patched: false,
        })
    }

    // Constructor for patching with a mask
    #[allow(unused)]
    pub fn new_with_mask(
        buffer_to_patch: &[u8],
        mask: &[u8],
        size_buffer: usize,
        location: *mut c_void,
    ) -> Result<Self, Box<dyn Error>> {
        // Allocate memory for patch instructions and original instructions
        let patch_instructions =
            unsafe { alloc(Layout::from_size_align(size_buffer, mem::align_of::<u8>())?) };
        let original_instructions =
            unsafe { alloc(Layout::from_size_align(size_buffer, mem::align_of::<u8>())?) };

        // Copy the buffer and the original instructions
        unsafe {
            ptr::copy_nonoverlapping(buffer_to_patch.as_ptr(), patch_instructions, size_buffer);
            ptr::copy_nonoverlapping(location as *const u8, original_instructions, size_buffer);
            for (i, &m) in mask.iter().enumerate().take(size_buffer) {
                if m == b'?' {
                    *patch_instructions.add(i) = *original_instructions.add(i);
                }
            }
        }

        Ok(MemoryPatch {
            location,
            patch_instructions,
            original_instructions,
            size_patch: size_buffer,
            is_patched: false,
        })
    }

    // Destructor to clean up
    pub fn cleanup(&mut self) {
        self.unpatch_memory().ok(); // Ignore any errors on unpatching
        unsafe {
            dealloc(
                self.patch_instructions,
                Layout::from_size_align(self.size_patch, mem::align_of::<u8>()).unwrap(),
            );
            dealloc(
                self.original_instructions,
                Layout::from_size_align(self.size_patch, mem::align_of::<u8>()).unwrap(),
            );
        }
    }

    // Method to patch memory
    pub fn patch_memory(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.is_patched {
            return Ok(true);
        }
        self.is_patched = true;
        self.change_protected_memory(self.location, self.patch_instructions, self.size_patch)
    }

    // Method to unpatch memory
    pub fn unpatch_memory(&mut self) -> Result<bool, Box<dyn Error>> {
        if !self.is_patched {
            return Ok(true);
        }
        self.is_patched = false;
        self.change_protected_memory(self.location, self.original_instructions, self.size_patch)
    }

    // Method to change memory protection
    fn change_protected_memory(
        &self,
        target: *mut c_void,
        src: *mut u8,
        size: usize,
    ) -> Result<bool, Box<dyn Error>> {
        let mut old_protection: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);

        unsafe {
            if VirtualProtect(target, size, PAGE_READWRITE, &mut old_protection).is_err() {
                return Err(format!(
                    "Failed changing protection of hooked function - Error {}",
                    std::io::Error::last_os_error()
                )
                .into());
            }

            ptr::copy_nonoverlapping(src, target as *mut u8, size);

            // Restore the old protection
            if VirtualProtect(target, size, old_protection, &mut old_protection).is_err() {
                return Err(format!(
                    "Failed changing to old protection of hooked function - Error {}",
                    std::io::Error::last_os_error()
                )
                .into());
            }
        }

        Ok(true)
    }
}

impl Drop for MemoryPatch {
    fn drop(&mut self) {
        self.cleanup();
    }
}
