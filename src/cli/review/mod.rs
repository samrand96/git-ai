mod branches;
mod diff;
mod limits;
mod notes;
mod output;
mod prompt;

use chrono::Local;
use clap::Args;

use crate::config::AppConfig;
use crate::services::ai::AiClient;
use crate::utils::cleanup::clean_review_output;
use crate::utils::colors::Colors;
use crate::utils::errors::AppResult;
use crate::utils::git;

#[derive(Args, Debug, Clone)]
pub struct ReviewArgs {
    #[arg(
        long,
        help = "Review current uncommitted staged and unstaged changes without target-branch comparison"
    )]
    pub fast: bool,
    #[arg(long, help = "Skip git fetch before listing origin branches")]
    pub no_fetch: bool,
    #[arg(
        long,
        help = "PR target branch, for example develop, master, staging, or origin/develop"
    )]
    pub target: Option<String>,
    #[arg(
        long,
        default_value_t = 5_000,
        help = "Changed-line threshold for large-file exclusion prompts"
    )]
    pub large_threshold: u64,
    #[arg(
        long,
        default_value_t = 50,
        help = "Warn when the PR changes more files"
    )]
    pub max_changed_files: usize,
    #[arg(
        long,
        default_value_t = 800,
        help = "Flag files with too many added lines"
    )]
    pub max_added_lines: u64,
    #[arg(
        long,
        default_value_t = 500,
        help = "Flag files with too many deleted lines"
    )]
    pub max_deleted_lines: u64,
    #[arg(long, help = "Disable large/binary diff checks and exclusion prompts")]
    pub no_large_check: bool,
    #[arg(long, help = "Prompt for task or ticket requirements to verify")]
    pub task: bool,
    #[arg(
        long,
        alias = "coments",
        help = "Prompt for previous PR comments or reviewer feedback to verify"
    )]
    pub comments: bool,
    #[arg(long, alias = "md", help = "Save the review as a Markdown report")]
    pub markdown: bool,
    #[arg(long, help = "Markdown output file name")]
    pub output: Option<String>,
}

pub fn run(config: AppConfig, args: ReviewArgs) -> AppResult<()> {
    let provider = config.provider();
    config.ensure_provider_ready(&provider)?;
    let ai = AiClient::new(config)?;
    let limits = limits::DiffLimits {
        enabled: !args.no_large_check,
        large_threshold: args.large_threshold,
        max_changed_files: args.max_changed_files,
        max_added_lines: args.max_added_lines,
        max_deleted_lines: args.max_deleted_lines,
    };

    if !args.fast && !args.no_fetch {
        branches::fetch_origin_best_effort();
    }

    let current_branch = git::get_branch()?;
    let target_branch = if args.fast {
        None
    } else {
        let origin_branches = git::list_origin_branches()?;
        Some(branches::resolve_target_branch(
            &origin_branches,
            &current_branch,
            args.target.as_deref(),
        )?)
    };

    let stats = if let Some(target_branch) = target_branch.as_deref() {
        git::get_branch_numstat(target_branch)?
    } else {
        git::get_uncommitted_numstat()?
    };
    if stats.is_empty() {
        print_empty_review_hint(target_branch.as_deref());
        return Ok(());
    }

    let comparison = target_branch
        .as_deref()
        .map(|target_branch| format!("HEAD compared with {target_branch}"))
        .unwrap_or_else(|| "current uncommitted changes".to_string());
    diff::print_diff_summary(&comparison, &stats, limits);
    diff::confirm_limit_warnings(&stats, limits)?;
    let excluded_paths = diff::select_large_file_exclusions(&stats, limits)?;
    let review_diff = if let Some(target_branch) = target_branch.as_deref() {
        git::get_branch_diff(target_branch, &excluded_paths)?
    } else {
        git::get_uncommitted_diff(&excluded_paths)?
    };
    if review_diff.trim().is_empty() {
        println!(
            "{}",
            Colors::info("[skip] filtered diff is empty after exclusions")
        );
        return Ok(());
    }

    diff::confirm_large_filtered_diff(&review_diff, limits)?;
    let notes = notes::collect_notes(&args)?;

    let prompt = prompt::build_review_prompt(&prompt::ReviewPromptInput {
        current_branch: &current_branch,
        target_branch: target_branch.as_deref(),
        fast: args.fast,
        stats: &stats,
        excluded_paths: &excluded_paths,
        limits,
        notes: &notes,
        diff: &review_diff,
    });

    print_ai_review_status(target_branch.as_deref());
    let review = ai.generate(&provider, &prompt, None, None)?;
    let review = clean_review_output(&review);

    let default_output = {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        format!("review_{timestamp}.md")
    };
    let output_file = output::markdown_output_path(args.output.clone(), default_output);
    output::write_review_output(
        args.markdown || args.output.is_some(),
        &output_file,
        &review,
    )
}

fn print_empty_review_hint(target_branch: Option<&str>) {
    if let Some(target_branch) = target_branch {
        println!(
            "{}",
            Colors::info(format!(
                "[skip] no changes between {target_branch} and HEAD"
            ))
        );
    } else {
        println!(
            "{}",
            Colors::info("[skip] no staged or unstaged changes to review")
        );
    }
}

fn print_ai_review_status(target_branch: Option<&str>) {
    if let Some(target_branch) = target_branch {
        println!(
            "{}",
            Colors::header(format!(
                "[ai] checking PR readiness against {target_branch}"
            ))
        );
    } else {
        println!(
            "{}",
            Colors::header("[ai] reviewing current uncommitted changes")
        );
    }
}
