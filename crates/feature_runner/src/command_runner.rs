use std::path::Path;
use std::process::Command;

use feature_core::{FeatureLabError, FeatureLabResult};

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub command: String,
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_command(program: &str, args: &[&str], cwd: &Path) -> FeatureLabResult<CommandOutput> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|error| FeatureLabError::io(None, error))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let status_code = output.status.code().unwrap_or(-1);
    let command = format!("{program} {}", args.join(" "));
    if output.status.success() {
        return Ok(CommandOutput {
            command,
            status_code,
            stdout,
            stderr,
        });
    }
    Err(FeatureLabError::CommandFailed {
        command,
        status_code,
        stdout,
        stderr,
    })
}

pub fn open_path_in_file_browser(path: &Path) -> FeatureLabResult<CommandOutput> {
    #[cfg(target_os = "macos")]
    {
        let arg = path.to_string_lossy().to_string();
        return run_command("open", &[arg.as_str()], Path::new("."));
    }
    #[cfg(target_os = "linux")]
    {
        let arg = path.to_string_lossy().to_string();
        return run_command("xdg-open", &[arg.as_str()], Path::new("."));
    }
    #[cfg(target_os = "windows")]
    {
        let arg = path.to_string_lossy().to_string();
        return run_command("cmd", &["/C", "start", arg.as_str()], Path::new("."));
    }
    #[allow(unreachable_code)]
    Err(FeatureLabError::Validation(
        "opening a feature folder is unsupported on this platform".into(),
    ))
}
