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
pub extern "C" fn std_send_notification(_title: *const c_char, _msg: *const c_char) -> c_int {
    // In a real C lib, this would call libnotify (Linux) or WinToast (Windows)
    0 // Success
}

// --- Environment & Paths ---
#[no_mangle]
pub extern "C" fn std_get_env_var(_name: *const c_char) -> *const c_char {
    // Mocking environment variable return
    "mock_env_value\0".as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_std_get_cpu_load() {
        let load = std_get_cpu_load();
        assert_eq!(load, 0.42);
    }

    #[test]
    fn test_std_get_cpu_load_within_range() {
        let load = std_get_cpu_load();
        assert!(
            load >= 0.0 && load <= 100.0,
            "CPU load should be between 0 and 100"
        );
    }

    #[test]
    fn test_std_get_mem_available() {
        let mem = std_get_mem_available();
        assert_eq!(mem, 8 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_std_get_mem_available_positive() {
        assert!(std_get_mem_available() > 0);
    }

    #[test]
    fn test_std_get_process_id() {
        let pid = std_get_process_id();
        assert_eq!(pid, 1234);
    }

    #[test]
    fn test_std_get_process_id_positive() {
        assert!(std_get_process_id() > 0);
    }

    #[test]
    fn test_std_kill_process_valid_pid() {
        let result = std_kill_process(100);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_std_kill_process_zero_pid() {
        let result = std_kill_process(0);
        assert_eq!(result, -1, "Invalid PID should return -1");
    }

    #[test]
    fn test_std_kill_process_negative_pid() {
        let result = std_kill_process(-5);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_std_send_notification_success() {
        let c_title = std::ffi::CString::new("Test").unwrap();
        let c_msg = std::ffi::CString::new("Message").unwrap();
        let result = std_send_notification(c_title.as_ptr(), c_msg.as_ptr());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_std_get_env_var_non_null() {
        let ptr = std_get_env_var(std::ptr::null());
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        assert_eq!(cstr.to_str().unwrap(), "mock_env_value");
    }

    #[test]
    fn test_std_get_env_var_with_name() {
        let c_name = std::ffi::CString::new("HOME").unwrap();
        let ptr = std_get_env_var(c_name.as_ptr());
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        // Mock always returns the same value regardless of input
        assert_eq!(cstr.to_str().unwrap(), "mock_env_value");
    }
}
