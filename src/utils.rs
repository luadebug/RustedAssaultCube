use std::ffi::{c_void, CString};
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::ptr;
use std::ptr::null_mut;
use std::sync::Mutex;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};
use windows::core::PCSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Console::{AllocConsole, FreeConsole};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{
    VirtualProtect, VirtualQuery, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE, PAGE_EXECUTE_READ,
    PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_PROTECTION_FLAGS, PAGE_READONLY, PAGE_READWRITE,
};
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::{GetCurrentProcess, CREATE_NO_WINDOW};

pub fn find_pattern(module: &str, pattern: &[u8], mask: &str) -> Option<usize> {
    let module_name = CString::new(module).unwrap();
    let hmodule = unsafe {
        let res = GetModuleHandleA(PCSTR(module_name.as_ptr() as *const u8));
        if res.is_err() {
            println!("[utils] Failed to get module handle: {}", module);
            return None;
        }
        res
    };

    // Get module information
    let mut module_info = MODULEINFO {
        lpBaseOfDll: null_mut(),
        SizeOfImage: 0,
        EntryPoint: null_mut(),
    };

    unsafe {
        if GetModuleInformation(
            GetCurrentProcess(),
            hmodule.unwrap(),
            &mut module_info,
            size_of::<MODULEINFO>() as u32,
        )
        .is_err()
        {
            println!("[utils] Failed to get module information");
            return None;
        }
    }

    let module_base = module_info.lpBaseOfDll as usize;
    let module_end = module_base + module_info.SizeOfImage as usize;
    let pattern_length = pattern.len();
    let mask_bytes = mask.as_bytes();

    // Validate that the pattern and mask lengths match
    if pattern_length != mask_bytes.len() {
        println!("[utils] Error: Pattern and mask lengths do not match!");
        return None;
    }

    // Skip table for optimization (modified to handle wildcards)
    let mut skip_table = [pattern_length; 256];
    let mut last_valid_byte_index = pattern_length; // Keep track of the last non-wildcard byte

    for i in (0..pattern_length).rev() {
        if mask_bytes[i] == b'x' {
            last_valid_byte_index = i;
            break; // Optimization: No need to process further if we find 'x'
        }
    }

    for i in (0..last_valid_byte_index).rev() {
        if mask_bytes[i] == b'x' {
            // Only consider non-wildcard bytes for the skip table
            skip_table[pattern[i] as usize] = last_valid_byte_index - i;
        }
    }

    let mut i = last_valid_byte_index; // Start from the last non-wildcard byte

    while i < module_end - module_base {
        let mut j = last_valid_byte_index;
        let mut k = i + module_base;

        // Check for a match (considering the mask)
        while j > 0 && (mask_bytes[j] == b'?' || unsafe { *(k as *const u8) } == pattern[j]) {
            k -= 1;
            j -= 1;
        }

        // Check if the entire pattern matched
        if j == 0 && (mask_bytes[j] == b'?' || unsafe { *(k as *const u8) } == pattern[j]) {
            return Some(k);
        }

        // Move to the next position using the skip table (modified)
        if i < module_end - module_base {
            // Prevent potential out-of-bounds access
            let current_byte = unsafe { *(module_base as *const u8).add(i) };
            i += skip_table[current_byte as usize];
        } else {
            break; // Exit the loop if we've reached the end of the module
        }
    }

    let mut i = 0; // Start from the beginning of the module

    while i < module_end - module_base - pattern_length {
        // Adjusted loop condition
        let mut matched = true;
        for j in 0..pattern_length {
            if mask_bytes[j] == b'x'
                && pattern[j] != unsafe { *(module_base as *const u8).add(i + j) }
            {
                matched = false;
                break;
            }
        }

        if matched {
            return Some(module_base + i);
        }

        i += 1; // Move to the next byte in the module
    }

    println!(
        "[utils] Pattern {:?} with mask {} was not found in {} module",
        pattern, mask, module
    );
    None
}

/*pub unsafe fn read_memory(address: *const c_void, buffer: *mut c_void, size: usize) -> bool {
    let mut old_protect = PAGE_PROTECTION_FLAGS(0);

    // Change the memory protection to PAGE_READWRITE
    if VirtualProtect(address as *mut _, size, PAGE_READWRITE, &mut old_protect).is_err() {
        println!("[read_memory] Failed to change memory protection to RW");
        return false; // VirtualProtect failed
    }

    // Copy memory from the address to the buffer
    ptr::copy_nonoverlapping(address, buffer, size);

    // Restore the old memory protection
    if VirtualProtect(address as *mut _, size, old_protect, &mut old_protect).is_err() {
        println!("[read_memory] Failed to restore original memory protection");
    }

    true // Indicate success
}*/

