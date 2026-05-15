use regex::Regex;

pub fn clean_ai_response(text: &str) -> String {
    if text.trim().is_empty() {
        return String::new();
    }

    let mut cleaned = text.to_string();
    let code_block = Regex::new(r"```[\w]*\n?").unwrap();
    cleaned = code_block.replace_all(&cleaned, "").to_string();
    cleaned = cleaned.replace("```", "");

    let inline_code = Regex::new(r"`([^`]+)`").unwrap();
    cleaned = inline_code.replace_all(&cleaned, "$1").to_string();

    cleaned = cleaned.trim().to_string();
    if (cleaned.starts_with('"') && cleaned.ends_with('"'))
        || (cleaned.starts_with('\'') && cleaned.ends_with('\''))
    {
        cleaned = cleaned[1..cleaned.len() - 1].to_string();
    }

    let prefixes = [
        r"^(here's?\s+(the\s+)?)?commit\s+message:?\s*",
        r"^(here's?\s+(a\s+)?)?review:?\s*",
        r"^(here's?\s+(the\s+)?)?analysis:?\s*",
        r"^response:?\s*",
        r"^answer:?\s*",
        r"^result:?\s*",
    ];

    for prefix in prefixes {
        let re = Regex::new(prefix).unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    let suffixes = [
        r"\s*\n\n.*this\s+(commit\s+)?message.*$",
        r"\s*\n\n.*hope\s+this\s+helps.*$",
        r"\s*\n\n.*let\s+me\s+know.*$",
    ];
    for suffix in suffixes {
        let re = Regex::new(suffix).unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    let extra_blank = Regex::new(r"\n\s*\n\s*\n").unwrap();
    cleaned = extra_blank.replace_all(&cleaned, "\n\n").to_string();

    cleaned.trim().to_string()
}

pub fn clean_commit_message(text: &str) -> String {
    let mut cleaned = clean_ai_response(text);
    let artifacts = [
        r"^commit\s+message:?\s*",
        r"^git\s+commit\s+message:?\s*",
        r"^\s*-\s*commit:?\s*",
        r"^\s*\*\s*commit:?\s*",
        r"^here\s+is\s+the\s+commit\s+message:?\s*",
        r"^the\s+commit\s+message\s+is:?\s*",
    ];

    for artifact in artifacts {
        let re = Regex::new(artifact).unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    cleaned = cleaned.trim().to_string();
    if cleaned.starts_with('"') && cleaned.ends_with('"') && cleaned.matches('"').count() == 2 {
        cleaned = cleaned[1..cleaned.len() - 1].to_string();
    }
    if cleaned.starts_with('\'') && cleaned.ends_with('\'') && cleaned.matches('\'').count() == 2 {
        cleaned = cleaned[1..cleaned.len() - 1].to_string();
    }

    let lines: Vec<&str> = cleaned.lines().collect();
    if lines.len() > 1 {
        let mut normalized = Vec::new();
        normalized.push(lines[0].trim().to_string());
        if lines
            .get(1)
            .map(|line| !line.trim().is_empty())
            .unwrap_or(false)
        {
            normalized.push(String::new());
        }
        for line in lines.iter().skip(1) {
            if !line.trim().is_empty() {
                normalized.push(line.trim_end().to_string());
            }
        }
        cleaned = normalized.join("\n");
    }

    cleaned.trim().to_string()
}

pub fn clean_review_output(text: &str) -> String {
    let mut cleaned = clean_ai_response(text);
    let artifacts = [
        r"^code\s+review:?\s*",
        r"^review\s+results?:?\s*",
        r"^analysis\s+results?:?\s*",
    ];
    for artifact in artifacts {
        let re = Regex::new(artifact).unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    let extra_blank = Regex::new(r"\n{3,}").unwrap();
    cleaned = extra_blank.replace_all(&cleaned, "\n\n").to_string();
    cleaned.trim().to_string()
}
