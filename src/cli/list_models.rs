use crate::config::AppConfig;
use crate::services::ai::AiClient;
use crate::utils::colors::Colors;
use crate::utils::errors::AppResult;

pub fn run(config: AppConfig) -> AppResult<()> {
    let provider = config.provider();
    config.ensure_provider_ready(&provider)?;
    let ai = AiClient::new(config)?;

    println!(
        "{}",
        Colors::header(format!("[net] fetching models for {provider}"))
    );

    match ai.list_models(&provider) {
        Ok(models) => {
            println!(
                "{}",
                Colors::success(format!("[models] available models for {provider}"))
            );
            for (idx, model) in models.iter().enumerate() {
                println!("{}", Colors::highlight(format!("  {}. {}", idx + 1, model)));
            }
            if models.is_empty() {
                println!(
                    "{}",
                    Colors::warning("[warn] no models found or model list is empty")
                );
            } else {
                println!(
                    "{}",
                    Colors::dim(format!("\n[info] {} model(s) available", models.len()))
                );
                println!(
                    "{}",
                    Colors::dim("[hint] use these model names in your configuration")
                );
            }
        }
        Err(err) => {
            println!(
                "{}",
                Colors::error(format!(
                    "[err] failed to fetch models: {}",
                    err.user_message()
                ))
            );
            println!(
                "{}",
                Colors::dim("[hint] check provider configuration and network access")
            );
        }
    }

    Ok(())
}
