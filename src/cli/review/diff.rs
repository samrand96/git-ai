use dialoguer::{MultiSelect, Select, theme::SimpleTheme};

use crate::utils::colors::Colors;
use crate::utils::errors::{AppError, AppResult};
use crate::utils::git::DiffFileStat;

use super::limits::DiffLimits;

const LARGE_FILTERED_DIFF_BYTES: usize = 400_000;

pub fn print_diff_summary(comparison: &str, stats: &[DiffFileStat], limits: DiffLimits) {
    let total_files = stats.len();
    let total_changed = stats.iter().map(DiffFileStat::changed_lines).sum::<u64>();
    println!(
        "{}",
        Colors::header(format!(
            "[diff] {comparison}: {total_files} file(s), {total_changed} changed line(s)"
        ))
    );

    for stat in stats.iter().take(20) {
        println!(
            "{}",
            Colors::dim(format!(
                "  {} (+{} -{})",
                stat.path,
                count_label(stat.added),
                count_label(stat.deleted)
            ))
        );
    }
    if stats.len() > 20 {
        println!(
            "{}",
            Colors::dim(format!("  ... {} more file(s)", stats.len() - 20))
        );
    }

    for warning in limits.warnings(stats) {
        println!("{}", Colors::warning(format!("[warn] {warning}")));
    }
}

pub fn select_large_file_exclusions(
    stats: &[DiffFileStat],
    limits: DiffLimits,
) -> AppResult<Vec<String>> {
    if !limits.enabled {
        println!(
            "{}",
            Colors::warning("[warn] large-diff checks disabled; no files will be auto-excluded")
        );
        return Ok(Vec::new());
    }

    let large = stats
        .iter()
        .map(|stat| (stat, limits.file_reasons(stat)))
        .filter(|(_, reasons)| !reasons.is_empty())
        .collect::<Vec<_>>();

    if large.is_empty() {
        return Ok(Vec::new());
    }

    println!(
        "{}",
        Colors::warning(format!(
            "[warn] {} large/binary file diff(s) detected; select entries to exclude from AI review",
            large.len()
        ))
    );
    let labels = large
        .iter()
        .map(|(stat, reasons)| {
            format!(
                "{} (+{} -{}) [{}]",
                stat.path,
                count_label(stat.added),
                count_label(stat.deleted),
                reasons.join(", ")
            )
        })
        .collect::<Vec<_>>();
    let defaults = vec![true; labels.len()];
    let selected = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt("exclude from AI payload")
        .items(&labels)
        .defaults(&defaults)
        .interact()?;

    Ok(selected
        .into_iter()
        .map(|idx| large[idx].0.path.clone())
        .collect())
}

pub fn confirm_limit_warnings(stats: &[DiffFileStat], limits: DiffLimits) -> AppResult<()> {
    let warnings = limits.warnings(stats);
    if !limits.enabled || warnings.is_empty() {
        return Ok(());
    }

    let items = ["continue with review", "abort"];
    let idx = Select::with_theme(&SimpleTheme)
        .with_prompt("diff limit action")
        .items(items)
        .default(0)
        .interact()?;
    if idx == 1 {
        return Err(AppError::Config(
            "Review aborted because diff policy limits were exceeded.".to_string(),
        ));
    }
    Ok(())
}

pub fn confirm_large_filtered_diff(diff: &str, limits: DiffLimits) -> AppResult<()> {
    if !limits.enabled {
        return Ok(());
    }

    if diff.len() <= LARGE_FILTERED_DIFF_BYTES {
        return Ok(());
    }

    println!(
        "{}",
        Colors::warning(format!(
            "[warn] filtered diff is still large ({} bytes)",
            diff.len()
        ))
    );
    let items = ["continue with this diff", "abort"];
    let idx = Select::with_theme(&SimpleTheme)
        .with_prompt("large diff action")
        .items(items)
        .default(1)
        .interact()?;
    if idx == 1 {
        return Err(AppError::Config(
            "Review aborted because the filtered diff is too large.".to_string(),
        ));
    }
    Ok(())
}

fn count_label(value: Option<u64>) -> String {
    value
        .map(|count| count.to_string())
        .unwrap_or_else(|| "binary".to_string())
}
