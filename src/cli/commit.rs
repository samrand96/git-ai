use clap::Args;
use dialoguer::{Select, theme::SimpleTheme};
use regex::Regex;
use std::io;

use crate::config::AppConfig;
use crate::services::ai::{AiClient, Message};
use crate::utils::cleanup::clean_commit_message;
use crate::utils::colors::Colors;
use crate::utils::errors::AppResult;
use crate::utils::git::{commit, get_branch, get_diff, push, stage_all};

#[derive(Args, Debug, Clone)]
pub struct CommitArgs {
    #[arg(long, value_parser = ["detailed", "one-line"], help = "Commit message format")]
    pub format: Option<String>,
    #[arg(long, help = "Provider to use (overrides config)")]
    pub provider: Option<String>,
    #[arg(long, help = "Model to use (overrides config)")]
    pub model: Option<String>,
    #[arg(long, help = "Push after commit")]
    pub push: bool,
    #[arg(long, help = "Skip git hooks (pass --no-verify to git commit)")]
    pub no_verify: bool,
}

pub fn run(mut config: AppConfig, args: CommitArgs) -> AppResult<()> {
    if let Some(format) = &args.format {
        config.set_commit_format(format);
        config.save()?;
    }

    let provider = args.provider.clone().unwrap_or_else(|| config.provider());
    config.ensure_provider_ready(&provider)?;
    let ai = AiClient::new(config.clone())?;

    stage_all()?;
    let diff = get_diff(true, None)?;
    if diff.trim().is_empty() {
        println!("{}", Colors::info("[skip] no staged changes to commit"));
        return Ok(());
    }

    let branch = get_branch()?;
    let prefix = get_ticket_prefix(&branch);
    let short = config.commit_format() == "one-line";

    let system_msg = "You are an expert Git commit assistant. When responding, return only the commit message text itself—no extra explanation, quotes, or formatting. \
First, inspect the current Git branch name. If it begins with a ticket code matching the pattern LETTERS-DIGITS (for example ABC-123 or EL-2024), capture that exact code and place it at the very start of your message, followed by a colon and a space. If no ticket code is present, do not include any prefix. \
Next, identify the primary change or task implied by the branch name and present it as the first action in your commit message. Then, describe any secondary updates, fixes, or refactoring included in this commit. \
Use a natural, professional tone that reads like a teammate clearly explaining the work you’ve done. Use bullet points to separate multiple actions if they exist.";

    let user_msg = format!(
        "Branch: {branch}\nWrite a {} commit message for these changes:\n\n{diff}",
        if short {
            "one-line"
        } else {
            "detailed, human-friendly"
        }
    );

    println!("{}", Colors::header("[ai] generating commit message"));
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: system_msg.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: user_msg.clone(),
        },
    ];

    let mut commit_msg =
        ai.generate(&provider, &user_msg, Some(&messages), args.model.as_deref())?;
    commit_msg = clean_commit_message(&commit_msg);

    if !prefix.is_empty() && !commit_msg.starts_with(&prefix) {
        commit_msg = format!("{prefix}: {commit_msg}");
    }

    println!("{}", Colors::success("\n[ok] generated commit message"));
    println!("{}", Colors::highlight(&commit_msg));
    println!();

    if select_edit_action()? {
        println!(
            "{}",
            Colors::dim("Enter new commit message. End with an empty line:")
        );
        let mut lines = Vec::new();
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                break;
            }
            lines.push(trimmed.to_string());
        }
        if !lines.is_empty() {
            commit_msg = lines.join("\n");
        }
    }

    commit(&commit_msg, args.no_verify)?;
    println!("{}", Colors::success("[ok] committed successfully"));

    if args.push {
        push()?;
        println!("{}", Colors::success("[ok] pushed to remote"));
    } else {
        println!(
            "{}",
            Colors::dim("[hint] use --push to push after committing")
        );
    }

    Ok(())
}

fn get_ticket_prefix(branch: &str) -> String {
    let re = Regex::new(r"^([A-Za-z]+-\d+)").unwrap();
    re.captures(branch)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_default()
}

fn select_edit_action() -> AppResult<bool> {
    let actions = ["commit generated message", "edit before commit"];
    let idx = Select::with_theme(&SimpleTheme)
        .with_prompt("next action")
        .items(actions)
        .default(0)
        .interact()?;
    Ok(idx == 1)
}
