// executor.rs
//! Centralized system command execution with a dry-run/testing helper.
//!
//! All OS-level commands (iptables/tc/other shell utilities) must be routed
//! through this module so we have a single place for sanitization, logging
//! and dry-run/mocking in tests.

use std::io;
use std::process::Command;

/// Result of an executed command (small, stable shape used across the repo).
#[derive(Debug, Clone)]
pub struct ExecOutput {
    /// The exit code if available (platform dependent).
    pub code: Option<i32>,
    /// Raw stdout bytes
    pub stdout: Vec<u8>,
    /// Raw stderr bytes
    pub stderr: Vec<u8>,
    /// The command string that was executed (for logging)
    pub cmd: String,
}

/// Trait that abstracts running system commands. Implementations include a
/// real executor and a dry-run executor useful for tests and local dev.
pub trait SystemExecutor: Send + Sync {
    /// Run `program` with `args`. Non-zero exit status is returned as output.
    /// I/O errors indicate that starting or collecting the process failed.
    fn run(&self, program: &str, args: &[&str]) -> io::Result<ExecOutput>;
}

/// Real executor which actually invokes `std::process::Command`.
pub struct RealExecutor;

impl SystemExecutor for RealExecutor {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<ExecOutput> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        let full = format!("{} {}", program, args.join(" "));
        let output = cmd.output()?;
        Ok(ExecOutput {
            code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
            cmd: full,
        })
    }
}

/// Dry-run executor that doesn't run anything but returns a successful
/// ExecOutput and logs the command. Use this in development and tests to
/// avoid modifying system state (iptables/tc).
#[derive(Default)]
pub struct DryRunExecutor {
    /// Optional message included in dry-run diagnostics.
    pub note: Option<String>,
}

impl SystemExecutor for DryRunExecutor {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<ExecOutput> {
        let full = format!("DRY-RUN: {} {}", program, args.join(" "));
        // For dry-run, we don't execute anything. Return success with empty output.
        // Higher-level code should respect `dry-run` and not treat this as a real
        // execution (i.e. don't assume side-effects happened).
        if let Some(note) = &self.note {
            eprintln!("[dry-run note] {note} -> {full}");
        } else {
            eprintln!("[dry-run] {full}");
        }

        Ok(ExecOutput {
            code: Some(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
            cmd: full,
        })
    }
}

/// Create a boxed executor. Use `dry_run = true` in tests or when running locally
/// to avoid real system modifications.
pub fn new_executor(dry_run: bool) -> Box<dyn SystemExecutor> {
    if dry_run {
        Box::new(DryRunExecutor::default())
    } else {
        Box::new(RealExecutor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_run_does_not_start_program() {
        let output = new_executor(true)
            .run("/nonexistent/bytetally-command", &["argument"])
            .unwrap();
        assert_eq!(output.code, Some(0));
        assert!(output.stdout.is_empty());
    }

    #[test]
    fn missing_program_is_an_io_error() {
        assert!(
            RealExecutor
                .run("/nonexistent/bytetally-command", &[])
                .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn nonzero_exit_is_preserved() {
        let output = RealExecutor
            .run("/bin/sh", &["-c", "printf problem >&2; exit 7"])
            .unwrap();
        assert_eq!(output.code, Some(7));
        assert_eq!(output.stderr, b"problem");
    }
}
