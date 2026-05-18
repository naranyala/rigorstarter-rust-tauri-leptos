use std::os::raw::{c_char, c_int};

// --- Simple Value types ---
#[no_mangle]
pub extern "C" fn mock_c_calculate_sum(a: c_int, b: c_int) -> c_int {
    a + b
}

// --- String types ---
#[no_mangle]
pub extern "C" fn mock_c_get_version() -> *const c_char {
    "v1.0.0-stable\0".as_ptr() as *const c_char
}

// --- Buffer/Memory types ---
#[no_mangle]
pub extern "C" fn mock_c_allocate_buffer(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

#[no_mangle]
pub extern "C" fn mock_c_free_buffer(_ptr: *mut u8) {
    // In a real C lib, this would be free(ptr)
    let _layout = std::alloc::Layout::from_size_align(0, 1).unwrap();
}

// --- Opaque Pointer / Handle Pattern ---
#[repr(C)]
pub struct COpaqueSession {
    pub id: c_int,
    pub secret: [u8; 16],
}

#[no_mangle]
pub extern "C" fn mock_c_session_create(id: c_int) -> *mut COpaqueSession {
    Box::into_raw(Box::new(COpaqueSession {
        id,
        secret: [0u8; 16],
    }))
}

#[no_mangle]
pub extern "C" fn mock_c_session_get_id(session: *mut COpaqueSession) -> c_int {
    if session.is_null() {
        return -1;
    }
    unsafe { (*session).id }
}

#[no_mangle]
pub extern "C" fn mock_c_session_destroy(session: *mut COpaqueSession) {
    if !session.is_null() {
        unsafe {
            let _ = Box::from_raw(session);
        }
    }
}

// --- Callback Pattern ---
pub type CCallback = Option<unsafe extern "C" fn(c_int)>;

#[no_mangle]
pub extern "C" fn mock_c_do_work_with_callback(_val: c_int, cb: CCallback) {
    if let Some(callback) = cb {
        unsafe {
            callback(_val * 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::slice;

    #[test]
    fn test_mock_c_calculate_sum_positive() {
        let result = mock_c_calculate_sum(10, 20);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_mock_c_calculate_sum_negative() {
        let result = mock_c_calculate_sum(-5, 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mock_c_calculate_sum_zero() {
        let result = mock_c_calculate_sum(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mock_c_get_version_non_null() {
        let ptr = mock_c_get_version();
        assert!(!ptr.is_null(), "Version pointer should not be null");
    }

    #[test]
    fn test_mock_c_get_version_content() {
        let ptr = mock_c_get_version();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let version = cstr.to_str().unwrap();
        assert_eq!(version, "v1.0.0-stable");
    }

    #[test]
    fn test_mock_c_allocate_buffer_zero_size() {
        let ptr = mock_c_allocate_buffer(0);
        // Zero-size allocation may or may not return null
        // The important thing is it doesn't crash
        let _ = ptr;
    }

    #[test]
    fn test_mock_c_allocate_buffer_normal_size() {
        let size = 1024;
        let ptr = mock_c_allocate_buffer(size);
        assert!(!ptr.is_null(), "Allocated pointer should not be null");
        let slice = unsafe { slice::from_raw_parts(ptr, size) };
        assert_eq!(slice.len(), size);
        mock_c_free_buffer(ptr);
    }

    #[test]
    fn test_mock_c_allocate_buffer_write_and_read() {
        let size = 64;
        let ptr = mock_c_allocate_buffer(size);
        assert!(!ptr.is_null());
        let slice = unsafe { slice::from_raw_parts_mut(ptr, size) };
        slice[0] = 42;
        slice[size - 1] = 99;
        assert_eq!(slice[0], 42);
        assert_eq!(slice[size - 1], 99);
        mock_c_free_buffer(ptr);
    }

    #[test]
    fn test_mock_c_session_create_and_get_id() {
        let session = mock_c_session_create(42);
        assert!(!session.is_null(), "Session handle should not be null");
        let id = mock_c_session_get_id(session);
        assert_eq!(id, 42);
        mock_c_session_destroy(session);
    }

    #[test]
    fn test_mock_c_session_get_id_null() {
        let id = mock_c_session_get_id(std::ptr::null_mut());
        assert_eq!(id, -1, "Null session should return -1");
    }

    #[test]
    fn test_mock_c_session_multiple_sessions() {
        let sessions: Vec<_> = (0..10).map(|i| mock_c_session_create(i)).collect();
        for (i, session) in sessions.iter().enumerate() {
            assert_eq!(mock_c_session_get_id(*session), i as c_int);
        }
        for session in sessions {
            mock_c_session_destroy(session);
        }
    }

    #[test]
    fn test_mock_c_session_large_id() {
        let session = mock_c_session_create(9999);
        assert_eq!(mock_c_session_get_id(session), 9999);
        mock_c_session_destroy(session);
    }

    #[test]
    fn test_mock_c_session_destroy_null_safety() {
        // Should not crash when destroying null
        mock_c_session_destroy(std::ptr::null_mut());
    }

    unsafe extern "C" fn test_callback(val: c_int) {
        assert_eq!(val, 10);
    }

    #[test]
    fn test_mock_c_do_work_with_callback() {
        let cb: CCallback = Some(test_callback);
        mock_c_do_work_with_callback(5, cb);
    }

    #[test]
    fn test_mock_c_do_work_with_none_callback() {
        // Should not panic when no callback is provided
        mock_c_do_work_with_callback(5, None);
    }
}
