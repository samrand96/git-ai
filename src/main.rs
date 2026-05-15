use clap::{Parser, Subcommand};

use gai::cli;
use gai::config::AppConfig;
use gai::utils::colors::Colors;
use gai::utils::errors::AppResult;

#[derive(Parser)]
#[command(
    name = "gai",
    version,
    about = "Terminal-native AI Git assistant",
    long_about = "gai generates commit messages and code reviews from your Git diff, with provider-aware model configuration and saved terminal themes."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate an AI commit message, commit staged changes, and optionally push")]
    Commit(cli::commit::CommitArgs),
    #[command(about = "Review PR readiness or use --fast for local uncommitted changes")]
    Review(cli::review::ReviewArgs),
    #[command(about = "Fetch model names from the active provider")]
    ListModels,
    #[command(about = "Configure provider, model, API key, commit format, and themes")]
    Config(cli::config::ConfigArgs),
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", Colors::error("[err] command failed"));
        eprintln!("{}", err.user_message());
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let cli = Cli::parse();
    let config = AppConfig::load()?;

    match &cli.command {
        Commands::Config(args) => {
            return cli::config::run(config, args.clone());
        }
        _ => {
            let provider = command_provider(&cli.command, &config);
            if let Err(err) = config.ensure_provider_ready(&provider) {
                println!("{}", Colors::error("[err] configuration is incomplete"));
                println!(
                    "{}",
                    Colors::info(
                        "[cfg] opening interactive setup. Reconfigure later with: gai config --interactive"
                    )
                );
                let interactive = cli::config::ConfigArgs {
                    interactive: true,
                    set: None,
                    set_provider: None,
                    theme: false,
                };
                let _ = cli::config::run(config, interactive);
                println!("{}", Colors::dim(err.user_message()));
                return Ok(());
            }
        }
    }

    match cli.command {
        Commands::Commit(args) => cli::commit::run(config, args),
        Commands::Review(args) => cli::review::run(config, args),
        Commands::ListModels => cli::list_models::run(config),
        Commands::Config(_) => Ok(()),
    }
}

fn command_provider(command: &Commands, config: &AppConfig) -> String {
    match command {
        Commands::Commit(args) => args.provider.clone().unwrap_or_else(|| config.provider()),
        Commands::Review(_) | Commands::ListModels | Commands::Config(_) => config.provider(),
    }
}
