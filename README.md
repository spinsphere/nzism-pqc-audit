# nzism-pqc-audit

**A TLS post-quantum readiness scanner, with New Zealand critical infrastructure as its first target list.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.86%2B-orange.svg)](https://www.rust-lang.org/)
[![Status: v0.1.0](https://img.shields.io/badge/status-v0.1.0-green.svg)](https://github.com/spinsphere/nzism-pqc-audit/releases)

---

## What this is (the 60-second version)

This is a small command-line tool, written in Rust, that answers one question for each domain you point it at:

> When a modern browser starts a secure connection to this server today, will the server agree to use **post-quantum cryptography** for the key exchange?

It runs the same kind of TLS handshake a web browser would, offers the server the NIST-standardised post-quantum hybrid key exchange `X25519MLKEM768`, and records what the server actually agrees to use. It also detects whether the server is behind a content delivery network (CDN) — Cloudflare, Akamai, AWS CloudFront, Fastly, Imperva — so you can tell whether any post-quantum protection you see is coming from the organisation itself or from a CDN vendor that turned it on transparently.

The output is a JSON file with one row per target, a readable HTML report, and (optionally) a sector-level summary.

Everything in this repository is open source under the MIT licence. If you want to run it against your own target list — whether that's one domain or a thousand — you only need this repository, a Rust toolchain, and a plain-text file of `host:port` lines or a CSV in the same shape as `targets/nz/critical-infrastructure.csv`.

---

## If you're landing here cold, read this first

**"Post-quantum" cryptography (PQC)** is the name for a new generation of mathematical building blocks designed to stay secure even once a sufficiently powerful quantum computer exists. Most of the encryption that protects the public internet today — RSA, Elliptic Curve Diffie-Hellman, ECDSA — can in principle be broken by a future quantum computer running Peter Shor's 1994 algorithm. That quantum computer does not exist yet. It also does not need to exist *today* for this to be a problem you care about, because of an attack pattern called **harvest now, decrypt later** (HNDL): an adversary with enough storage can record encrypted traffic today, sit on it for a decade, and decrypt it later once a capable quantum machine is built. Anything with a long confidentiality tail — health records, legal matters, trade secrets, state information, long-retention financial records — is already exposed under that threat model.

In August 2024, the US National Institute of Standards and Technology (NIST) finalised the first three post-quantum standards:

- **FIPS 203** — ML-KEM (formerly CRYSTALS-Kyber), for key encapsulation / key exchange.
- **FIPS 204** — ML-DSA (formerly CRYSTALS-Dilithium), for digital signatures.
- **FIPS 205** — SLH-DSA (formerly SPHINCS+), a stateless hash-based signature backup.

For web TLS specifically, browsers, TLS libraries and CDNs have converged on a hybrid key exchange called `X25519MLKEM768`, which runs classical X25519 and ML-KEM-768 side by side. The connection stays safe as long as *either* primitive holds. Chrome, Firefox, Safari, OpenSSL, Go and recent Apple operating systems enable it by default. Cloudflare, Akamai, AWS and Microsoft have rolled it out on the server side.

**NZISM** is the [New Zealand Information Security Manual](https://nzism.gcsb.govt.nz/), published by the Government Communications Security Bureau (GCSB). Section 2.4 is NZISM's post-quantum cryptography section — it requires agencies to monitor PQC developments, inventory their cryptographic systems, and develop migration plans. This tool exists partly to make that easier to measure and partly to give New Zealand organisations an external, public baseline for how their TLS stack compares.

The name of this repository — `nzism-pqc-audit` — reflects that NZISM framing. The tool itself is generic: you can point it at any domain list, NZ or not.

---

## April 2026 findings (summary)

On 14 April 2026 we ran this scanner against 118 New Zealand critical infrastructure entities — covering all seven essential service sectors defined by New Zealand's Department of the Prime Minister and Cabinet (DPMC) critical infrastructure consultation.

- **116 of 118 scans succeeded.** Two handshake errors.
- **61 (52.6%) of successful scans** negotiated the post-quantum hybrid `X25519MLKEM768`.
- **48 of those 61 results came from CDN edges** (Cloudflare, Imperva, AWS CloudFront). Those CDN providers enabled post-quantum TLS by default between 2024 and early 2026, and the NZ entities behind them received that protection transparently.
- **Only 13 entities (11% of the scanned population)** run post-quantum TLS on infrastructure they operate themselves. They're named in [`results/2026-04-14/origin-honour-roll.md`](results/2026-04-14/origin-honour-roll.md).
- **The Drinking Water & Wastewater sector had zero self-hosted post-quantum endpoints.**
- **Four entities sit behind Fastly, which had not rolled out default post-quantum TLS** at the time of the scan.

The sector-aggregated results are in [`results/2026-04-14/summary.json`](results/2026-04-14/summary.json) and [`results/2026-04-14/summary.md`](results/2026-04-14/summary.md). The full write-up — including the global post-quantum trajectory (Gidney 2025, Google's 2029 migration deadline, Google Quantum AI's March 2026 ECC paper, NSA CNSA 2.0, NIST IR 8547), the CDN analysis, the DPMC consultation gap, and what organisations should actually do about it — is on the [Kaysec blog at kaysec.spinsphere.xyz](https://kaysec.spinsphere.xyz).

### DPMC consultation submission

On 19 April 2026 we submitted these findings to the Department of the Prime Minister and Cabinet as a formal response to the [Discussion Document on enhancing the cyber security of New Zealand's critical infrastructure system](https://consultation.dpmc.govt.nz/cyber/cyber-security-of-critical-infrastructure/). The full submission PDF is available at [`submissions/Enhancing NZ's critical infrastructure cybersecurity with a PQC perspective - by Spinsphere.pdf`](submissions/Enhancing%20NZ%27s%20critical%20infrastructure%20cybersecurity%20with%20a%20PQC%20perspective%20-%20by%20Spinsphere.pdf).

We publish **sector-level aggregates only** as public results. We do not publish a per-entity "wall of shame"; we only name the 13 positive origin examples. See [Responsible disclosure](#responsible-disclosure) below.

---

## Install

### Prerequisites

- A Rust toolchain, version 1.86 or newer. The easiest way to install Rust is [rustup](https://rustup.rs/).
- An internet connection (the scanner opens TLS handshakes to the targets you give it).

### Build from source

```bash
git clone https://github.com/spinsphere/nzism-pqc-audit.git
cd nzism-pqc-audit
cargo build --release
```

The binary will be at `target/release/nzism-pqc-audit`.

### Or install with cargo

```bash
cargo install --git https://github.com/spinsphere/nzism-pqc-audit.git
```

This puts `nzism-pqc-audit` on your `PATH` (usually `~/.cargo/bin/`).

---

## Usage

### Scan the included NZ critical infrastructure target list

```bash
nzism-pqc-audit scan \
  --input targets/nz/critical-infrastructure.csv \
  --format csv \
  --output-json results.json \
  --output-html report.html \
  --timeout 15 \
  --concurrency 10
```

This takes roughly 1–2 minutes against the 118 entities shipped in this repo. `results.json` contains the full per-target data, `report.html` is a self-contained HTML summary you can open in a browser.

### Scan your own list (plain text)

Create a file with one `host:port` pair per line:

```
example.com:443
api.example.com:443
www.example.org:443
```

Then:

```bash
nzism-pqc-audit scan \
  --input targets.txt \
  --format text \
  --output-json results.json
```

### Re-render an HTML report from existing JSON

```bash
nzism-pqc-audit report \
  --input results.json \
  --output report.html
```

### Options

| Flag | Default | Meaning |
|---|---|---|
| `--timeout <seconds>` | 10 | Connection / handshake timeout per target |
| `--concurrency <n>` | 20 | How many scans to run in parallel |
| `--format <csv\|text>` | *(required)* | Input file format |
| `--output-json <path>` | *(required)* | Where to write per-target JSON |
| `--output-html <path>` | — | Optional HTML report destination |

---

## What it measures

For each TLS endpoint:

- **TLS version negotiated.** TLS 1.3 is required for `X25519MLKEM768`.
- **Cipher suite negotiated.**
- **Key exchange group negotiated.** This is the field that determines PQC status — for example `X25519MLKEM768` (post-quantum hybrid), `X25519` (classical), `secp256r1` (classical, weaker), etc.
- **PQC support flag.** `true` if the negotiated key exchange group is a known post-quantum hybrid (currently `X25519MLKEM768` and `SecP256r1MLKEM768`).
- **CDN detection.** By inspecting HTTP response headers (`CF-*`, `X-Amz-Cf-*`, `X-Cache`, `Server`, `X-Akamai-*`, `X-Iinfo`, `Via`, etc.), the scanner records which CDN — if any — is fronting the endpoint. If the endpoint is CDN-fronted, the report notes that any PQC support observed may reflect the CDN's configuration rather than the origin server's.

## What it does NOT measure

This scanner gives you **one signal**: whether a publicly-reachable TLS front door will currently negotiate a hybrid post-quantum key exchange with a modern client. That signal is necessary but not sufficient for post-quantum readiness. It does **not** measure:

- Internal cryptography (database encryption, service mesh, VPN, SSH, Kerberos, AD)
- Certificate signature algorithms — ML-DSA adoption on your PKI is a separate, later migration
- Data at rest encryption
- Key management quality or HSM posture
- Cryptographic primitives in application code, signing pipelines, backups, email, or document workflows
- The PQC posture of the back-end connection from a CDN to its origin

A "PQC-positive" handshake is a starting point, not a finish line. A cryptographic inventory is the real precondition for a credible migration plan.

---

## The target list (`targets/nz/critical-infrastructure.csv`)

The repository ships with a list of **118 New Zealand critical infrastructure entities**, selected to approximate the draft thresholds in [DPMC Supplementary Document 2 — Defining critical infrastructure](https://www.dpmc.govt.nz/sites/default/files/2026-02/nz-cyber-security-discussion-supp-doc-2-feb-2026.pdf) (February 2026).

Coverage across the seven DPMC essential service sectors:

| Sector | Entities |
|---|---|
| Communications & Data | 18 |
| Defence | 6 |
| Energy | 30 |
| Finance | 14 |
| Health | 16 |
| Transport | 25 |
| Drinking Water & Wastewater | 9 |

See [`targets/nz/README.md`](targets/nz/README.md) for the full methodology, selection criteria, and caveats.

You do not have to use this list. Any text file or CSV in the same shape will work.

---

## Responsible disclosure

This repository publishes **sector-level aggregates only** for its own 14 April 2026 scan. We do not publish a per-entity "no PQC" list, because:

1. A single TLS handshake is a narrow signal and could be misread as a total cryptographic posture verdict.
2. Naming an individual organisation as "not ready" shifts attention away from the system-level question, which is where the cyber-security consultation is actually happening.
3. The positive cases — organisations that have clearly and deliberately enabled origin-side post-quantum TLS — deserve visible credit, and our [origin honour roll](results/2026-04-14/origin-honour-roll.md) names those.

If you scan your own infrastructure or your own customers' infrastructure, the per-target JSON output will contain everything you need. Please be thoughtful about what you publish.

**Security reports about this tool itself** (not about targets you scan with it) should follow [SECURITY.md](SECURITY.md).

---

## Contributing

Pull requests are welcome — bug fixes, new CDN fingerprints, new key exchange groups, new target lists for other jurisdictions, report improvements.

Before sending a large change, please open an issue first to discuss the approach. See [CONTRIBUTING.md](CONTRIBUTING.md) for the day-to-day mechanics.

---

## About

`nzism-pqc-audit` is developed by **[Kaysec](https://kaysec.spinsphere.xyz)**, the post-quantum security practice inside **[Spinsphere](https://spinsphere.xyz)**, a New Zealand-based quantum technology company.

Kaysec helps New Zealand organisations with cryptographic inventory, harvest-now-decrypt-later (HNDL) risk assessment, post-quantum migration planning, and NZISM Section 2.4 alignment. Our typical engagement is a scoped inventory and gap assessment for small-to-medium NZ businesses that handle long-lived secrets (legal, health, engineering IP, financial records, anything subject to statutory retention) and need help thinking through what a credible migration plan looks like.

If anything in this report matters to your organisation, get in touch: `simon [at] spinsphere.xyz`.

---

## References

Key primary sources (the full reference list is in the [Kaysec blog report](https://kaysec.spinsphere.xyz)):

- Shor, P. W. (1994). *Algorithms for quantum computation: discrete logarithms and factoring.*
- NIST. (August 2024). *FIPS 203 (ML-KEM), FIPS 204 (ML-DSA), FIPS 205 (SLH-DSA).* [nist.gov/pqc](https://www.nist.gov/pqc)
- Gidney, C. (May 2025). *How to factor 2048-bit RSA integers with less than a million noisy qubits.* [arXiv:2505.15917](https://arxiv.org/abs/2505.15917)
- Google (Adkins & Schmieg). (25 March 2026). *Google's timeline for post-quantum cryptography migration.* [blog.google](https://blog.google/innovation-and-ai/technology/safety-security/cryptography-migration-timeline/)
- Cloudflare. (25 March 2026). *Cloudflare targets 2029 for full post-quantum security.* [blog.cloudflare.com](https://blog.cloudflare.com/post-quantum-roadmap/)
- Google Research. (March 2026). *Safeguarding cryptocurrency by disclosing quantum vulnerabilities responsibly.* [research.google](https://research.google/blog/safeguarding-cryptocurrency-by-disclosing-quantum-vulnerabilities-responsibly/)
- NIST. (2024). *NIST IR 8547 (Draft): Transition to Post-Quantum Cryptography Standards.* [csrc.nist.gov](https://csrc.nist.gov/pubs/ir/8547/ipd)
- NSA. (May 2025). *Commercial National Security Algorithm Suite 2.0.* [media.defense.gov](https://media.defense.gov/2025/May/30/2003728741/-1/-1/0/CSA_CNSA_2.0_ALGORITHMS.PDF)
- NZISM Section 2.4. [nzism.gcsb.govt.nz](https://nzism.gcsb.govt.nz/)
- DPMC. (February 2026). *Enhancing the cyber security of New Zealand's critical infrastructure system.* [dpmc.govt.nz](https://www.dpmc.govt.nz/publications/discussion-document-enhancing-cyber-security-new-zealands-critical-infrastructure-system)

---

## Licence

MIT — see [LICENSE](LICENSE).
