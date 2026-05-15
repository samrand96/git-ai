use std::path::Path;

pub fn load_env_files(config_env_path: Option<&Path>) {
    if let Some(path) = config_env_path {
        let _ = dotenvy::from_path(path);
    }
    let _ = dotenvy::dotenv();
}

pub fn get_env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|val| !val.trim().is_empty())
}

pub fn set_env_var(path: &Path, key: &str, value: &str) -> std::io::Result<()> {
    let mut lines = std::collections::BTreeMap::new();
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                lines.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }
    lines.insert(key.to_string(), value.to_string());

    let mut output = String::new();
    for (k, v) in lines {
        output.push_str(&format!("{k}={v}\n"));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, output)?;

    // Keep the current config session in sync with the .env file we just wrote.
    unsafe {
        std::env::set_var(key, value);
    }

    Ok(())
}
