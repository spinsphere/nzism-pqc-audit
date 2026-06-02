# Origin PQC Honour Roll — 2 June 2026

These 25 entities (of 118 scanned) run post-quantum TLS on infrastructure they operate themselves, with no content delivery network detected in front of the endpoint we tested. They represent **21.7%** of the successfully scanned NZ critical infrastructure population — nearly double the 13 (11.2%) recorded in the [April 2026 scan](../2026-04-14/origin-honour-roll.md).

An entity is on this list only if the scan recorded `X25519MLKEM768` on a non-CDN-fronted endpoint. This is the same methodology as April. Entities marked **NEW** were not on the April list.

## Defence / Intelligence

| Entity | Role |
|---|---|
| **GCSB** — Government Communications Security Bureau | SIGINT; houses the NCSC. |
| **NZSIS** — NZ Security Intelligence Service | Domestic security intelligence. |
| **DIA** — Department of Internal Affairs | Operates RealMe, the NZ federated digital identity service. |

## Energy

| Entity | Role |
|---|---|
| **Meridian Energy** | Electricity generation (largest NZ generator). |
| **Firstgas** | National gas transmission and distribution. |
| **Manawa Energy** | Hydro generation (acquired by Contact Energy July 2025). |
| **Pioneer Energy** | Hydro and wind generation. |
| **Marlborough Lines** | Electricity distribution (Marlborough region). |

## Finance

| Entity | Role |
|---|---|
| **ANZ New Zealand** | Registered bank; domestic systemically important. |
| **SBS Bank** | Registered bank (Southland-headquartered). |
| **The Co-operative Bank** | Registered bank, customer-owned. |
| **Vero Insurance** | General insurer, Suncorp NZ subsidiary. |

## Communications & Data

| Entity | Role | Notes |
|---|---|---|
| **Tuatahi First Fibre** | Local Fibre Company serving Waikato, Bay of Plenty, Taranaki and Whanganui. | |
| **Southern Cross Cables** | International submarine cable network (SCCN + NEXT). | **NEW** — joined since April 2026. |

## Health

All eleven health entities below are **NEW** to the origin honour roll. In April 2026, every PQC-positive health endpoint appeared Imperva CDN-fronted. By June 2026, Imperva CDN is no longer detected in front of these domains. The underlying origin servers were already running `X25519MLKEM768` — what changed is the CDN layer, not the origin configuration.

| Entity | Role |
|---|---|
| **Health NZ / Te Whatu Ora** | National health system; operates all public hospitals. |
| **Waikato Hospital** | Central North Island tertiary hospital. |
| **Tauranga Hospital** | Bay of Plenty regional hospital. |
| **Wellington Regional Hospital** | Largest ICU in NZ; tertiary referral centre. |
| **Christchurch Hospital** | South Island tertiary hospital. |
| **Palmerston North Hospital** | MidCentral region secondary hospital. |
| **Hutt Hospital** | Lower/Upper Hutt secondary hospital. |
| **Nelson Hospital** | Nelson-Marlborough secondary hospital. |
| **Hawke's Bay Hospital** | Hawke's Bay regional hospital. |
| **Rotorua Hospital** | Rotorua/Lakes secondary hospital. |
| **Southland Hospital** | Southland/Invercargill secondary hospital. |

---

## How this list was produced

For each of the 118 scanned entities we recorded two independent signals:

1. Whether the server negotiated `X25519MLKEM768` when offered it (the "PQC-positive" signal).
2. Whether the endpoint was fronted by a CDN we can fingerprint (Cloudflare, AWS CloudFront, Akamai, Fastly, Imperva/Incapsula).

An entity makes this list only if **both** of the following are true:

- The PQC-positive signal is `true`.
- The CDN signal is `false` (no CDN detected on the public endpoint we tested).

This is a narrow test. We scanned one primary domain per entity. An entity not on this list may have excellent cryptographic posture on other surfaces we did not observe.

## A note to the listed organisations

We are publishing this list to make positive examples visible. If your organisation is on it and would rather not be, contact `simon [at] spinsphere.xyz` and we will remove you on request, no questions asked.
