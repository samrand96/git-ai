use std::fs;

use crate::utils::colors::Colors;
use crate::utils::errors::AppResult;

pub fn write_review_output(save_markdown: bool, output_file: &str, review: &str) -> AppResult<()> {
    if save_markdown {
        fs::write(output_file, review)?;
        println!(
            "{}",
            Colors::success(format!("[ok] review saved to: {output_file}"))
        );
    } else {
        let formatted = Colors::format_cli_output(review);
        let separator = "=".repeat(60);
        println!("{}", Colors::header(format!("\n{separator}")));
        println!("{}", Colors::header("  REVIEW REPORT  "));
        println!("{}", Colors::header(format!("{separator}\n")));
        println!("{formatted}");
        println!("{}", Colors::header(format!("\n{separator}")));
        println!("{}", Colors::header("  REVIEW COMPLETE  "));
        println!("{}", Colors::header(separator));
    }

    Ok(())
}

pub fn markdown_output_path(output: Option<String>, default_name: String) -> String {
    let path = output.unwrap_or(default_name);
    if path.to_ascii_lowercase().ends_with(".md") {
        path
    } else {
        format!("{path}.md")
    }
}
