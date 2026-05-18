use super::raw;
use std::ffi::CStr;
use std::os::raw::c_int;

/// High-level API for C FFI interaction.
pub struct FfiBridge;

impl FfiBridge {
    // 1. Simple Values
    pub fn calculate_sum(a: i32, b: i32) -> i32 {
        raw::mock_c_calculate_sum(a as c_int, b as c_int) as i32
    }

    // 2. Strings
    pub fn get_version() -> String {
        let ptr = unsafe { raw::mock_c_get_version() };
        if ptr.is_null() {
            return "unknown".to_string();
        }
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }

    // 3. Memory Buffers
    pub fn allocate_and_read(size: usize) -> Vec<u8> {
        let ptr = unsafe { raw::mock_c_allocate_buffer(size) };
        if ptr.is_null() {
            return Vec::new();
        }
        let data = unsafe { std::slice::from_raw_parts(ptr, size).to_vec() };
        unsafe { raw::mock_c_free_buffer(ptr) };
        data
    }

    // 4. Callback Wrapper
    pub fn run_task_with_callback<F>(_input: i32, _callback: F)
    where
        F: Fn(i32) + 'static,
    {
        // Simplified for demo. In production, use a trampoline.
    }
}

/// Opaque Handle Wrapper (RAII)
pub struct Session {
    handle: *mut raw::COpaqueSession,
}

impl Session {
    pub fn new(id: i32) -> Self {
        Self {
            handle: unsafe { raw::mock_c_session_create(id as c_int) },
        }
    }

    pub fn id(&self) -> i32 {
        unsafe { raw::mock_c_session_get_id(self.handle) as i32 }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            raw::mock_c_session_destroy(self.handle);
        }
    }
}
