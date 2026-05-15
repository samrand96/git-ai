use dialoguer::{Input, Select, theme::SimpleTheme};

use crate::utils::colors::{Colors, Rgb};
use crate::utils::errors::AppResult;

pub fn select_bool(prompt: &str, default: bool) -> AppResult<bool> {
    let items = ["no", "yes"];
    let idx = Select::with_theme(&prompt_theme())
        .with_prompt(prompt)
        .items(items)
        .default(usize::from(default))
        .interact()?;
    Ok(idx == 1)
}

pub fn input_with_default(label: &str, default: &str) -> AppResult<String> {
    Ok(Input::<String>::with_theme(&prompt_theme())
        .with_prompt(label)
        .default(default.to_string())
        .interact_text()?
        .trim()
        .to_string())
}

pub fn prompt_rgb(label: &str, default: Rgb) -> AppResult<Rgb> {
    loop {
        let value = Input::<String>::with_theme(&prompt_theme())
            .with_prompt(format!("{label} RGB"))
            .default(Colors::format_rgb(default))
            .interact_text()?;
        match Colors::parse_rgb(&value) {
            Ok(rgb) => return Ok(rgb),
            Err(err) => println!("{}", Colors::warning(format!("[warn] {err}"))),
        }
    }
}

pub fn provider_api_env_key(provider: &str) -> String {
    format!("{}_API_KEY", provider.trim().to_uppercase())
}

pub fn prompt_theme() -> SimpleTheme {
    SimpleTheme
}
