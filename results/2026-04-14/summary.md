# NZ Critical Infrastructure PQC Readiness — 14 April 2026

**Scan date:** 14 April 2026
**Tool version:** `nzism-pqc-audit` v0.1.0
**Target list:** [`targets/nz/critical-infrastructure.csv`](../../targets/nz/critical-infrastructure.csv) — 118 entities across all seven DPMC essential service sectors
**Full write-up:** [Kaysec blog, kaysec.spinsphere.xyz](https://kaysec.spinsphere.xyz)

This file is a **sector-aggregated, non-attributable** summary of the scan. It is deliberately limited to aggregates — no individual entity is named as either "has PQC" or "does not have PQC" here. The 13 entities that deliberately run origin-side post-quantum TLS are named in [`origin-honour-roll.md`](origin-honour-roll.md) as positive examples only.

## Headline

- **Targets scanned:** 118
- **Successful scans:** 116
- **Handshake errors:** 2
- **TLS 1.3 support:** 107 (92.2%)
- **PQC hybrid (`X25519MLKEM768`) negotiated:** **61 (52.6%)**
- **Behind a CDN:** 54 (46.6%)

Split by infrastructure layer:

| Class | Endpoints | PQC negotiated | Rate |
|---|---|---|---|
| CDN-fronted | 54 | 48 | **88.9%** |
| Self-hosted (origin) | 62 | 13 | **21.0%** |

52.6% of endpoints negotiate post-quantum TLS, but **48 of those 61 results are delivered transparently by CDN vendors** who enabled PQC by default between 2024 and early 2026. Only **13 entities (11%)** run post-quantum TLS on infrastructure they operate themselves.

## Sector breakdown

| Sector | Scanned | Total PQC | PQC via CDN | PQC on origin | Errors |
|---|---:|---:|---:|---:|---:|
| Communications & Data | 18 | 9 (50%) | 8 | 1 | 0 |
| Defence | 6 | 5 (83%) | 2 | 3 | 0 |
| Drinking Water & Wastewater | 9 | 3 (33%) | 3 | 0 | 0 |
| Energy | 30 | 14 (47%) | 9 | 5 | 1 |
| Finance | 14 | 7 (50%) | 3 | 4 | 0 |
| Health | 16 | 12 (75%) | 12 | 0 | 1 |
| Transport | 25 | 11 (44%) | 11 | 0 | 0 |
| **Total** | **118** | **61 (51.7%)** | **48** | **13** | **2** |

*Percentages in the sector column are over the full sector total (including any errors). The 52.6% global headline is over successful scans only (116).*

Notable sector observations:

- **Health 75%** — every PQC-positive health endpoint is behind a single CDN provider (Imperva/Incapsula), not the hospital origin servers themselves. Edge-only protection.
- **Defence 83%** — the highest sector rate, and the only sector where origin-side PQC is meaningful relative to size. Three of the five origin heroes are NZ intelligence / digital-identity agencies.
- **Drinking Water & Wastewater 33%** — zero origin-side PQC. Water utilities are the weakest sector on self-hosted cryptographic posture.
- **Transport 44%** — every PQC-positive transport endpoint is CDN-delivered. Zero origin-side.
- **Finance 50%** — four of the seven PQC-positive endpoints are origin-side, including one of the "big-4" systemically important banks. Three others (SBS Bank, The Co-operative Bank, Vero Insurance) also run origin-side PQC.

## CDN provider PQC coverage

54 endpoints sit behind a CDN. 48 of them get post-quantum TLS transparently from that CDN.

| CDN provider | Entities fronted | PQC negotiated | Rate |
|---|---:|---:|---:|
| Cloudflare | 29 | 28 | 97% |
| Imperva (incl. Incapsula) | 17 | 17 | 100% |
| AWS CloudFront | 4 | 3 | 75% |
| Fastly | 4 | 0 | **0%** |

Fastly is the visible gap. Four NZ critical infrastructure entities sit behind Fastly, and none of them negotiated post-quantum TLS at the edge on scan day. Fastly is publicly committed to PQC but had not yet enabled it by default at the time of the scan.

## Key exchange group distribution

What groups were actually negotiated across the 116 successful handshakes:

| Key exchange group | Count | Classification |
|---|---:|---|
| `X25519MLKEM768` | 61 | Post-quantum hybrid (FIPS 203) |
| `X25519` | 43 | Classical |
| `secp256r1` | 9 | Classical, TLS 1.2 |
| `secp384r1` | 3 | Classical, TLS 1.2 |

## TLS version distribution

| TLS version | Count |
|---|---:|
| TLS 1.3 | 107 |
| TLS 1.2 | 9 |

## Methodology

The scanner opens a TLS 1.3 handshake against each target's port 443, offering a set of key share groups that includes `X25519MLKEM768`, and records whatever the server negotiates. CDN fingerprinting is done separately via HTTP response headers.

A "PQC-positive" result means the server explicitly chose to negotiate `X25519MLKEM768` when given the choice. A "PQC-negative" result means it chose something classical — most often `X25519`.

**This measures exactly one signal: whether a publicly-reachable TLS front door will currently negotiate a hybrid post-quantum key exchange with a modern client.** It does not measure internal cryptography, certificate signature algorithms, data at rest, SSH, key management quality, or application-layer cryptography. A PQC-positive handshake is necessary but not sufficient for post-quantum readiness.

## Raw data (structured)

The machine-readable version of the same numbers is at [`summary.json`](summary.json).

## What to do with this

If you run one of the 118 entities and you want to see your own result: rerun the tool yourself, it's open source.

If you want to enable post-quantum TLS on your own infrastructure, the shortest path is usually a TLS library update (if you self-host) or a conversation with your CDN account manager (if you're fronted by one) — depending on the CDN, you may already have it on by default.

If you want a more comprehensive cryptographic inventory, HNDL risk assessment, or PQC migration plan, [Kaysec](https://kaysec.spinsphere.xyz) does that work for New Zealand organisations.
