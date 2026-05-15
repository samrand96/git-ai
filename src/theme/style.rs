use std::io::IsTerminal;
use std::sync::{OnceLock, RwLock};

use super::detect;
use super::palette::{BUILT_IN_THEMES, BuiltInTheme, Rgb, ThemePalette};

pub struct Colors;

static ACTIVE_THEME: OnceLock<RwLock<ThemePalette>> = OnceLock::new();

impl Colors {
    const BOLD: &'static str = "\u{001b}[1m";
    const DIM: &'static str = "\u{001b}[2m";
    const RESET: &'static str = "\u{001b}[0m";

    pub fn supports_color() -> bool {
        std::io::stdout().is_terminal()
    }

    pub fn built_in_themes() -> &'static [BuiltInTheme] {
        &BUILT_IN_THEMES
    }

    pub fn default_theme() -> ThemePalette {
        detect::system_theme()
    }

    pub fn built_in_theme(name: &str) -> Option<ThemePalette> {
        if name.eq_ignore_ascii_case("auto") {
            return Some(detect::system_theme());
        }

        BUILT_IN_THEMES
            .iter()
            .find(|theme| theme.name.eq_ignore_ascii_case(name))
            .map(|theme| theme.palette)
    }

    pub fn system_scheme() -> &'static str {
        detect::system_scheme()
    }

    pub fn set_theme(theme: ThemePalette) {
        let active = ACTIVE_THEME.get_or_init(|| RwLock::new(Self::default_theme()));
        if let Ok(mut guard) = active.write() {
            *guard = theme;
        }
    }

    pub fn parse_rgb(input: &str) -> Result<Rgb, String> {
        Rgb::parse(input)
    }

    pub fn format_rgb(rgb: Rgb) -> String {
        format!("{},{},{}", rgb.r, rgb.g, rgb.b)
    }

    pub fn success(text: impl AsRef<str>) -> String {
        Self::colorize(text.as_ref(), Self::active_theme().success.fg())
    }

    pub fn error(text: impl AsRef<str>) -> String {
        Self::colorize(
            text.as_ref(),
            format!("{}{}", Self::active_theme().error.fg(), Self::BOLD),
        )
    }

    pub fn warning(text: impl AsRef<str>) -> String {
        Self::colorize(text.as_ref(), Self::active_theme().warning.fg())
    }

    pub fn info(text: impl AsRef<str>) -> String {
        Self::colorize(text.as_ref(), Self::active_theme().info.fg())
    }

    pub fn header(text: impl AsRef<str>) -> String {
        Self::colorize(
            text.as_ref(),
            format!("{}{}", Self::active_theme().header.fg(), Self::BOLD),
        )
    }

    pub fn dim(text: impl AsRef<str>) -> String {
        Self::colorize(
            text.as_ref(),
            format!("{}{}", Self::active_theme().dim.fg(), Self::DIM),
        )
    }

    pub fn highlight(text: impl AsRef<str>) -> String {
        Self::colorize(
            text.as_ref(),
            format!("{}{}", Self::active_theme().highlight.fg(), Self::BOLD),
        )
    }

    pub fn format_cli_output(text: &str) -> String {
        if !Self::supports_color() {
            return text.to_string();
        }

        text.lines()
            .map(Self::format_cli_line)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn active_theme() -> ThemePalette {
        ACTIVE_THEME
            .get_or_init(|| RwLock::new(Self::default_theme()))
            .read()
            .map(|guard| *guard)
            .unwrap_or_else(|_| Self::default_theme())
    }

    fn colorize(text: &str, color: String) -> String {
        if Self::supports_color() {
            format!("{color}{text}{}", Self::RESET)
        } else {
            text.to_string()
        }
    }

    fn format_cli_line(line: &str) -> String {
        let lower = line.to_lowercase();
        let trimmed = line.trim_start();
        let theme = Self::active_theme();
        let is_bullet = trimmed.starts_with('-')
            || trimmed.starts_with('*')
            || trimmed.starts_with('•')
            || trimmed.starts_with('→');

        if contains_any(
            &lower,
            &[
                "error",
                "bug",
                "vulnerability",
                "critical",
                "severe",
                "dangerous",
                "fail",
            ],
        ) {
            Self::colorize(line, format!("{}{}", theme.error.fg(), Self::BOLD))
        } else if contains_any(
            &lower,
            &[
                "warning",
                "caution",
                "potential",
                "consider",
                "should",
                "might",
            ],
        ) {
            Self::colorize(line, theme.warning.fg())
        } else if contains_any(
            &lower,
            &[
                "good",
                "excellent",
                "well",
                "nice",
                "correct",
                "proper",
                "success",
            ],
        ) {
            Self::colorize(line, theme.success.fg())
        } else if line.starts_with('#') {
            Self::colorize(line, format!("{}{}", theme.header.fg(), Self::BOLD))
        } else if is_bullet {
            Self::colorize(line, theme.highlight.fg())
        } else {
            line.to_string()
        }
    }
}

fn contains_any(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

#[cfg(test)]
mod tests {
    use super::Colors;

    #[test]
    fn auto_theme_resolves_to_a_palette() {
        assert!(Colors::built_in_theme("auto").is_some());
    }
}
