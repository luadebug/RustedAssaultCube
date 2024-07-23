use std::ffi::{c_void, CString};
use std::io::Read;
use std::mem::size_of;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::ptr;
use std::ptr::null_mut;

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
        res.unwrap()
    };

    if hmodule.0.is_null() {
        println!("[utils] Failed to get module handle is null");
        return None;
    }

    let mut module_info = MODULEINFO {
        lpBaseOfDll: null_mut(),
        SizeOfImage: 0,
        EntryPoint: null_mut(),
    };

    unsafe {
        GetModuleInformation(GetCurrentProcess(), hmodule, &mut module_info, size_of::<MODULEINFO>() as u32)
            .expect("[utils] Failed to get module information");
    }

    let module_base = module_info.lpBaseOfDll as usize;
    let module_end = module_base + module_info.SizeOfImage as usize;
    let pattern_length = pattern.len();
    let mask_bytes = mask.as_bytes();

    let mut current_address = module_base;
    while current_address <= module_end - pattern_length {
        let mut found = true;

        for j in 0..pattern_length {

            let current_byte = unsafe { *(current_address as *const u8).add(j) };
            if mask_bytes[j] != b'?' && pattern[j] != current_byte {
                found = false;
                break;
            }
        }

        if found {
            return Some(current_address);
        }
        current_address += 1;
    }
    println!("[utils] Pattern {:?} with mask {} was not found in {} module", pattern, mask, module);
    None
}




pub unsafe fn read_memory(address: *const c_void, buffer: *mut c_void, size: usize) -> bool {
    let mut old_protect = PAGE_PROTECTION_FLAGS(0);

    // Change the memory protection to PAGE_READWRITE
    if VirtualProtect(address as *mut _, size, PAGE_READWRITE, &mut old_protect).is_err() {
        return false; // VirtualProtect failed
    }

    // Copy memory from the address to the buffer
    ptr::copy_nonoverlapping(address, buffer, size);

    // Restore the old memory protection
    VirtualProtect(address as *mut _, size, old_protect, &mut old_protect).ok().unwrap();

    true // Indicate success
}

pub fn open_console() {
    unsafe
    {
        AllocConsole().expect("Failed to allocate console");
    }
}

pub fn close_console() {
    unsafe
    {
        FreeConsole().expect("Failed to free console");
    }
}

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
/*    std::env::set_var("RUST_LOG", "info"); //trace

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
        .init();*/
}