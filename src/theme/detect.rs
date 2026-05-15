use std::process::Command;

use super::palette::{DARK_THEME, LIGHT_THEME, ThemePalette};

pub fn system_scheme() -> &'static str {
    if system_prefers_light() {
        "light"
    } else {
        "dark"
    }
}

pub(crate) fn system_theme() -> ThemePalette {
    if system_prefers_light() {
        LIGHT_THEME
    } else {
        DARK_THEME
    }
}

fn system_prefers_light() -> bool {
    if cfg!(windows)
        && let Ok(output) = Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output()
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        return stdout.contains("0x1");
    }

    if cfg!(target_os = "macos")
        && let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        return !stdout.contains("dark");
    }

    if let Ok(value) = std::env::var("COLORFGBG")
        && let Some(bg) = value.split(';').next_back()
        && let Ok(code) = bg.parse::<u8>()
    {
        return (7..=15).contains(&code);
    }

    false
}
