# NZ Critical Infrastructure PQC Readiness — 2 June 2026

**Scan date:** 2 June 2026
**Tool version:** `nzism-pqc-audit` v0.1.0
**Target list:** [`targets/nz/critical-infrastructure.csv`](../../targets/nz/critical-infrastructure.csv) — 118 entities across all seven DPMC essential service sectors
**Previous scan:** [14 April 2026](../2026-04-14/summary.md)
**Full write-up:** [Kaysec blog, kaysec.spinsphere.xyz](https://kaysec.spinsphere.xyz)

This file is a **sector-aggregated, non-attributable** summary of the scan. It is deliberately limited to aggregates — no individual entity is named as either "has PQC" or "does not have PQC" here. Entities that run origin-side post-quantum TLS are named in [`origin-honour-roll.md`](origin-honour-roll.md) as positive examples only.

## Headline

- **Targets scanned:** 118
- **Successful scans:** 115
- **Handshake errors:** 3
- **TLS 1.3 support:** 106 (92.2%)
- **PQC hybrid (`X25519MLKEM768`) negotiated:** **60 (52.2%)**
- **Behind a CDN:** 41 (35.7%)

Split by infrastructure layer:

| Class | Endpoints | PQC negotiated | Rate |
|---|---|---|---|
| CDN-fronted | 41 | 35 | **85.4%** |
| Self-hosted (origin) | 74 | 25 | **33.8%** |

52.2% of endpoints negotiate post-quantum TLS. **35 of those 60 results are delivered by CDN vendors**. **25 entities (21.7% of successful scans)** run post-quantum TLS on infrastructure they operate themselves — nearly double the 13 (11.2%) recorded in April 2026.

## Change vs April 2026

| Metric | April 2026 | June 2026 | Change |
|---|---:|---:|---:|
| Successful scans | 116 | 115 | −1 |
| Handshake errors | 2 | 3 | +1 |
| PQC negotiated | 61 (52.6%) | 60 (52.2%) | −1 |
| Behind CDN | 54 (46.6%) | 41 (35.7%) | −13 |
| PQC via CDN | 48 | 35 | −13 |
| PQC via origin | 13 (11.2%) | 25 (21.7%) | **+12** |
| No PQC behind CDN | 6 | 6 | 0 |
| No PQC at origin | 49 | 49 | 0 |

The headline PQC rate is essentially flat, but the composition changed substantially. The primary driver is that **Imperva CDN is no longer detected in front of 13 entities** that were Imperva-fronted in April (12 health-sector entities and KiwiRail). For the 11 health entities that retained PQC, the underlying origin servers were already running `X25519MLKEM768` — the CDN layer removal revealed self-hosted PQC capability that existed but was not directly visible in April. KiwiRail's origin does not have PQC, so it lost the protection its CDN was providing.

## Sector breakdown

| Sector | Scanned | Total PQC | PQC via CDN | PQC on origin | Errors |
|---|---:|---:|---:|---:|---:|
| Communications & Data | 18 | 10 (55.6%) | 8 | 2 | 0 |
| Defence | 6 | 5 (83.3%) | 2 | 3 | 0 |
| Drinking Water & Wastewater | 9 | 3 (33.3%) | 3 | 0 | 0 |
| Energy | 30 | 14 (46.7%) | 9 | 5 | 1 |
| Finance | 14 | 7 (50.0%) | 3 | 4 | 0 |
| Health | 16 | 11 (68.8%) | 0 | 11 | 2 |
| Transport | 25 | 10 (40.0%) | 10 | 0 | 0 |
| **Total** | **118** | **60 (51.7%)** | **35** | **25** | **3** |

*Percentages in the sector column are over the full sector total including errors. The 52.2% global headline is over successful scans only (115).*

Notable sector observations vs April:

- **Health: 0 → 11 origin-side PQC.** All 12 health endpoints that showed PQC in April were Imperva CDN-delivered. 11 of those entities now show as non-CDN origin, with the same PQC negotiation. One (Taranaki Base Hospital) is now returning an SNI error. Three health entities (Auckland City, Middlemore, North Shore hospitals) have no CDN and no origin PQC.
- **Transport: 11 → 10 CDN PQC.** KiwiRail was Imperva CDN-fronted with PQC in April. Imperva is no longer detected; the origin negotiates classical X25519.
- **Communications & Data: +1 PQC.** Southern Cross Cables, previously classical at origin, now negotiates `X25519MLKEM768` directly — the only entity in this scan to have genuinely enabled PQC on its origin infrastructure between April and June.
- **Defence, Energy, Finance, Drinking Water:** sector-level numbers unchanged from April.

## CDN provider PQC coverage

| CDN provider | Entities fronted | PQC negotiated | Rate | vs April |
|---|---:|---:|---:|---:|
| Cloudflare | 29 | 28 | 97% | unchanged |
| Imperva (incl. Incapsula) | 4 | 4 | 100% | was 17 |
| AWS CloudFront | 4 | 3 | 75% | unchanged |
| Fastly | 4 | 0 | **0%** | unchanged |

Fastly remains the gap. Four NZ critical infrastructure entities sit behind Fastly and none negotiated post-quantum TLS. Fastly has publicly committed to PQC enablement but had not deployed it by default as of this scan.

## Key exchange group distribution

| Key exchange group | Count | Classification |
|---|---:|---|
| `X25519MLKEM768` | 60 | Post-quantum hybrid (FIPS 203) |
| `X25519` | 43 | Classical |
| `secp256r1` | 9 | Classical, TLS 1.2 |
| `secp384r1` | 3 | Classical, TLS 1.2 |

## TLS version distribution

| TLS version | Count |
|---|---:|
| TLS 1.3 | 106 |
| TLS 1.2 | 9 |

## Scan errors

| Entity | Sector | Error |
|---|---|---|
| Wellington Electricity | Energy | HandshakeFailure (persistent — same as April) |
| Dunedin Hospital | Health | Timeout (persistent — same as April) |
| Taranaki Base Hospital | Health | UnrecognisedName (new — SNI mismatch) |

## Methodology

The scanner opens a TLS 1.3 handshake against each target's port 443, offering key share groups including `X25519MLKEM768`, and records whatever the server negotiates. CDN fingerprinting is done via HTTP response headers. Full methodology at [`targets/nz/README.md`](../../targets/nz/README.md).

**This measures exactly one signal: whether a publicly-reachable TLS front door will currently negotiate a hybrid post-quantum key exchange with a modern client.** It does not measure internal cryptography, certificate signature algorithms, data at rest, SSH, key management quality, or application-layer cryptography.

## Raw data

Machine-readable: [`summary.json`](summary.json)
Origin honour roll: [`origin-honour-roll.md`](origin-honour-roll.md)
