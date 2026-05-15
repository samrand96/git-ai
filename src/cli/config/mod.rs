mod display;
mod model_select;
mod prompts;
mod provider_select;
mod theme_select;

use clap::Args;
use dialoguer::Password;

use crate::config::AppConfig;
use crate::utils::colors::Colors;
use crate::utils::env::set_env_var;
use crate::utils::errors::{AppError, AppResult};

use display::{masked_api_key_status, print_current};
use model_select::select_model;
use prompts::{input_with_default, provider_api_env_key, select_bool};
use provider_select::{select_commit_format, select_provider};
use theme_select::{choose_theme, theme_names};

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    #[arg(long, help = "Edit config interactively")]
    pub interactive: bool,
    #[arg(
        long,
        num_args = 2,
        value_names = ["KEY", "VALUE"],
        help = "Set a global config value"
    )]
    pub set: Option<Vec<String>>,
    #[arg(
        long,
        num_args = 3,
        value_names = ["PROVIDER", "KEY", "VALUE"],
        help = "Set a provider config value"
    )]
    pub set_provider: Option<Vec<String>>,
    #[arg(long, help = "Choose and save a color theme interactively")]
    pub theme: bool,
}

pub fn run(mut config: AppConfig, args: ConfigArgs) -> AppResult<()> {
    if args.theme {
        choose_theme(&mut config)?;
        config.save()?;
        return Ok(());
    }

    if args.interactive || (args.set.is_none() && args.set_provider.is_none()) {
        interactive_edit(&mut config)?;
    }

    if let Some(values) = args.set
        && values.len() == 2
    {
        set_global_value(&mut config, &values[0], &values[1])?;
    }

    if let Some(values) = args.set_provider
        && values.len() == 3
    {
        let provider = values[0].to_lowercase();
        let key = values[1].clone();
        let value = values[2].clone();
        config.set_provider_option(&provider, &key, &value)?;
        config.save()?;
        println!(
            "{}",
            Colors::success(format!("[ok] set [{provider}].{key} = {value}"))
        );
    }

    Ok(())
}

fn set_global_value(config: &mut AppConfig, key: &str, value: &str) -> AppResult<()> {
    match key.to_uppercase().as_str() {
        "PROVIDER" => {
            config.set_provider(value);
            config.save()?;
            println!(
                "{}",
                Colors::success(format!("[ok] set PROVIDER = {value}"))
            );
        }
        "COMMIT_FORMAT" => {
            config.set_commit_format(value);
            config.save()?;
            println!(
                "{}",
                Colors::success(format!("[ok] set COMMIT_FORMAT = {value}"))
            );
        }
        "THEME" => {
            if config.theme_palette_by_name(value).is_none() {
                return Err(AppError::Config(format!(
                    "Unknown theme '{value}'. Available: {}.",
                    theme_names(config)
                )));
            }
            config.set_theme(value);
            config.save()?;
            println!("{}", Colors::success(format!("[ok] set THEME = {value}")));
        }
        _ => {
            return Err(AppError::Config(format!(
                "Unknown global key '{}'. Supported: PROVIDER, COMMIT_FORMAT, THEME.",
                key.to_uppercase()
            )));
        }
    }

    Ok(())
}

fn interactive_edit(config: &mut AppConfig) -> AppResult<()> {
    print_current(config);

    println!(
        "{}",
        Colors::header("\n[cfg] global settings (press Enter to keep current value)")
    );
    let provider = select_provider(config)?;
    config.set_provider(&provider);

    let commit_format = select_commit_format(config)?;
    config.set_commit_format(&commit_format);

    choose_theme(config)?;

    let provider = config.provider();
    let mut provider_settings = config.provider_settings(&provider, None, None);
    println!("{}", Colors::header("\n[cfg] provider settings"));

    if provider_settings.requires_api_key {
        println!("{}", Colors::info(format!("[provider] {provider}")));
        println!(
            "{}",
            Colors::dim(format!(
                "  API_KEY: {}",
                masked_api_key_status(&provider_settings)
            ))
        );
        let update_key =
            provider_settings.api_key.is_none() || select_bool("replace stored API key?", false)?;
        if update_key {
            let api_env_key = provider_api_env_key(&provider);
            let api_key = Password::with_theme(&prompts::prompt_theme())
                .with_prompt(format!("API_KEY (stored as {api_env_key})"))
                .allow_empty_password(true)
                .interact()?;
            if !api_key.trim().is_empty() {
                set_env_var(config.env_path(), &api_env_key, api_key.trim())?;
                println!("{}", Colors::success(format!("[ok] updated {api_env_key}")));
                provider_settings = config.provider_settings(&provider, None, None);
                println!(
                    "{}",
                    Colors::dim(format!(
                        "  API_KEY: {}",
                        masked_api_key_status(&provider_settings)
                    ))
                );
            }
        }
    }

    let base_url = input_with_default("BASE_URL", &provider_settings.base_url)?;
    config.set_provider_option(&provider, "BASE_URL", &base_url)?;
    provider_settings = config.provider_settings(&provider, None, None);

    let model = select_model(config, &provider, &provider_settings)?;
    config.set_provider_option(&provider, "MODEL", &model)?;

    config.save()?;
    println!("{}", Colors::success("[ok] settings updated"));
    Ok(())
}
