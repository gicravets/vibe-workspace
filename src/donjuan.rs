//! Local owner CLI adapter. Never registered as an MCP write tool.
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Operation {
    Prepare,
    Apply,
    Readback,
}

pub fn project_operation(
    python: &Path,
    sdk_root: &Path,
    workspace_root: &Path,
    operation: Operation,
) -> io::Result<ExitStatus> {
    if !python.is_absolute() || !sdk_root.is_absolute() || !workspace_root.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Python, SDK root and workspace root must be absolute paths",
        ));
    }
    let verb = match operation {
        Operation::Prepare => "prepare",
        Operation::Apply => "apply",
        Operation::Readback => "readback",
    };
    Command::new(python)
        .args(["-E", "-B"])
        .arg(sdk_root.join("scripts/donjuan_project_adapter.py"))
        .arg("--root")
        .arg(workspace_root.canonicalize()?)
        .arg(verb)
        .status()
}
