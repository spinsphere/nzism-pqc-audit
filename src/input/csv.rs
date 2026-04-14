use super::Target;
use anyhow::{Context, Result};
use std::path::Path;

pub fn load_csv(path: &Path) -> Result<Vec<Target>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("Failed to open CSV file: {}", path.display()))?;

    let mut targets = Vec::new();

    for result in reader.records() {
        let record = result?;

        let entity_name = record.get(0).unwrap_or("").to_string();
        let sector = record.get(1).unwrap_or("").to_string();
        let sub_sector = record.get(2).unwrap_or("").to_string();
        let domain = record.get(3).unwrap_or("").to_string();
        let scan_targets_str = record.get(4).unwrap_or("");
        let notes = record.get(6).unwrap_or("").to_string();

        for scan_target in scan_targets_str.split(',') {
            let scan_target = scan_target.trim();
            if scan_target.is_empty() {
                continue;
            }

            let (host, port) = if let Some(colon_pos) = scan_target.rfind(':') {
                let h = &scan_target[..colon_pos];
                let p = scan_target[colon_pos + 1..]
                    .parse::<u16>()
                    .unwrap_or(443);
                (h.to_string(), p)
            } else {
                (scan_target.to_string(), 443)
            };

            targets.push(Target {
                entity_name: entity_name.clone(),
                sector: sector.clone(),
                sub_sector: sub_sector.clone(),
                domain: domain.clone(),
                host,
                port,
                notes: notes.clone(),
            });
        }
    }

    Ok(targets)
}
