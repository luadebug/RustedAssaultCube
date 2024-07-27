use std::ffi::{c_void, CString};
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::ptr;
use std::ptr::null_mut;
use std::sync::Mutex;

use tracing_subscriber::{EnvFilter, fmt, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use windows::core::PCSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Console::{AllocConsole, FreeConsole};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{PAGE_PROTECTION_FLAGS, PAGE_READWRITE, VirtualProtect};
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::{CREATE_NO_WINDOW, GetCurrentProcess};

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
        ).is_err()
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
        if mask_bytes[i] == b'x' { // Only consider non-wildcard bytes for the skip table
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
        if i < module_end - module_base { // Prevent potential out-of-bounds access
            let current_byte = unsafe { *(module_base as *const u8).add(i) };
            i += skip_table[current_byte as usize];
        } else {
            break; // Exit the loop if we've reached the end of the module
        }
    }



    let mut i = 0; // Start from the beginning of the module

    while i < module_end - module_base - pattern_length { // Adjusted loop condition
        let mut matched = true;
        for j in 0..pattern_length {
            if mask_bytes[j] == b'x' && pattern[j] != unsafe { *(module_base as *const u8).add(i + j) } {
                matched = false;
                break;
            }
        }

        if matched {
            return Some(module_base + i);
        }

        i += 1; // Move to the next byte in the module
    }





    println!("[utils] Pattern {:?} with mask {} was not found in {} module", pattern, mask, module);
    None
}


pub unsafe fn read_memory(address: *const c_void, buffer: *mut c_void, size: usize) -> bool {
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
}

#[allow(unused)]
pub fn open_console() {
    unsafe
    {
        AllocConsole().expect("Failed to allocate console");
    }
}
#[allow(unused)]
pub fn close_console() {
    unsafe
    {
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
    if e.is_err()
    {
        println!("[MainThread] Failed to allocate console: {:?}", GetLastError());
    }
    else
    {
        println!("[MainThread] Allocated console");
    }
    hudhook::enable_console_colors();
    std::env::set_var("RUST_LOG", "info"); //trace

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