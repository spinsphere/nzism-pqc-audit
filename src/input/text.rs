use super::Target;
use anyhow::{Context, Result};
use std::path::Path;

pub fn load_text(path: &Path) -> Result<Vec<Target>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read text file: {}", path.display()))?;

    let mut targets = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (host, port) = if let Some(colon_pos) = line.rfind(':') {
            let h = &line[..colon_pos];
            let p = line[colon_pos + 1..].parse::<u16>().unwrap_or(443);
            (h.to_string(), p)
        } else {
            (line.to_string(), 443)
        };

        targets.push(Target {
            entity_name: host.clone(),
            sector: "Unknown".to_string(),
            sub_sector: "Unknown".to_string(),
            domain: host.clone(),
            host,
            port,
            notes: String::new(),
        });
    }

    Ok(targets)
}
