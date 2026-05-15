use crate::config::{AppConfig, ProviderSettings};
use crate::utils::colors::Colors;

pub fn print_current(config: &AppConfig) {
    println!("{}", Colors::header("[cfg] current global settings"));
    println!(
        "{}{}",
        Colors::info("  PROVIDER: "),
        Colors::highlight(config.provider())
    );
    println!(
        "{}{}",
        Colors::info("  COMMIT_FORMAT: "),
        Colors::highlight(config.commit_format())
    );
    println!(
        "{}{}",
        Colors::info("  THEME: "),
        Colors::highlight(theme_display_name(config))
    );

    let provider = config.provider();
    let provider_settings = config.provider_settings(&provider, None, None);
    println!(
        "{}{}",
        Colors::info("  API_KEY: "),
        Colors::highlight(masked_api_key_status(&provider_settings))
    );

    println!("{}", Colors::header("\n[cfg] provider settings"));
    for (provider, pdata) in config.providers() {
        println!("{}", Colors::success(format!("[{provider}]")));
        if let Some(model) = &pdata.model {
            println!("{}{}", Colors::dim("  MODEL: "), Colors::highlight(model));
        }
        if let Some(base_url) = &pdata.base_url {
            println!(
                "{}{}",
                Colors::dim("  BASE_URL: "),
                Colors::highlight(base_url)
            );
        }
    }
}

pub fn theme_name_with_scheme(name: &str) -> String {
    if name == "auto" {
        format!("auto ({})", Colors::system_scheme())
    } else {
        name.to_string()
    }
}

pub fn masked_api_key_status(settings: &ProviderSettings) -> String {
    if !settings.requires_api_key {
        return "not required".to_string();
    }

    match (&settings.api_key, &settings.api_key_source) {
        (Some(key), Some(source)) => format!("set ({} via {source})", mask_secret(key)),
        (Some(key), None) => format!("set ({})", mask_secret(key)),
        (None, _) => "missing".to_string(),
    }
}

fn theme_display_name(config: &AppConfig) -> String {
    theme_name_with_scheme(&config.theme())
}

fn mask_secret(secret: &str) -> String {
    let chars = secret.trim().chars().collect::<Vec<_>>();
    match chars.len() {
        0 => "missing".to_string(),
        1..=4 => "*".repeat(chars.len()),
        5..=10 => format!(
            "{}{}{}",
            chars.iter().take(2).collect::<String>(),
            "*".repeat(chars.len().saturating_sub(4).max(3)),
            chars
                .iter()
                .skip(chars.len().saturating_sub(2))
                .collect::<String>()
        ),
        len => format!(
            "{}{}{}",
            chars.iter().take(4).collect::<String>(),
            "*".repeat((len - 8).clamp(4, 12)),
            chars.iter().skip(len - 4).collect::<String>()
        ),
    }
}
