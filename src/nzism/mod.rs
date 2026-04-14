use crate::scanner::ScanResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NzismReport {
    pub scan_date: String,
    pub total_targets: usize,
    pub total_scanned: usize,
    pub total_errors: usize,
    pub total_pqc_supported: usize,
    pub total_tls13: usize,
    pub total_cdn_detected: usize,
    pub pqc_percentage: f64,
    pub tls13_percentage: f64,
    pub cdn_percentage: f64,
    pub sectors: Vec<SectorSummary>,
    pub nzism_mapping: NzismMapping,
    pub results: Vec<ScanResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorSummary {
    pub name: String,
    pub total: usize,
    pub scanned: usize,
    pub pqc_supported: usize,
    pub tls13_supported: usize,
    pub cdn_detected: usize,
    pub errors: usize,
    pub pqc_percentage: f64,
    pub tls13_percentage: f64,
    pub cdn_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NzismMapping {
    pub point1_monitor: String,
    pub point2_inventory_data: String,
    pub point3_inventory_systems: String,
    pub point4_prioritise: String,
    pub point5_migration_plan: String,
}

pub fn build_report(results: &[ScanResult]) -> NzismReport {
    let scan_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let total_targets = results.len();
    let total_errors = results.iter().filter(|r| r.error.is_some()).count();
    let total_scanned = total_targets - total_errors;

    let total_pqc_supported = results
        .iter()
        .filter(|r| r.tls.as_ref().map(|t| t.pqc_supported).unwrap_or(false))
        .count();

    let total_tls13 = results
        .iter()
        .filter(|r| {
            r.tls
                .as_ref()
                .and_then(|t| t.tls_version.as_ref())
                .map(|v| v.contains("1_3") || v.contains("1.3") || v.contains("Tls13") || v.contains("TLS13"))
                .unwrap_or(false)
        })
        .count();

    let total_cdn_detected = results
        .iter()
        .filter(|r| r.cdn.as_ref().map(|c| c.detected).unwrap_or(false))
        .count();

    let pqc_percentage = if total_scanned > 0 {
        (total_pqc_supported as f64 / total_scanned as f64) * 100.0
    } else {
        0.0
    };

    let tls13_percentage = if total_scanned > 0 {
        (total_tls13 as f64 / total_scanned as f64) * 100.0
    } else {
        0.0
    };

    let cdn_percentage = if total_scanned > 0 {
        (total_cdn_detected as f64 / total_scanned as f64) * 100.0
    } else {
        0.0
    };

    // Build sector summaries
    let mut sector_map: HashMap<String, Vec<&ScanResult>> = HashMap::new();
    for result in results {
        sector_map
            .entry(result.sector.clone())
            .or_default()
            .push(result);
    }

    let sector_order = [
        "Communications & Data",
        "Defence",
        "Energy",
        "Finance",
        "Health",
        "Transport",
        "Drinking Water & Wastewater",
    ];

    let mut sectors = Vec::new();
    for sector_name in &sector_order {
        if let Some(sector_results) = sector_map.get(*sector_name) {
            let total = sector_results.len();
            let errors = sector_results.iter().filter(|r| r.error.is_some()).count();
            let scanned = total - errors;
            let pqc = sector_results
                .iter()
                .filter(|r| r.tls.as_ref().map(|t| t.pqc_supported).unwrap_or(false))
                .count();
            let tls13 = sector_results
                .iter()
                .filter(|r| {
                    r.tls
                        .as_ref()
                        .and_then(|t| t.tls_version.as_ref())
                        .map(|v| v.contains("1_3") || v.contains("1.3") || v.contains("Tls13") || v.contains("TLS13"))
                        .unwrap_or(false)
                })
                .count();
            let cdn = sector_results
                .iter()
                .filter(|r| r.cdn.as_ref().map(|c| c.detected).unwrap_or(false))
                .count();

            sectors.push(SectorSummary {
                name: sector_name.to_string(),
                total,
                scanned,
                pqc_supported: pqc,
                tls13_supported: tls13,
                cdn_detected: cdn,
                errors,
                pqc_percentage: if scanned > 0 {
                    (pqc as f64 / scanned as f64) * 100.0
                } else {
                    0.0
                },
                tls13_percentage: if scanned > 0 {
                    (tls13 as f64 / scanned as f64) * 100.0
                } else {
                    0.0
                },
                cdn_percentage: if scanned > 0 {
                    (cdn as f64 / scanned as f64) * 100.0
                } else {
                    0.0
                },
            });
        }
    }

    let nzism_mapping = NzismMapping {
        point1_monitor: "NZISM 2.4 Point 1 requires agencies to monitor PQC developments \
            and follow GCSB updates. This is not directly measurable by an external scan. \
            Note: The GCSB has not yet approved any PQC algorithms for NZISM use."
            .to_string(),
        point2_inventory_data: "NZISM 2.4 Point 2 requires maintaining an inventory of \
            sensitive and critical datasets with long confidentiality requirements. This \
            requires internal assessment and is not measurable by external scanning."
            .to_string(),
        point3_inventory_systems: format!(
            "NZISM 2.4 Point 3 requires maintaining an inventory of systems using \
            public-key cryptography and identifying those vulnerable to quantum attack. \
            This scan directly addresses this point: of {} scanned endpoints across NZ \
            critical infrastructure, {:.1}% support PQC key exchange and {:.1}% support TLS 1.3. \
            {:.1}% of endpoints are behind CDNs, meaning their PQC status may reflect CDN \
            configuration rather than the organisation's own cryptographic posture.",
            total_scanned,
            pqc_percentage,
            tls13_percentage,
            cdn_percentage,
        ),
        point4_prioritise: "NZISM 2.4 Point 4 requires prioritising systems for migration \
            based on asset value, sensitivity, dependencies, and data longevity. The \
            sector-by-sector breakdown in this report provides a starting point for \
            prioritisation across NZ critical infrastructure."
            .to_string(),
        point5_migration_plan: "NZISM 2.4 Point 5 requires developing a migration plan so \
            that systems can transition once approved PQC algorithms are available. \
            New Zealand has not set a migration deadline, unlike Australia (plan by end \
            2026, migrate by 2030), the UK (2035), and the US (2035). The absence of a \
            deadline does not reduce the urgency \u{2014} the harvest-now-decrypt-later \
            threat is active today."
            .to_string(),
    };

    NzismReport {
        scan_date,
        total_targets,
        total_scanned,
        total_errors,
        total_pqc_supported,
        total_tls13,
        total_cdn_detected,
        pqc_percentage,
        tls13_percentage,
        cdn_percentage,
        sectors,
        nzism_mapping,
        results: results.to_vec(),
    }
}
