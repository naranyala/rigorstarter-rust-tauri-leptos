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
