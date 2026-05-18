use std::os::raw::{c_char, c_double, c_int};

// --- System Metrics (Hardware) ---
#[no_mangle]
pub extern "C" fn std_get_cpu_load() -> c_double {
    // In a real C lib, this would read /proc/stat or use Mach kernel APIs
    0.42 // Mock 42% load
}

#[no_mangle]
pub extern "C" fn std_get_mem_available() -> u64 {
    // Mock available memory in bytes (e.g., 8GB)
    8 * 1024 * 1024 * 1024
}

// --- Process Management ---
#[no_mangle]
pub extern "C" fn std_get_process_id() -> c_int {
    // Mock PID
    1234
}

#[no_mangle]
pub extern "C" fn std_kill_process(pid: c_int) -> c_int {
    if pid <= 0 {
        return -1;
    }
    0 // Success
}

// --- System Communication ---
#[no_mangle]
pub extern "C" fn std_send_notification(title: *const c_char, msg: *const c_char) -> c_int {
    // In a real C lib, this would call libnotify (Linux) or WinToast (Windows)
    0 // Success
}

// --- Environment & Paths ---
#[no_mangle]
pub extern "C" fn std_get_env_var(name: *const c_char) -> *const c_char {
    // Mocking environment variable return
    "mock_env_value\0".as_ptr() as *const c_char
}