pub unsafe fn write_memory<T>(address: usize, value: T) -> Result<(), String> {
    unsafe {
        // Create a MEMORY_BASIC_INFORMATION structure to hold the memory info
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();

        // Variable to hold the old protection flags
        let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);

        // Query the memory information at the specified address
        if VirtualQuery(
            Option::from(address as *const c_void),
            &mut mbi,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        ) == 0
        {
            return Err(format!(
                "Failed to query memory information at {:x}",
                address
            ));
        }

        // Check the current protection flags
        let current_protect = mbi.Protect;

        // Determine if the memory is writable
        let is_writable =
            (current_protect & (PAGE_READWRITE | PAGE_EXECUTE_READWRITE)) != PAGE_NOACCESS;

        // If the memory is not writable, change the protection
        if !is_writable {
            let new_protect = if current_protect == PAGE_EXECUTE {
                // If the current protection allows execution, change it to PAGE_EXECUTE_READ
                PAGE_EXECUTE_READ
            } else {
                // If the protection is not readable or writable, fallback to PAGE_READWRITE
                PAGE_READWRITE
            };

            // Change memory protection
            if VirtualProtect(
                mbi.BaseAddress,
                mbi.RegionSize,
                new_protect,
                &mut old_protect,
            )
            .is_err()
            {
                return Err(format!(
                    "Failed to change memory protection at {:x}",
                    address
                ));
            }
        }

        // Write the value to the specified address
        ptr::write_unaligned((address as *mut T).cast(), value);

        // Restore the original memory protection if it was changed
        if !is_writable {
            if VirtualProtect(
                mbi.BaseAddress,
                mbi.RegionSize,
                old_protect,
                &mut old_protect,
            )
            .is_err()
            {
                return Err(format!(
                    "Failed to restore memory protection at {:x}",
                    address
                ));
            }
        }

        Ok(())
    }
}

pub unsafe fn read_memory<T>(address: usize) -> Result<T, String> {
    unsafe {
        // Determine the size of T
        //let size_of_t = std::mem::size_of::<T>();

        // Create a MEMORY_BASIC_INFORMATION structure to hold the memory info
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();

        // Variable to hold the old protection flags
        let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);

        // Query the memory information at the specified address
        if VirtualQuery(
            Option::from(address as *const c_void),
            &mut mbi,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        ) == 0
        {
            return Err(format!(
                "Failed to query memory information at {:x}",
                address
            ));
        }

        // Check the current protection flags
        let current_protect = mbi.Protect;

        // Determine if the memory is readable
        let is_readable = (current_protect
            & (PAGE_READONLY | PAGE_READWRITE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE))
            != PAGE_NOACCESS;

        // If the memory is not readable, change the protection
        if !is_readable {
            let new_protect = if current_protect == PAGE_EXECUTE {
                // If the current protection allows execution, change it to PAGE_EXECUTE_READ
                PAGE_EXECUTE_READ
            } else {
                // If the protection is not readable or writable, fallback to PAGE_READWRITE
                PAGE_READWRITE
            };

            // Change memory protection
            if VirtualProtect(
                mbi.BaseAddress,
                mbi.RegionSize,
                new_protect,
                &mut old_protect,
            )
            .is_err()
            {
                return Err(format!(
                    "Failed to change memory protection at {:x}",
                    address
                ));
            }
        }

        // Read the value from the specified address
        let value = ptr::read_unaligned((address as *const T).cast());

        // Restore the original memory protection if it was changed
        if !is_readable {
            if VirtualProtect(
                mbi.BaseAddress,
                mbi.RegionSize,
                old_protect,
                &mut old_protect,
            )
            .is_err()
            {
                return Err(format!(
                    "Failed to restore memory protection at {:x}",
                    address
                ));
            }
        }

        Ok(value)
    }
}

