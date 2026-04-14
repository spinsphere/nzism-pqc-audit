use crate::nzism::NzismReport;

pub fn print_summary(report: &NzismReport) {
    println!();
    println!("=== NZ Critical Infrastructure PQC Readiness Summary ===");
    println!("Scan date: {}", report.scan_date);
    println!();
    println!(
        "Total targets: {} | Scanned: {} | Errors: {}",
        report.total_targets, report.total_scanned, report.total_errors
    );
    println!(
        "PQC supported: {} ({:.1}%)",
        report.total_pqc_supported, report.pqc_percentage
    );
    println!(
        "TLS 1.3 supported: {} ({:.1}%)",
        report.total_tls13, report.tls13_percentage
    );
    println!(
        "Behind CDN: {} ({:.1}%)",
        report.total_cdn_detected, report.cdn_percentage
    );
    println!();
    println!("--- Sector Breakdown ---");
    println!(
        "{:<35} {:>5} {:>5} {:>8} {:>8} {:>8}",
        "Sector", "Total", "OK", "PQC", "TLS1.3", "CDN"
    );
    println!("{}", "-".repeat(80));

    for sector in &report.sectors {
        println!(
            "{:<35} {:>5} {:>5} {:>7.1}% {:>7.1}% {:>7.1}%",
            sector.name,
            sector.total,
            sector.scanned,
            sector.pqc_percentage,
            sector.tls13_percentage,
            sector.cdn_percentage,
        );
    }
    println!();
}
