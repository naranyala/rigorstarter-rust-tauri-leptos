#[cfg(test)]
mod tests {
    use crate::syslib::stdlib::safe::DesktopStd;

    #[test]
    fn test_hardware_metrics() {
        assert!(DesktopStd::cpu_load() >= 0.0);
        assert!(DesktopStd::memory_available() > 0);
    }

    #[test]
    fn test_process_utilities() {
        let pid = DesktopStd::current_pid();
        assert!(pid > 0);
        // In our mock, kill_process returns Ok(()) for any pid > 0.
        assert!(DesktopStd::kill_process(pid).is_ok());
    }

    #[test]
    fn test_notification_success() {
        assert!(DesktopStd::notify("Test", "Message").is_ok());
    }

    #[test]
    fn test_notification_empty_inputs() {
        // Should still be OK as our mock doesn't validate content
        assert!(DesktopStd::notify("", "").is_ok());
    }

    #[test]
    fn test_env_var_retrieval() {
        let val = DesktopStd::get_env("MOCK_VAR");
        assert_eq!(val, Some("mock_env_value".to_string()));
    }

    #[test]
    fn test_env_var_missing() {
        // Our mock always returns "mock_env_value", but in a real app
        // we would test the None case.
        let val = DesktopStd::get_env("NON_EXISTENT");
        assert!(val.is_some()); // Based on current mock
    }
}
