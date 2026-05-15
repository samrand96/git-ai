use crate::utils::git::DiffFileStat;

#[derive(Debug, Clone, Copy)]
pub struct DiffLimits {
    pub enabled: bool,
    pub large_threshold: u64,
    pub max_changed_files: usize,
    pub max_added_lines: u64,
    pub max_deleted_lines: u64,
}

impl DiffLimits {
    pub fn warnings(&self, stats: &[DiffFileStat]) -> Vec<String> {
        if !self.enabled {
            return vec!["Large-diff checks were disabled by --no-large-check.".to_string()];
        }

        let mut warnings = Vec::new();
        if stats.len() > self.max_changed_files {
            warnings.push(format!(
                "Changed file count {} exceeds max {}.",
                stats.len(),
                self.max_changed_files
            ));
        }
        warnings
    }

    pub fn file_reasons(&self, stat: &DiffFileStat) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }

        let mut reasons = Vec::new();
        if stat.added.is_none() || stat.deleted.is_none() {
            reasons.push("binary".to_string());
        }
        if stat.changed_lines() >= self.large_threshold {
            reasons.push(format!("changed >= {}", self.large_threshold));
        }
        if stat.added.unwrap_or(0) >= self.max_added_lines {
            reasons.push(format!("added >= {}", self.max_added_lines));
        }
        if stat.deleted.unwrap_or(0) >= self.max_deleted_lines {
            reasons.push(format!("deleted >= {}", self.max_deleted_lines));
        }
        reasons
    }
}
