use std::collections::BTreeMap;
use std::process::{Command, Stdio};

use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct DiffFileStat {
    pub path: String,
    pub added: Option<u64>,
    pub deleted: Option<u64>,
}

impl DiffFileStat {
    pub fn changed_lines(&self) -> u64 {
        self.added.unwrap_or(0) + self.deleted.unwrap_or(0)
    }
}

fn run_git_command(args: &[&str], input: Option<&str>) -> AppResult<String> {
    let mut command = Command::new("git");
    command.args(args);

    if let Some(input_text) = input {
        command.stdin(Stdio::piped());
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            stdin.write_all(input_text.as_bytes())?;
        }
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(AppError::Git(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }

    let output = command.output()?;
    if !output.status.success() {
        return Err(AppError::Git(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_branch() -> AppResult<String> {
    run_git_command(&["rev-parse", "--abbrev-ref", "HEAD"], None)
}

pub fn get_diff(staged: bool, commit: Option<&str>) -> AppResult<String> {
    let args = if let Some(commit_range) = commit {
        vec!["diff", commit_range]
    } else if staged {
        vec!["diff", "--staged"]
    } else {
        vec!["diff"]
    };
    run_git_command(&args, None)
}

pub fn fetch_remotes() -> AppResult<()> {
    run_git_command(&["fetch", "--all", "--prune"], None)?;
    Ok(())
}

pub fn list_origin_branches() -> AppResult<Vec<String>> {
    let output = run_git_command(
        &[
            "for-each-ref",
            "--format=%(refname:short)",
            "refs/remotes/origin",
        ],
        None,
    )?;
    let mut branches = output
        .lines()
        .map(str::trim)
        .filter(|branch| !branch.is_empty())
        .filter(|branch| !branch.ends_with("/HEAD"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    branches.sort();
    branches.dedup();
    Ok(branches)
}

pub fn get_branch_numstat(base_ref: &str) -> AppResult<Vec<DiffFileStat>> {
    let range = format!("{base_ref}...HEAD");
    let output = run_git_command(&["diff", "--numstat", &range], None)?;
    Ok(output.lines().filter_map(parse_numstat_line).collect())
}

pub fn get_branch_diff(base_ref: &str, excluded_paths: &[String]) -> AppResult<String> {
    let range = format!("{base_ref}...HEAD");
    run_diff_with_excludes(&["diff", &range], excluded_paths)
}

pub fn get_uncommitted_numstat() -> AppResult<Vec<DiffFileStat>> {
    let staged = run_git_command(&["diff", "--staged", "--numstat"], None)?;
    let unstaged = run_git_command(&["diff", "--numstat"], None)?;
    Ok(merge_numstat_outputs(&[staged, unstaged]))
}

pub fn get_uncommitted_diff(excluded_paths: &[String]) -> AppResult<String> {
    let staged = run_diff_with_excludes(&["diff", "--staged"], excluded_paths)?;
    let unstaged = run_diff_with_excludes(&["diff"], excluded_paths)?;
    Ok([staged, unstaged]
        .into_iter()
        .filter(|diff| !diff.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n"))
}

pub fn stage_all() -> AppResult<()> {
    run_git_command(&["add", "."], None)?;
    Ok(())
}

pub fn commit(message: &str, no_verify: bool) -> AppResult<()> {
    let mut args = vec!["commit"];
    if no_verify {
        args.push("--no-verify");
    }
    args.extend(["-F", "-"]);
    run_git_command(&args, Some(message))?;
    Ok(())
}

pub fn push() -> AppResult<()> {
    run_git_command(&["push"], None)?;
    Ok(())
}

fn parse_numstat_line(line: &str) -> Option<DiffFileStat> {
    let mut parts = line.splitn(3, '\t');
    let added = parts.next()?.parse::<u64>().ok();
    let deleted = parts.next()?.parse::<u64>().ok();
    let path = parts.next()?.trim().to_string();

    if path.is_empty() {
        return None;
    }

    Some(DiffFileStat {
        path,
        added,
        deleted,
    })
}

fn run_diff_with_excludes(args: &[&str], excluded_paths: &[String]) -> AppResult<String> {
    let mut owned_args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
    owned_args.push("--".to_string());
    owned_args.push(".".to_string());
    owned_args.extend(
        excluded_paths
            .iter()
            .map(|path| format!(":(exclude){path}")),
    );
    let refs = owned_args.iter().map(String::as_str).collect::<Vec<_>>();
    run_git_command(&refs, None)
}

fn merge_numstat_outputs(outputs: &[String]) -> Vec<DiffFileStat> {
    let mut files = BTreeMap::<String, (Option<u64>, Option<u64>)>::new();
    for stat in outputs
        .iter()
        .flat_map(|output| output.lines())
        .filter_map(parse_numstat_line)
    {
        files
            .entry(stat.path)
            .and_modify(|(added, deleted)| {
                *added = merge_count(*added, stat.added);
                *deleted = merge_count(*deleted, stat.deleted);
            })
            .or_insert((stat.added, stat.deleted));
    }

    files
        .into_iter()
        .map(|(path, (added, deleted))| DiffFileStat {
            path,
            added,
            deleted,
        })
        .collect()
}

fn merge_count(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left + right),
        _ => None,
    }
}
