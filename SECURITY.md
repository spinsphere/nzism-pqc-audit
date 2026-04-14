# Security policy

## Reporting a vulnerability in this tool

If you believe you have found a security vulnerability in `nzism-pqc-audit` itself — the scanner code, build pipeline, or released artifacts — please report it privately. Do not open a public GitHub issue for security-sensitive reports.

**Preferred channel:** email `simon [at] spinsphere.xyz` with the subject line `[nzism-pqc-audit security]`. Please include:

- A description of the issue and its potential impact.
- The version / commit of the tool you tested against (output of `nzism-pqc-audit --version` and `git rev-parse HEAD` from the source tree if you built from source).
- Steps to reproduce.
- Any proof-of-concept code, with the caveat that it should not be run against third-party infrastructure you do not have permission to scan.

We will acknowledge receipt within a reasonable period and work with you on disclosure timing. We are a small team; please be patient.

## What is in scope

- Memory safety, crash, or logic bugs in the scanner (`src/`).
- Bugs in the TLS client configuration that cause the scanner to silently under-report or over-report post-quantum support, false-negative CDN detection, or misclassification of results.
- Dependency vulnerabilities with a plausible exploitation path through this tool.
- Bugs in the report generation code that could enable XSS or injection into the generated HTML report.

## What is out of scope

- Behaviour of the target servers this tool is pointed at. If you find something concerning about a specific organisation's TLS posture, contact that organisation — this tool only measures and reports.
- Vulnerabilities in the underlying Rust toolchain or in `rustls` / `aws-lc-rs` / other upstream libraries — those should be reported to their respective maintainers.
- Issues in the NZ critical infrastructure entities we happen to include in the sample target list. Those are not our systems.

## Scanning responsibly

Just because you *can* run this tool against a list of domains does not mean you *should*. A TLS handshake is relatively lightweight, but high-concurrency scans against third-party infrastructure can still look like reconnaissance to a defender, and publishing per-entity results without context is unhelpful. We ask that users:

- Scan only infrastructure you own, operate, or have clear authorisation to assess.
- Respect the target list's responsible disclosure stance: **sector-level aggregates** when reporting publicly.
- Credit positive examples where appropriate; do not publish "walls of shame" based on a single TLS handshake.

Thank you.
