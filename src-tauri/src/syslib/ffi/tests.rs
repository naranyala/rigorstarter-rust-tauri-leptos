#[cfg(test)]
mod tests {
    use crate::syslib::ffi::raw;
    use crate::syslib::ffi::safe::{FfiBridge, Session};

    #[test]
    fn test_simple_sum() {
        assert_eq!(FfiBridge::calculate_sum(10, 20), 30);
        assert_eq!(FfiBridge::calculate_sum(-5, 5), 0);
    }

    #[test]
    fn test_string_version() {
        let version = FfiBridge::get_version();
        assert!(version.contains("v1.0.0"));
        assert!(!version.is_empty());
    }

    #[test]
    fn test_buffer_allocation() {
        let size = 1024;
        let data = FfiBridge::allocate_and_read(size);
        assert_eq!(data.len(), size);
    }

    #[test]
    fn test_buffer_allocation_edge_cases() {
        let data_min = FfiBridge::allocate_and_read(1);
        assert_eq!(data_min.len(), 1);

        let data_zero = FfiBridge::allocate_and_read(0);
        assert_eq!(data_zero.len(), 0);
    }

    #[test]
    fn test_session_handle_raii() {
        {
            let session = Session::new(42);
            assert_eq!(session.id(), 42);
            // Session is dropped here, triggering mock_c_session_destroy
        }
    }

    #[test]
    fn test_session_null_handling() {
        // Test the raw logic for null handles
        unsafe {
            let id = raw::mock_c_session_get_id(std::ptr::null_mut());
            assert_eq!(id, -1);
        }
    }

    #[test]
    fn test_session_lifecycle_stress() {
        for i in 0..100 {
            let session = Session::new(i);
            assert_eq!(session.id(), i);
        }
    }
}
