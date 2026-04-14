# Origin PQC Honour Roll — 14 April 2026

These 13 entities (of 118 scanned) run post-quantum TLS on infrastructure they operate themselves, with no content delivery network in front of the endpoint we tested. They represent **11%** of the scanned NZ critical infrastructure population.

In our view, these are the only organisations in the scan for which a PQC-positive result reflects an observable cryptographic decision, rather than a default flipped on by a CDN vendor. Whatever you did — keep doing it, and thank you.

## Defence / Intelligence

| Entity | Role |
|---|---|
| **GCSB** — Government Communications Security Bureau | SIGINT; houses the NCSC. |
| **NZSIS** — NZ Security Intelligence Service | Domestic security intelligence. |
| **DIA** — Department of Internal Affairs | Operates RealMe, the NZ federated digital identity service. |

The three agencies responsible for New Zealand's cryptographic policy are running the cryptography they publicly recommend.

## Energy

| Entity | Role |
|---|---|
| **Meridian Energy** | Electricity generation (largest NZ generator). |
| **Firstgas** | National gas transmission and distribution. |
| **Manawa Energy** | Hydro generation (formerly Trustpower generation). |
| **Pioneer Energy** | Hydro and wind generation. |
| **Marlborough Lines** | Electricity distribution (Marlborough region). |

## Finance

| Entity | Role |
|---|---|
| **ANZ New Zealand** | Registered bank; domestic systemically important. The only "big-4" bank on this list. |
| **SBS Bank** | Registered bank (Southland-headquartered). |
| **The Co-operative Bank** | Registered bank, customer-owned. |
| **Vero Insurance** | General insurer, Suncorp NZ subsidiary. |

## Communications & Data

| Entity | Role |
|---|---|
| **Tuatahi First Fibre** | Local Fibre Company serving the Waikato, Bay of Plenty, Taranaki and Whanganui regions. |

---

## How this list was produced

For each of the 118 scanned entities we recorded two independent signals:

1. Whether the server negotiated `X25519MLKEM768` when offered it (the "PQC-positive" signal).
2. Whether the endpoint was fronted by a CDN we can fingerprint (Cloudflare, AWS CloudFront, Akamai, Fastly, Imperva/Incapsula).

An entity makes this list only if **both** of the following are true:

- The PQC-positive signal is `true`.
- The CDN signal is `false` (no CDN detected on the public endpoint we tested).

This is a narrow test. We did not scan every subdomain, or the back-end systems, or anything beyond a single primary domain per entity. An entity that is on this list for one domain may still be classical elsewhere. An entity that is not on this list may have excellent cryptographic posture on other surfaces we did not observe.

## A note to the listed organisations

We are publishing this list to make positive examples visible. If your organisation is on it and you would rather not be, contact `simon [at] spinsphere.xyz` and we will remove you on request, no questions asked. We will not publish a corresponding "wall of shame" of entities with classical TLS.
