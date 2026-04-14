# Contributing

Thanks for considering a contribution. `nzism-pqc-audit` is a small Rust project with a narrow scope — it's intentionally boring so that the cryptographic result it produces is easy to trust.

## How to get started

```bash
git clone https://github.com/spinsphere/nzism-pqc-audit.git
cd nzism-pqc-audit
cargo build
cargo test
```

A quick smoke test that doesn't hit any real infrastructure:

```bash
echo "cloudflare.com:443" > /tmp/smoke.txt
cargo run --release -- scan --input /tmp/smoke.txt --format text --output-json /tmp/smoke.json
```

Cloudflare has had `X25519MLKEM768` enabled by default since 2024, so this should return a PQC-positive result. If it doesn't, something has regressed.

## What makes a good contribution

**Small and self-explaining.** One change per PR. A clear commit message. A short description of what changed and why.

**Tests where they make sense.** The TLS handshake path is hard to unit-test without network access, but the CSV parser, the CDN fingerprint matcher, the NZISM mapping, and the report rendering code are all pure functions and should have tests around any new behaviour you add.

**No premature abstractions.** This project is ~500 lines of Rust. Don't add trait hierarchies, config files, or plugin systems speculatively. If you need them, the existing code will start to feel painful and you can introduce them then.

## Areas where contributions are especially welcome

- **New CDN fingerprints.** The current set covers Cloudflare, AWS CloudFront, Akamai, Fastly and Imperva/Incapsula. If you know a CDN that isn't detected, open an issue with an example target and/or a PR to `src/scanner/cdn.rs`.
- **New key exchange groups.** As the post-quantum ecosystem evolves (new hybrid variants, signature-based PQC key exchange, etc.), the `pqc_supported` detection logic in `src/scanner/tls.rs` will need updating.
- **Target lists for other jurisdictions.** Australia, UK, Canada, EU member states all have critical infrastructure frameworks. A well-sourced target list CSV + a README documenting the selection criteria is a very welcome contribution. Keep the shape consistent with `targets/nz/critical-infrastructure.csv`.
- **Report improvements.** The HTML output is self-contained and minimal. Sector breakdowns, charts, and accessibility improvements are all welcome.
- **Documentation.** If something in the README or SECURITY.md confused you, that's a bug — PR it.

## Things that will probably be declined

- **Alternative TLS backends.** The scanner uses `rustls` with the `aws-lc-rs` crypto provider because that combination currently gives us the most reliable post-quantum key share support in Rust. We will reconsider when the ecosystem moves, but please open an issue to discuss before opening a PR.
- **"Fix" PRs that rename things without changing behaviour.** Naming preferences vary; please don't.
- **PRs that add telemetry, phone-home, or analytics.** The tool must never send data anywhere except the targets you explicitly tell it to scan.

## Commit style

- One logical change per commit.
- Imperative mood in subject lines: "Add X", "Fix Y", not "Added" or "Adds".
- Wrap body text at ~72 characters.
- Sign-off is not required but always appreciated.

## Code of conduct

Be kind. Assume good faith. If a reviewer asks you to rework something, it's not personal. If a contributor asks you to clarify something in your review, it's because the answer wasn't obvious.

Issues that devolve into personal attacks will be locked.

## Licence of your contributions

By submitting a pull request, you agree to licence your contribution under the same [MIT licence](LICENSE) that covers the rest of this project.