pub unsafe fn read_vector<T>(address: usize, len: usize) -> Result<Vec<T>, String> {
    unsafe {
        // Create a MEMORY_BASIC_INFORMATION structure to hold the memory info
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();

        // Variable to hold the old protection flags
        let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);

        // Query the memory information at the specified address
        if VirtualQuery(
            Option::from(address as *const c_void),
            &mut mbi,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        ) == 0
        {
            return Err(format!(
                "Failed to query memory information at {:x}",
                address
            ));
        }

        // Check the current protection flags
        let current_protect = mbi.Protect;

        // Determine if the memory is readable
        let is_readable = (current_protect
            & (PAGE_READONLY | PAGE_READWRITE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE))
            != PAGE_NOACCESS;

        // If the memory is not readable, change the protection
        if !is_readable {
            let new_protect = if current_protect == PAGE_EXECUTE {
                // If the current protection allows execution, change it to PAGE_EXECUTE_READ
                PAGE_EXECUTE_READ
            } else {
                // If the protection is not readable or writable, fallback to PAGE_READWRITE
                PAGE_READWRITE
            };

            // Change memory protection
            if VirtualProtect(
                mbi.BaseAddress,
                mbi.RegionSize,
                new_protect,
                &mut old_protect,
            )
            .is_err()
            {
                return Err(format!(
                    "Failed to change memory protection at {:x}",
                    address
                ));
            }
        }

        // Create a vector to store the read values
        let mut values: Vec<T> = Vec::with_capacity(len);

        // Read each element of the vector
        for i in 0..len {
            let element_address = address + i * std::mem::size_of::<T>();
            let value = ptr::read_unaligned((element_address as *const T).cast());
            values.push(value);
        }

        // Restore the original memory protection if it was changed
        if !is_readable {
            if VirtualProtect(
                mbi.BaseAddress,
                mbi.RegionSize,
                old_protect,
                &mut old_protect,
            )
            .is_err()
            {
                return Err(format!(
                    "Failed to restore memory protection at {:x}",
                    address
                ));
            }
        }

        Ok(values)
    }
}

pub unsafe fn read_view_matrix(address: usize) -> Result<[f32; 16], String> {
    // Read the data into a Vec<f32> (same logic as before)
    unsafe {
        let vector: Vec<f32> = read_vector(address, 16)?;

        // Try to convert the Vec<f32> into a [f32; 16]
        let array: [f32; 16] = vector.try_into().unwrap(); // This will panic if vector.len() != 16

        Ok(array)
    }
}

/*pub unsafe fn read_memory<T>(address: usize) -> Result<T, String> {
    // Change memory protection to readable and executable if needed
    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    let size = std::mem::size_of::<T>();
    if VirtualProtect(address as *mut c_void, size, PAGE_READWRITE, &mut old_protect).is_err() {
        return Err(format!("Failed to change memory protection at {:x}", address));
    }

    // Read the value
    let value = ptr::read_unaligned((address as *const T).cast());

    // Restore original memory protection
    if VirtualProtect(address as *mut c_void, size, old_protect, &mut old_protect).is_err() {
        return Err(format!("Failed to restore memory protection at {:x}", address));
    }

    Ok(value)
}*/

pub unsafe fn read_memory_into_slice(address: usize, buffer: &mut [u8]) -> Result<(), String> {
    let size = buffer.len();

    // Change memory protection to readable and executable if needed
    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    unsafe {
        if VirtualProtect(
            address as *mut c_void,
            size,
            PAGE_READWRITE,
            &mut old_protect,
        )
        .is_err()
        {
            return Err(format!(
                "Failed to change memory protection at {:x}",
                address
            ));
        }

        // Copy the memory into the buffer
        ptr::copy_nonoverlapping(address as *const u8, buffer.as_mut_ptr(), size);

        // Restore original memory protection
        if VirtualProtect(address as *mut c_void, size, old_protect, &mut old_protect).is_err() {
            return Err(format!(
                "Failed to restore memory protection at {:x}",
                address
            ));
        }
    }
    Ok(())
}

#[allow(unused)]
pub fn open_console() {
    unsafe {
        AllocConsole().expect("Failed to allocate console");
    }
}
#[allow(unused)]
pub fn close_console() {
    unsafe {
        FreeConsole().expect("Failed to free console");
    }
}
#[allow(unused)]
pub fn run_cmd(command: &str) -> String {
    let mut result = String::new();

    let mut child = Command::new("cmd")
        .args(&["/C", command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW.0)
        .spawn()
        .expect("Failed to execute command");

    let stdout = child.stdout.take().unwrap();
    let mut reader = std::io::BufReader::new(stdout);

    let mut buf = [0; 1024];
    loop {
        let bytes_read = reader.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        result.push_str(std::str::from_utf8(&buf[0..bytes_read]).unwrap());
    }

    result
}

pub unsafe fn setup_tracing() {
    let e = hudhook::alloc_console();
    if e.is_err() {
        unsafe {
            println!(
                "[MainThread] Failed to allocate console: {:?}",
                GetLastError()
            );
        }
    } else {
        println!("[MainThread] Allocated console");
    }
    hudhook::enable_console_colors();
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    } //trace

    let log_file = hudhook::util::get_dll_path()
        .map(|mut path| {
            path.set_extension("log");
            path
        })
        .and_then(|path| File::create(path).ok())
        .unwrap();

    tracing_subscriber::registry()
        .with(
            fmt::layer().event_format(
                fmt::format()
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true),
            ),
        )
        .with(
            fmt::layer()
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_names(true)
                .with_writer(Mutex::new(log_file))
                .with_ansi(false)
                .boxed(),
        )
        .with(EnvFilter::from_default_env())
        .init();
}
