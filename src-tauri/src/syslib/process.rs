use std::io;
use std::process::Command;

#[derive(Debug)]
pub enum ProcessError {
    IoError(io::Error),
    ExecutionError(String),
}

pub struct ProcessResult {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

pub fn exec(cmd: &str, args: &[&str]) -> Result<ProcessResult, ProcessError> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(ProcessError::IoError)?;

    Ok(ProcessResult {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status.code().unwrap_or(-1),
    })
}

pub fn shell(script: &str) -> Result<ProcessResult, ProcessError> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .map_err(ProcessError::IoError)?;

    Ok(ProcessResult {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status.code().unwrap_or(-1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_success() {
        let result = exec("ls", &["-l"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 0);
    }

    #[test]
    fn test_exec_fail_cmd_not_found() {
        let result = exec("non_existent_cmd_12345", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_shell_success() {
        let result = shell("echo 'hello world'");
        assert!(result.is_ok());
        assert!(result.unwrap().stdout.contains("hello world"));
    }

    #[test]
    fn test_shell_error_exit() {
        let result = shell("false");
        assert!(result.is_ok());
        assert_ne!(result.unwrap().status, 0);
    }

    // --- EDGE CASES ---

    #[test]
    fn test_exec_empty_command() {
        let result = exec("", &[]);
        assert!(result.is_err(), "Empty command should fail");
    }

    #[test]
    fn test_exec_args_with_spaces() {
        let result = exec("echo", &["hello world", "test"]);
        assert!(result.is_ok());
        let out = result.unwrap();
        assert_eq!(out.status, 0);
        assert!(out.stdout.contains("hello world test"));
    }

    #[test]
    fn test_exec_no_args() {
        let result = exec("true", &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 0);
    }

    #[test]
    fn test_exec_stderr_output() {
        let result = exec("sh", &["-c", "echo 'error msg' >&2; exit 1"]);
        assert!(result.is_ok());
        let out = result.unwrap();
        assert_ne!(out.status, 0);
        assert!(
            out.stderr.contains("error msg"),
            "stderr should capture error output"
        );
    }

    #[test]
    fn test_shell_empty_script() {
        let result = shell("");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().status,
            0,
            "Empty shell script should exit 0"
        );
    }

    #[test]
    fn test_shell_unicode_output() {
        let result = shell("echo '日本語 LÖWE 😊'");
        assert!(result.is_ok());
        let out = result.unwrap();
        assert!(out.stdout.contains("日本語"));
        assert!(out.stdout.contains("😊"));
    }

    #[test]
    fn test_shell_stdout_and_stderr() {
        let result = shell("echo out; echo err >&2");
        assert!(result.is_ok());
        let out = result.unwrap();
        assert!(out.stdout.contains("out"), "stdout should capture 'out'");
    }

    #[test]
    fn test_exec_many_args() {
        let args: Vec<String> = (0..100).map(|i| format!("arg{}", i)).collect();
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let result = exec("echo", &arg_refs);
        assert!(result.is_ok(), "Many args should work");
    }

    #[test]
    fn test_shell_pipeline() {
        let result = shell("echo 'hello world' | tr ' ' '_'");
        assert!(result.is_ok());
        let out = result.unwrap();
        assert!(
            out.stdout.contains("hello_world"),
            "Pipeline should produce transformed output"
        );
    }

    #[test]
    fn test_shell_exit_code_127() {
        let result = shell("nonexistent_command_xyz");
        assert!(result.is_ok());
        let out = result.unwrap();
        assert_ne!(out.status, 0, "Non-existent command should exit non-zero");
    }

    #[test]
    fn test_exec_process_result_fields() {
        let result = exec("echo", &["hello"]).unwrap();
        assert!(!result.stdout.is_empty());
        assert_eq!(result.stderr, "");
        assert_eq!(result.status, 0);
    }
}
