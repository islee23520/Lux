use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const INSTALLER_VERSION: &str = env!("CARGO_PKG_VERSION");
const RELEASE_BASE_URL: &str = "https://github.com/islee23520/lux/releases/latest/download";

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("lux installer error: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args.iter().any(|arg| arg == "--installer-version") {
        println!("linalab-lux {INSTALLER_VERSION}");
        return Ok(ExitCode::SUCCESS);
    }

    let binary = resolve_binary()?;
    execute_binary(&binary, &args)
}

fn resolve_binary() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("LUX_INSTALLER_BINARY") {
        return Ok(PathBuf::from(path));
    }

    let target = target_triple()?;
    let binary = cache_dir()?.join(&target).join(binary_name());
    if binary.is_file() {
        return Ok(binary);
    }

    download_binary(&target, &binary)?;
    Ok(binary)
}

fn target_triple() -> Result<String, String> {
    let arch = match env::consts::ARCH {
        "aarch64" => "aarch64",
        "x86_64" => "x86_64",
        other => return Err(format!("unsupported CPU architecture: {other}")),
    };

    let os = match env::consts::OS {
        "macos" => "apple-darwin",
        "linux" => "unknown-linux-gnu",
        "windows" => "pc-windows-msvc",
        other => return Err(format!("unsupported operating system: {other}")),
    };

    Ok(format!("{arch}-{os}"))
}

fn binary_name() -> &'static str {
    if cfg!(windows) {
        "lux.exe"
    } else {
        "lux"
    }
}

fn cache_dir() -> Result<PathBuf, String> {
    if let Some(home) = env::var_os("HOME") {
        return Ok(PathBuf::from(home).join(".lux").join("bin"));
    }

    if let Some(profile) = env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(profile).join(".lux").join("bin"));
    }

    Err("HOME or USERPROFILE is required to locate the LUX binary cache".to_string())
}

fn download_binary(target: &str, destination: &Path) -> Result<(), String> {
    let parent = destination
        .parent()
        .ok_or_else(|| "binary cache path has no parent directory".to_string())?;
    fs::create_dir_all(parent).map_err(|err| format!("create cache directory: {err}"))?;

    let url = format!("{RELEASE_BASE_URL}/lux-{target}");
    let temp_path = destination.with_extension("download");
    let status = Command::new("curl")
        .arg("--fail")
        .arg("--location")
        .arg("--silent")
        .arg("--show-error")
        .arg("--output")
        .arg(&temp_path)
        .arg(&url)
        .status()
        .map_err(|err| format!("start curl for {url}: {err}"))?;

    if !status.success() {
        return Err(format!("download failed from {url}"));
    }

    make_executable(&temp_path)?;
    fs::rename(&temp_path, destination).map_err(|err| format!("install downloaded binary: {err}"))
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .map_err(|err| format!("read downloaded binary metadata: {err}"))?
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
        .map_err(|err| format!("mark downloaded binary executable: {err}"))
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<(), String> {
    Ok(())
}

fn execute_binary(binary: &Path, args: &[OsString]) -> Result<ExitCode, String> {
    let status = Command::new(binary)
        .args(args)
        .status()
        .map_err(|err| command_error(binary, err))?;

    Ok(status.code().map_or_else(|| ExitCode::from(1), exit_code))
}

fn command_error(binary: &Path, err: io::Error) -> String {
    format!("run {}: {err}", binary.display())
}

fn exit_code(code: i32) -> ExitCode {
    match u8::try_from(code) {
        Ok(value) => ExitCode::from(value),
        Err(_) => ExitCode::from(1),
    }
}
