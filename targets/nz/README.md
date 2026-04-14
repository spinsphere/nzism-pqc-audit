# NZ Critical Infrastructure - PQC Readiness Scan Targets

## Overview

This directory contains the target list for scanning New Zealand critical infrastructure
entities for post-quantum cryptography (PQC) readiness. The scan checks TLS configurations
on public-facing domains for quantum-vulnerable cryptographic algorithms and PQC migration
status.

## Source Framework

Entities are categorised according to the 7 sectors defined in the DPMC (Department of the
Prime Minister and Cabinet) critical infrastructure discussion documents:

1. **Communications & Data** - Telcos, ISPs, broadcast, submarine cables, data centres, domain registry
2. **Defence** - NZDF, Ministry of Defence
3. **Energy** - Electricity generation/transmission/distribution, gas pipelines, fuel infrastructure
4. **Finance** - Systemically important banks, central bank, securities exchange, payment systems
5. **Health** - Health NZ / Te Whatu Ora, major hospitals with ICU capabilities
6. **Transport** - National airline, rail, road authority, airports, seaports
7. **Drinking Water & Wastewater** - Major water utilities serving 25,000+ connections

## Key References

- [DPMC Cyber Security of Critical Infrastructure Discussion Document (Feb 2026)](https://www.dpmc.govt.nz/publications/discussion-document-enhancing-cyber-security-new-zealands-critical-infrastructure-system)
- [DPMC Supplementary Document 2 - Defining Critical Infrastructure (Feb 2026)](https://www.dpmc.govt.nz/sites/default/files/2026-02/nz-cyber-security-discussion-supp-doc-2-feb-2026.pdf)
- [NZISM Section 2.4 - Post-Quantum Cryptography](https://nzism.gcsb.govt.nz/)

## Methodology

### Entity Selection Criteria

Entities were selected based on the proposed thresholds in the DPMC discussion documents:

| Sector | Threshold |
|--------|-----------|
| Communications & Data | All MNOs, national fibre provider, submarine cable operators, broadcast lifeline utilities, .nz registry, major data centre operators |
| Defence | NZDF and Ministry of Defence |
| Energy - Generation | Generators with >25 MW installed capacity |
| Energy - Transmission | Sole national grid operator (Transpower) |
| Energy - Distribution | EDBs with >25,000 ICPs (installation control points) |
| Energy - Gas | National gas transmission pipeline operators |
| Energy - Fuel | Major fuel distributors with bulk storage terminals; fuel import terminal operators |
| Finance | D-SIBs (domestically systemically important banks) as designated by RBNZ; central bank; securities exchange; core payment system operators |
| Health | Health NZ (national entity) plus individual hospitals with ICU/tertiary capabilities |
| Transport | Flag carrier airline; national rail operator; state highway authority; international airports; ports handling significant cargo/container volumes |
| Water | Water utilities serving >25,000 connections |

### Domain Verification

Each domain was verified through:

1. Web search confirming the entity's primary public-facing website
2. Cross-referencing with official corporate pages, government registries
3. Verification of HTTPS (port 443) availability on the primary domain

### meets_proposed_threshold Column

- **Yes** - Entity clearly meets or exceeds the thresholds proposed in the DPMC discussion documents
- **Borderline** - Entity is near the threshold boundary or its inclusion depends on final threshold definitions

### Limitations

- Hospital domains use legacy DHB subdomains which may redirect to tewhatuora.govt.nz;
  the scan follows redirects and assesses the final destination
- Some entities (CDC, Datacom) use .com domains as they are AU/NZ companies; their
  NZ-hosted infrastructure is still critical to NZ
- Defence domains (.mil.nz) may not respond to standard TLS scans due to security policies
- EDB ICP counts are approximate based on Commerce Commission disclosure data
- **CDN caveat**: Many endpoints sit behind CDNs (Cloudflare, AWS CloudFront, Akamai).
  PQC support detected at these endpoints may reflect the CDN's configuration, not the
  organisation's own cryptographic posture. The scanner detects and flags CDN-fronted endpoints.

## File Format

`critical-infrastructure.csv` columns:

| Column | Description |
|--------|-------------|
| entity_name | Organisation name |
| sector | One of the 7 DPMC sectors |
| sub_sector | More specific classification within the sector |
| domain | Primary public-facing domain (no protocol prefix) |
| scan_targets | Domain and port to scan (domain:443) |
| meets_proposed_threshold | Yes / Borderline |
| notes | Context about the entity, scale, ownership, and why it qualifies |

## Statistics

- **Total entities**: 118
- **Sectors covered**: 7/7
- Communications & Data: 18
- Defence: 6
- Energy: 30
- Finance: 14
- Health: 16
- Transport: 25
- Drinking Water & Wastewater: 9

## Last Updated

2026-03-26
