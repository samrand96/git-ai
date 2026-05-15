use std::io;

use crate::utils::colors::Colors;
use crate::utils::errors::AppResult;

use super::ReviewArgs;

#[derive(Debug, Clone)]
pub struct ReviewNotes {
    pub task: Option<String>,
    pub comments: Option<String>,
}

pub fn collect_notes(args: &ReviewArgs) -> AppResult<ReviewNotes> {
    let task = if args.task {
        Some(read_multiline("task or ticket requirements")?)
    } else {
        None
    };
    let comments = if args.comments {
        Some(read_multiline("previous PR comments or reviewer feedback")?)
    } else {
        None
    };

    Ok(ReviewNotes { task, comments })
}

fn read_multiline(label: &str) -> AppResult<String> {
    println!("{}", Colors::header(format!("[input] paste {label}")));
    println!(
        "{}",
        Colors::dim("Finish with a line containing only END. Blank lines are kept.")
    );

    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.eq_ignore_ascii_case("END") {
            break;
        }
        lines.push(trimmed.to_string());
    }

    let text = lines.join("\n").trim().to_string();
    if text.is_empty() {
        Ok("Not provided.".to_string())
    } else {
        Ok(text)
    }
}
