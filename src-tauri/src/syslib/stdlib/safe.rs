use super::raw;
use std::ffi::CString;
use std::os::raw::c_int;

pub struct DesktopStd;

impl DesktopStd {
    pub fn cpu_load() -> f64 {
        raw::std_get_cpu_load() as f64
    }

    pub fn memory_available() -> u64 {
        raw::std_get_mem_available()
    }

    pub fn current_pid() -> i32 {
        raw::std_get_process_id() as i32
    }

    pub fn kill_process(pid: i32) -> Result<(), String> {
        let res = raw::std_kill_process(pid as c_int);
        if res == 0 {
            Ok(())
        } else {
            Err("Failed to kill process".into())
        }
    }

    pub fn notify(title: &str, message: &str) -> Result<(), String> {
        let c_title = CString::new(title).map_err(|_| "Invalid title")?;
        let c_msg = CString::new(message).map_err(|_| "Invalid message")?;

        let res = unsafe { raw::std_send_notification(c_title.as_ptr(), c_msg.as_ptr()) };

        if res == 0 {
            Ok(())
        } else {
            Err("Notification failed".into())
        }
    }

    pub fn get_env(name: &str) -> Option<String> {
        let c_name = CString::new(name).ok()?;
        let ptr = unsafe { raw::std_get_env_var(c_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
        }
    }
}

use std::ffi::CStr;
