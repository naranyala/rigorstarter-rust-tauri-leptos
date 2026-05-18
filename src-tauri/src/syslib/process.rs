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
        let result = shell("false"); // 'false' command returns exit code 1
        assert!(result.is_ok()); // The execution happened, but status is non-zero
        assert_ne!(result.unwrap().status, 0);
    }
}
