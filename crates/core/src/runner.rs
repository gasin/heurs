use crate::types::CommandResult;
use std::process::{Command, Stdio};

/// コマンド実行器のトレイト
pub trait Runner {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
    ) -> Result<CommandResult, Box<dyn std::error::Error>>;
}

/// ローカル実行器の実装
pub struct LocalRunner;

impl LocalRunner {
    pub fn new() -> Self {
        LocalRunner
    }
}

impl Runner for LocalRunner {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
    ) -> Result<CommandResult, Box<dyn std::error::Error>> {
        let output = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Ok(CommandResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }
}

impl Default for LocalRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_runner() {
        let runner = LocalRunner::new();
        let result = runner.execute("echo", &["hello"]).unwrap();
        assert!(result.success);
        assert_eq!(result.stdout.trim(), "hello");
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn test_local_runner_default() {
        let runner = LocalRunner::default();
        let result = runner.execute("echo", &["world"]).unwrap();
        assert!(result.success);
        assert_eq!(result.stdout.trim(), "world");
        assert_eq!(result.exit_code, Some(0));
    }
}
