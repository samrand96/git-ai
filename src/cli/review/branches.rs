use std::collections::BTreeSet;

use dialoguer::{Select, theme::SimpleTheme};

use crate::utils::colors::Colors;
use crate::utils::errors::{AppError, AppResult};
use crate::utils::git;

enum BranchAction {
    Up,
    Enter(String),
    Pick(String),
}

struct BranchEntry {
    label: String,
    action: BranchAction,
}

pub fn fetch_origin_best_effort() {
    println!("{}", Colors::info("[git] fetching remotes"));
    match git::fetch_remotes() {
        Ok(()) => println!("{}", Colors::success("[ok] remotes updated")),
        Err(err) => println!(
            "{}",
            Colors::warning(format!(
                "[warn] fetch failed; using local remote refs: {}",
                err.user_message()
            ))
        ),
    }
}

pub fn resolve_target_branch(
    branches: &[String],
    current_branch: &str,
    requested: Option<&str>,
) -> AppResult<String> {
    if branches.is_empty() {
        return Err(AppError::Git(
            "No origin branches found. Run git fetch or check remote configuration.".to_string(),
        ));
    }

    if let Some(target) = requested.map(str::trim).filter(|target| !target.is_empty()) {
        return normalize_requested_target(branches, target);
    }

    select_origin_branch(branches, current_branch)
}

fn select_origin_branch(branches: &[String], current_branch: &str) -> AppResult<String> {
    let mut path = Vec::<String>::new();
    loop {
        let entries = branch_entries(branches, &path, current_branch);
        let labels = entries
            .iter()
            .map(|entry| entry.label.as_str())
            .collect::<Vec<_>>();
        let idx = Select::with_theme(&SimpleTheme)
            .with_prompt(branch_prompt(&path))
            .items(&labels)
            .default(0)
            .interact()?;

        match &entries[idx].action {
            BranchAction::Up => {
                path.pop();
            }
            BranchAction::Enter(segment) => path.push(segment.clone()),
            BranchAction::Pick(branch) => return Ok(branch.clone()),
        }
    }
}

fn branch_entries(branches: &[String], path: &[String], current_branch: &str) -> Vec<BranchEntry> {
    let mut dirs = BTreeSet::new();
    let mut picks = Vec::new();
    let prefix = path.join("/");

    for branch in branches {
        let Some(relative) = branch.strip_prefix("origin/") else {
            continue;
        };
        if !prefix.is_empty() && relative != prefix && !relative.starts_with(&format!("{prefix}/"))
        {
            continue;
        }

        let remainder = if prefix.is_empty() {
            relative
        } else {
            relative
                .strip_prefix(&format!("{prefix}/"))
                .unwrap_or_default()
        };
        if remainder.is_empty() {
            continue;
        }

        let mut parts = remainder.split('/');
        let first = parts.next().unwrap_or_default();
        if parts.next().is_some() {
            dirs.insert(first.to_string());
        } else {
            let label = branch_label(branch, relative, current_branch);
            picks.push(BranchEntry {
                label,
                action: BranchAction::Pick(branch.clone()),
            });
        }
    }

    picks.sort_by(|a, b| a.label.cmp(&b.label));
    let mut entries = Vec::new();
    if !path.is_empty() {
        entries.push(BranchEntry {
            label: "..".to_string(),
            action: BranchAction::Up,
        });
    }
    entries.extend(dirs.into_iter().map(|dir| BranchEntry {
        label: format!("{dir}/"),
        action: BranchAction::Enter(dir),
    }));
    entries.extend(picks);
    entries
}

fn branch_label(branch: &str, relative: &str, current_branch: &str) -> String {
    if relative == current_branch {
        format!("{branch}  [current]")
    } else {
        branch.to_string()
    }
}

fn branch_prompt(path: &[String]) -> String {
    if path.is_empty() {
        "PR target branch".to_string()
    } else {
        format!("origin/{}/", path.join("/"))
    }
}

fn normalize_requested_target(branches: &[String], requested: &str) -> AppResult<String> {
    let target = if requested.starts_with("origin/") {
        requested.to_string()
    } else {
        format!("origin/{requested}")
    };

    if branches.iter().any(|branch| branch == &target) {
        return Ok(target);
    }

    Err(AppError::Git(format!(
        "Target branch '{requested}' was not found under origin. Available examples: {}",
        branch_examples(branches)
    )))
}

fn branch_examples(branches: &[String]) -> String {
    branches
        .iter()
        .take(8)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ")
}
