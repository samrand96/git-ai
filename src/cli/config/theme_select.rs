use dialoguer::{Input, Select};

use crate::config::{AppConfig, ThemeFile};
use crate::utils::colors::{Colors, ThemePalette};
use crate::utils::errors::AppResult;

use super::display::theme_name_with_scheme;
use super::prompts::{prompt_rgb, prompt_theme, select_bool};

pub fn choose_theme(config: &mut AppConfig) -> AppResult<()> {
    let entries = theme_entries(config);
    let current = config.theme();
    let default = entries
        .iter()
        .position(|(_, name)| name.as_deref() == Some(current.as_str()))
        .unwrap_or(0);
    let labels = entries
        .iter()
        .map(|(label, _)| label.as_str())
        .collect::<Vec<_>>();

    let idx = Select::with_theme(&prompt_theme())
        .with_prompt("color theme")
        .items(&labels)
        .default(default)
        .interact()?;

    if let Some(theme_name) = entries[idx].1.as_deref() {
        config.set_theme(theme_name);
        let palette = config
            .theme_palette_by_name(theme_name)
            .unwrap_or_else(Colors::default_theme);
        show_theme_preview(theme_name, palette);
    } else {
        create_custom_theme(config)?;
    }

    Ok(())
}

pub fn theme_names(config: &AppConfig) -> String {
    let mut names = Colors::built_in_themes()
        .iter()
        .map(|theme| theme.name.to_string())
        .collect::<Vec<_>>();
    names.extend(config.custom_themes().keys().cloned());
    names.sort();
    names.join(", ")
}

fn create_custom_theme(config: &mut AppConfig) -> AppResult<()> {
    let base = choose_theme_seed(config)?;
    let default_name = format!("custom-{}", config.custom_themes().len() + 1);
    let name = prompt_custom_theme_name(config, &default_name)?;

    let palette = ThemePalette {
        error: prompt_rgb("error", base.error)?,
        success: prompt_rgb("success", base.success)?,
        warning: prompt_rgb("warning", base.warning)?,
        info: prompt_rgb("info", base.info)?,
        header: prompt_rgb("header", base.header)?,
        highlight: prompt_rgb("highlight", base.highlight)?,
        dim: prompt_rgb("dim", base.dim)?,
    };

    let key = config.set_custom_theme(&name, ThemeFile::from_palette(palette));
    config.set_theme(&key);
    show_theme_preview(&key, palette);
    Ok(())
}

fn choose_theme_seed(config: &AppConfig) -> AppResult<ThemePalette> {
    let entries = theme_seed_entries(config);
    let current = config.theme();
    let default = entries
        .iter()
        .position(|(_, _, name)| name == &current)
        .unwrap_or(0);
    let labels = entries
        .iter()
        .map(|(label, _, _)| label.as_str())
        .collect::<Vec<_>>();

    let idx = Select::with_theme(&prompt_theme())
        .with_prompt("start from theme")
        .items(&labels)
        .default(default)
        .interact()?;
    Ok(entries[idx].1)
}

fn prompt_custom_theme_name(config: &AppConfig, default_name: &str) -> AppResult<String> {
    loop {
        let value = Input::<String>::with_theme(&prompt_theme())
            .with_prompt("custom theme name")
            .default(default_name.to_string())
            .interact_text()?;
        let value = value.trim();
        let key = theme_key(value);
        if value.is_empty() {
            println!("{}", Colors::warning("[warn] theme name cannot be empty"));
        } else if Colors::built_in_theme(&key).is_some() {
            println!(
                "{}",
                Colors::warning("[warn] custom theme name conflicts with a built-in theme")
            );
        } else if config.custom_themes().contains_key(&key)
            && !select_bool("overwrite existing custom theme?", false)?
        {
            println!("{}", Colors::dim("[skip] choose a different theme name"));
        } else {
            return Ok(value.to_string());
        }
    }
}

fn theme_entries(config: &AppConfig) -> Vec<(String, Option<String>)> {
    let mut entries = Colors::built_in_themes()
        .iter()
        .map(|theme| {
            (
                theme_menu_label(theme.name, theme.description),
                Some(theme.name.to_string()),
            )
        })
        .collect::<Vec<_>>();

    let mut custom = config.custom_themes().keys().cloned().collect::<Vec<_>>();
    custom.sort();
    for name in custom {
        entries.push((format!("{name} - custom RGB"), Some(name)));
    }

    entries.push(("create custom RGB theme".to_string(), None));
    entries
}

fn theme_seed_entries(config: &AppConfig) -> Vec<(String, ThemePalette, String)> {
    let mut entries = Colors::built_in_themes()
        .iter()
        .map(|theme| {
            let palette = if theme.name == "auto" {
                Colors::built_in_theme("auto").unwrap_or(theme.palette)
            } else {
                theme.palette
            };
            (
                theme_menu_label(theme.name, theme.description),
                palette,
                theme.name.to_string(),
            )
        })
        .collect::<Vec<_>>();

    let mut custom = config.custom_themes().keys().cloned().collect::<Vec<_>>();
    custom.sort();
    for name in custom {
        if let Some(palette) = config.theme_palette_by_name(&name) {
            entries.push((format!("{name} - custom RGB"), palette, name));
        }
    }

    entries
}

fn show_theme_preview(name: &str, palette: ThemePalette) {
    Colors::set_theme(palette);
    println!(
        "{}",
        Colors::header(format!("[theme] {}", theme_name_with_scheme(name)))
    );
    println!(
        "{}  {}  {}  {}  {}  {}",
        Colors::success("[ok]"),
        Colors::warning("[warn]"),
        Colors::error("[err]"),
        Colors::info("[info]"),
        Colors::highlight("highlight"),
        Colors::dim("dim")
    );
}

fn theme_menu_label(name: &str, description: &str) -> String {
    if name == "auto" {
        format!(
            "{name} - {description} (detected {})",
            Colors::system_scheme()
        )
    } else {
        format!("{name} - {description}")
    }
}

fn theme_key(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}
