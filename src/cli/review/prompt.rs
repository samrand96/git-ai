use crate::utils::git::DiffFileStat;

use super::limits::DiffLimits;
use super::notes::ReviewNotes;

pub struct ReviewPromptInput<'a> {
    pub current_branch: &'a str,
    pub target_branch: Option<&'a str>,
    pub fast: bool,
    pub stats: &'a [DiffFileStat],
    pub excluded_paths: &'a [String],
    pub limits: DiffLimits,
    pub notes: &'a ReviewNotes,
    pub diff: &'a str,
}

pub fn build_review_prompt(input: &ReviewPromptInput<'_>) -> String {
    let summary = input
        .stats
        .iter()
        .map(|stat| {
            format!(
                "- {} (+{} -{})",
                stat.path,
                count_label(stat.added),
                count_label(stat.deleted)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let exclusions = if input.excluded_paths.is_empty() {
        "None".to_string()
    } else {
        input
            .excluded_paths
            .iter()
            .map(|path| format!("- {path}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let flagged_files = flagged_file_summary(input.stats, input.limits);
    let limit_warnings = input
        .limits
        .warnings(input.stats)
        .into_iter()
        .map(|warning| format!("- {warning}"))
        .collect::<Vec<_>>()
        .join("\n");
    let limit_warnings = if limit_warnings.is_empty() {
        "None".to_string()
    } else {
        limit_warnings
    };
    let task = input.notes.task.as_deref().unwrap_or("Not provided.");
    let comments = input.comments_text();
    let mode = if input.fast {
        "Fast local review of current uncommitted staged and unstaged changes.".to_string()
    } else {
        format!(
            "PR readiness review against target branch {}.",
            input.target_branch.unwrap_or("unknown")
        )
    };
    let target = input.target_branch.unwrap_or("not used in --fast mode");
    let sections = if input.fast {
        "SUMMARY, TASK COMPLETION, COMMENT RESOLUTION, FINDINGS, RISKS, TEST GAPS, and NEXT STEPS"
    } else {
        "SUMMARY, MERGE READINESS, TASK COMPLETION, COMMENT RESOLUTION, BLOCKING FINDINGS, RISKS, TEST GAPS, and NEXT STEPS"
    };

    format!(
        "You are a senior engineer performing a production-grade code review.\n\n\
Review mode: {mode}\n\
Source branch: {}\n\
Target branch: {target}\n\n\
Task or ticket requirements:\n{task}\n\n\
Previous review comments or feedback:\n{comments}\n\n\
Changed file summary:\n{summary}\n\n\
Large-diff policy:\n\
- Enabled: {}\n\
- Changed-line threshold: {}\n\
- Max changed files: {}\n\
- Max added lines per file: {}\n\
- Max deleted lines per file: {}\n\n\
Flagged large/binary files:\n{flagged_files}\n\n\
Excluded large/binary files:\n{exclusions}\n\n\
Diff limit warnings:\n{limit_warnings}\n\n\
Review boundaries:\n\
Do not claim to have reviewed excluded files. Mention excluded files as skipped review risk when relevant.\n\n\
Return Markdown only. Avoid emojis and decorative iconography.\n\
Use these sections: {sections}.\n\
Use severity labels CRITICAL/HIGH/MEDIUM/LOW/INFO. In PR mode, clearly say whether the branch looks ready for PR merge. In --fast mode, focus on concrete local findings in the uncommitted diff.\n\n\
Diff to review:\n```diff\n{diff}\n```",
        input.current_branch,
        if input.limits.enabled { "yes" } else { "no" },
        input.limits.large_threshold,
        input.limits.max_changed_files,
        input.limits.max_added_lines,
        input.limits.max_deleted_lines,
        diff = input.diff
    )
}

impl ReviewPromptInput<'_> {
    fn comments_text(&self) -> &str {
        self.notes.comments.as_deref().unwrap_or("Not provided.")
    }
}

fn count_label(value: Option<u64>) -> String {
    value
        .map(|count| count.to_string())
        .unwrap_or_else(|| "binary".to_string())
}

fn flagged_file_summary(stats: &[DiffFileStat], limits: DiffLimits) -> String {
    if !limits.enabled {
        return "Large-diff checks disabled.".to_string();
    }

    let summary = stats
        .iter()
        .filter_map(|stat| {
            let reasons = limits.file_reasons(stat);
            if reasons.is_empty() {
                None
            } else {
                Some(format!("- {} [{}]", stat.path, reasons.join(", ")))
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if summary.is_empty() {
        "None".to_string()
    } else {
        summary
    }
}
