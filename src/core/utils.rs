use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct PathHelper;

impl PathHelper {
    /// Finds the absolute path of an executable in the system's PATH.
    pub fn find_executable(cmd: &str) -> Option<String> {
        let path_var = env::var("PATH").ok()?;

        for dir in path_var.split(':') {
            let full_path = Path::new(dir).join(cmd);

            if let Ok(metadata) = fs::metadata(&full_path) {
                if metadata.is_file() {
                    let perms = metadata.permissions();
                    if perms.mode() & 0o111 != 0 {
                        return Some(full_path.to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }

    /// Returns a list of all executable names available in the system's PATH.
    pub fn get_all_executables() -> Option<Vec<String>> {
        let mut executables = Vec::new();
        if let Ok(path_var) = env::var("PATH") {
            for dir in path_var.split(':') {
                let dir_path = Path::new(dir);

                if let Ok(entries) = fs::read_dir(dir_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();

                        if path.is_file() {
                            #[cfg(unix)]
                            {
                                if let Ok(metadata) = path.metadata() {
                                    let perm = metadata.permissions();
                                    if perm.mode() & 0o111 != 0 {
                                        if let Some(name) = path.file_name() {
                                            executables.push(name.to_string_lossy().to_string());
                                        }
                                    }
                                }
                            }

                            #[cfg(windows)]
                            {
                                if let Some(ext) = path.extension() {
                                    if ext == "exe" {
                                        if let Some(name) = path.file_name() {
                                            executables.push(name.to_string_lossy().to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Some(executables)
    }

    /// Returns the current working directory as a string.
    pub fn get_current_dir() -> Option<String> {
        env::current_dir().ok().map(|dir| dir.display().to_string())
    }

    /// Checks if a path exists.
    pub fn path_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Changes the current working directory.
    pub fn change_dir(path: &str) -> std::io::Result<()> {
        env::set_current_dir(path)
    }
}
