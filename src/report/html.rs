use crate::nzism::NzismReport;
use anyhow::Result;
use std::path::Path;

pub fn write_html_report(report: &NzismReport, path: &Path) -> Result<()> {
    let html = render_html(report);
    std::fs::write(path, html)?;
    Ok(())
}

fn render_html(report: &NzismReport) -> String {
    let sector_rows: String = report
        .sectors
        .iter()
        .map(|s| {
            format!(
                r#"<tr>
                    <td>{}</td>
                    <td class="num">{}</td>
                    <td class="num">{}</td>
                    <td class="num">{} ({:.1}%)</td>
                    <td class="num">{} ({:.1}%)</td>
                    <td class="num">{} ({:.1}%)</td>
                    <td class="num">{}</td>
                </tr>"#,
                s.name,
                s.total,
                s.scanned,
                s.pqc_supported,
                s.pqc_percentage,
                s.tls13_supported,
                s.tls13_percentage,
                s.cdn_detected,
                s.cdn_percentage,
                s.errors,
            )
        })
        .collect();

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>NZ Critical Infrastructure PQC Readiness Assessment &mdash; {date}</title>
<style>
:root {{
    --primary: #00E5FF;
    --bg: #001B3D;
    --bg-card: #0a2a52;
    --border: #1a3a62;
    --text: #e0e8f0;
    --text-muted: #9EADBD;
    --dark: #0B1218;
}}
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, monospace;
    background: var(--bg);
    color: var(--text);
    line-height: 1.6;
}}
.container {{ max-width: 1100px; margin: 0 auto; padding: 2rem; }}
header {{
    background: var(--dark);
    border-bottom: 2px solid var(--primary);
    padding: 2rem 0;
}}
header h1 {{ font-size: 1.8rem; color: #fff; }}
header .subtitle {{ color: var(--text-muted); margin-top: 0.5rem; }}
.badge {{
    display: inline-block;
    background: var(--primary);
    color: var(--bg);
    padding: 0.2rem 0.8rem;
    border-radius: 3px;
    font-weight: 700;
    font-size: 0.85rem;
    margin-top: 0.5rem;
}}
section {{ margin: 2rem 0; }}
h2 {{
    color: var(--primary);
    border-bottom: 1px solid var(--border);
    padding-bottom: 0.5rem;
    margin-bottom: 1rem;
    font-size: 1.4rem;
}}
h3 {{ color: #fff; margin: 1rem 0 0.5rem; }}
.stats-grid {{
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin: 1rem 0;
}}
.stat-card {{
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 1.5rem;
    text-align: center;
}}
.stat-card .number {{
    font-size: 2.5rem;
    font-weight: 700;
    color: var(--primary);
}}
.stat-card .label {{
    color: var(--text-muted);
    font-size: 0.9rem;
    margin-top: 0.3rem;
}}
table {{
    width: 100%;
    border-collapse: collapse;
    margin: 1rem 0;
    font-size: 0.9rem;
}}
th {{
    background: var(--dark);
    color: var(--primary);
    padding: 0.75rem;
    text-align: left;
    border: 1px solid var(--border);
}}
td {{
    padding: 0.75rem;
    border: 1px solid var(--border);
}}
td.num {{ text-align: center; }}
tr:nth-child(even) {{ background: rgba(10, 42, 82, 0.5); }}
.caveat {{
    background: rgba(255, 200, 0, 0.1);
    border-left: 4px solid #ffc800;
    padding: 1rem;
    margin: 1rem 0;
    border-radius: 0 4px 4px 0;
}}
.caveat strong {{ color: #ffc800; }}
.nzism-point {{
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 1.5rem;
    margin: 1rem 0;
}}
.nzism-point h3 {{
    color: var(--primary);
    margin-top: 0;
}}
.nzism-point p {{ color: var(--text-muted); }}
footer {{
    background: var(--dark);
    border-top: 1px solid var(--border);
    padding: 2rem 0;
    margin-top: 3rem;
    text-align: center;
    color: var(--text-muted);
    font-size: 0.85rem;
}}
footer a {{ color: var(--primary); text-decoration: none; }}
a {{ color: var(--primary); }}
</style>
</head>
<body>

<header>
<div class="container">
    <h1>NZ Critical Infrastructure PQC Readiness Assessment</h1>
    <div class="subtitle">Scan date: {date} | Mapped to NZISM Section 2.4</div>
    <span class="badge">SECTOR-LEVEL AGGREGATES ONLY</span>
</div>
</header>

<div class="container">

<section>
<h2>1. Executive Summary</h2>
<p>This report presents the results of scanning <strong>{total_scanned}</strong> public-facing TLS endpoints
across <strong>{total_targets}</strong> NZ critical infrastructure entities in all seven proposed essential
service sectors from the DPMC critical infrastructure consultation.</p>

<div class="stats-grid">
    <div class="stat-card">
        <div class="number">{pqc_pct:.1}%</div>
        <div class="label">PQC Key Exchange Supported</div>
    </div>
    <div class="stat-card">
        <div class="number">{tls13_pct:.1}%</div>
        <div class="label">TLS 1.3 Supported</div>
    </div>
    <div class="stat-card">
        <div class="number">{cdn_pct:.1}%</div>
        <div class="label">Behind CDN</div>
    </div>
    <div class="stat-card">
        <div class="number">{total_scanned}</div>
        <div class="label">Endpoints Scanned</div>
    </div>
</div>

<div class="caveat">
<strong>Important limitation:</strong> Many NZ critical infrastructure endpoints sit behind CDNs
(Cloudflare, AWS CloudFront, Akamai). When a CDN has PQC enabled, the scan result reflects
the CDN&rsquo;s configuration, not necessarily the organisation&rsquo;s own cryptographic posture.
{cdn_pct:.1}% of scanned endpoints were detected behind a CDN. This report is transparent
about this limitation.
</div>
</section>

<section>
<h2>2. Methodology</h2>
<p>Endpoints were scanned using <code>nzism-pqc-audit</code>, an open-source Rust tool that
attempts TLS 1.3 connections and checks for PQC hybrid key exchange support
(X25519MLKEM768, SecP256r1MLKEM768). CDN detection is performed via HTTP header analysis
(CF-Ray, X-Amz-Cf-Id, X-Akamai-Transformed, X-Served-By).</p>
<p>Targets were identified from the DPMC &ldquo;Strengthening the Cyber Security of
Critical Infrastructure&rdquo; discussion document (February 2026), covering the seven
proposed essential service sectors.</p>
<p><strong>What this scan measures:</strong> Whether a public-facing TLS endpoint supports
post-quantum hybrid key exchange algorithms as part of the TLS handshake.</p>
<p><strong>What this scan does NOT measure:</strong> Internal cryptographic posture, data-at-rest
encryption, certificate signing algorithms, key management practices, or compliance with
any specific framework.</p>
</section>

<section>
<h2>3. NZISM Section 2.4 Mapping</h2>
<p>NZISM Section 2.4 &ldquo;Preparation for Post-Quantum Cryptography&rdquo; sets out five
recommendations. This scan addresses Point 3 directly and provides context for Point 4.</p>

<div class="nzism-point">
<h3>Point 1: Monitor PQC Developments</h3>
<p>{point1}</p>
</div>

<div class="nzism-point">
<h3>Point 2: Inventory Sensitive Data</h3>
<p>{point2}</p>
</div>

<div class="nzism-point">
<h3>Point 3: Inventory Public-Key Systems</h3>
<p>{point3}</p>
</div>

<div class="nzism-point">
<h3>Point 4: Prioritise for Migration</h3>
<p>{point4}</p>
</div>

<div class="nzism-point">
<h3>Point 5: Develop Migration Plan</h3>
<p>{point5}</p>
</div>
</section>

<section>
<h2>4. Sector-by-Sector Results</h2>
<table>
<thead>
<tr>
    <th>Sector</th>
    <th>Entities</th>
    <th>Scanned</th>
    <th>PQC Support</th>
    <th>TLS 1.3</th>
    <th>Behind CDN</th>
    <th>Errors</th>
</tr>
</thead>
<tbody>
{sector_rows}
</tbody>
</table>
</section>

<section>
<h2>5. Aggregate Statistics</h2>
<p>Of {total_scanned} successfully scanned endpoints:</p>
<ul>
<li><strong>{pqc_count}</strong> ({pqc_pct:.1}%) support PQC hybrid key exchange</li>
<li><strong>{tls13_count}</strong> ({tls13_pct:.1}%) support TLS 1.3</li>
<li><strong>{cdn_count}</strong> ({cdn_pct:.1}%) are behind a CDN (results may reflect CDN configuration)</li>
<li><strong>{error_count}</strong> endpoints could not be scanned (connection timeout, DNS failure, etc.)</li>
</ul>
</section>

<section>
<h2>6. Global Comparison</h2>
<p>For context, F5 Labs&rsquo; &ldquo;The State of PQC on the Web&rdquo; found that only
<strong>8.6%</strong> of the top 1 million websites globally support hybrid PQC, and only
<strong>3%</strong> of banking sites. NZ critical infrastructure&rsquo;s {pqc_pct:.1}%
PQC adoption rate should be viewed in this global context.</p>
<p>It should also be noted that much of the observed PQC support globally is driven by
CDN providers (particularly Cloudflare) rather than by the organisations themselves.</p>
</section>

<section>
<h2>7. Recommendations</h2>
<ol>
<li><strong>Inventory cryptographic systems</strong> as required by NZISM Section 2.4 Point 3.
This scan provides an external baseline; internal assessments will reveal the full picture.</li>
<li><strong>Develop a PQC migration plan</strong> aligned to NZISM Section 2.4 Point 5. Reference
Australia&rsquo;s ASD guidance (refined plan by end 2026) as a benchmark.</li>
<li><strong>Enable TLS 1.3</strong> across all public-facing endpoints as a prerequisite for PQC
key exchange support.</li>
<li><strong>Evaluate CDN PQC configuration</strong> &mdash; if your endpoint is behind a CDN,
verify whether PQC is enabled at the CDN level and plan for origin-to-CDN PQC migration.</li>
<li><strong>Engage with the DPMC consultation</strong> (submissions close 19 April 2026) to
advocate for explicit PQC readiness requirements in the proposed critical infrastructure
risk management framework.</li>
</ol>
</section>

<section>
<h2>8. About</h2>
<p>This report was generated by <a href="https://github.com/spinsphere/nzism-pqc-audit">nzism-pqc-audit</a>,
an open-source PQC readiness scanner developed by
<a href="https://kaysec.spinsphere.xyz">Kaysec</a>, the post-quantum security division of
<a href="https://spinsphere.xyz">Spinsphere</a>.</p>
<p>For a detailed assessment of your organisation&rsquo;s PQC readiness, contact
<a href="https://spinsphere.xyz/contact/">Kaysec</a>.</p>
</section>

</div>

<footer>
<div class="container">
<p>Generated by <a href="https://github.com/spinsphere/nzism-pqc-audit">nzism-pqc-audit</a>
v{version} on {date}</p>
<p><a href="https://kaysec.spinsphere.xyz">Kaysec</a> by
<a href="https://spinsphere.xyz">Spinsphere</a> &mdash; Post-Quantum Security</p>
</div>
</footer>

</body>
</html>"##,
        date = report.scan_date,
        total_targets = report.total_targets,
        total_scanned = report.total_scanned,
        pqc_pct = report.pqc_percentage,
        tls13_pct = report.tls13_percentage,
        cdn_pct = report.cdn_percentage,
        pqc_count = report.total_pqc_supported,
        tls13_count = report.total_tls13,
        cdn_count = report.total_cdn_detected,
        error_count = report.total_errors,
        sector_rows = sector_rows,
        point1 = report.nzism_mapping.point1_monitor,
        point2 = report.nzism_mapping.point2_inventory_data,
        point3 = report.nzism_mapping.point3_inventory_systems,
        point4 = report.nzism_mapping.point4_prioritise,
        point5 = report.nzism_mapping.point5_migration_plan,
        version = env!("CARGO_PKG_VERSION"),
    )
}
